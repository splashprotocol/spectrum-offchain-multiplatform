use std::cmp::{max, min};
use std::mem;
use std::ops::{Sub, SubAssign};

use algebra_core::monoid::Monoid;
use either::Either;
use log::trace;
use num_rational::Ratio;

use spectrum_offchain::data::{Has, Stable};
use spectrum_offchain::maker::Maker;

use crate::execution_engine::liquidity_book::fragment::{Fragment, OrderState, StateTrans};
use crate::execution_engine::liquidity_book::pool::Pool;
use crate::execution_engine::liquidity_book::recipe::{
    ExecutionRecipe, Fill, IntermediateRecipe, PartialFill, Swap, TerminalInstruction,
};
use crate::execution_engine::liquidity_book::side::Side::{Ask, Bid};
use crate::execution_engine::liquidity_book::side::{Side, SideM};
use crate::execution_engine::liquidity_book::state::{IdleState, TLBState, VersionedState};
use crate::execution_engine::liquidity_book::types::AbsolutePrice;
use crate::execution_engine::types::Time;

pub mod fragment;
pub mod interpreter;
pub mod pool;
pub mod recipe;
pub mod side;
mod state;
pub mod time;
pub mod types;
pub mod weight;

/// TLB is a Universal Liquidity Aggregator (ULA), it is able to aggregate every piece of composable
/// liquidity available in the market.
///
/// Composable liquidity falls into two essential categories:
/// (1.) Discrete Fragments of liquidity;
/// (2.) Pooled (according to some AMM formula) liquidity;
pub trait TemporalLiquidityBook<Fr, Pl> {
    fn attempt(&mut self) -> Option<ExecutionRecipe<Fr, Pl>>;
}

/// TLB API for external events affecting its state.
pub trait ExternalTLBEvents<Fr, Pl> {
    fn advance_clocks(&mut self, new_time: u64);
    fn add_fragment(&mut self, fr: Fr);
    fn remove_fragment(&mut self, fr: Fr);
    fn update_pool(&mut self, pool: Pl);
    fn remove_pool(&mut self, pool: Pl);
}

/// TLB API for feedback events affecting its state.
pub trait TLBFeedback<Fr, Pl> {
    fn on_recipe_succeeded(&mut self);
    fn on_recipe_failed(&mut self);
}

#[derive(Debug, Copy, Clone)]
pub struct ExecutionCap<U> {
    pub soft: U,
    pub hard: U,
}

impl<U: Sub<Output = U> + Copy> ExecutionCap<U> {
    fn safe_threshold(&self) -> U {
        self.hard - self.soft
    }
}

#[derive(Debug, Clone)]
pub struct TLB<Fr, Pl: Stable, U> {
    state: TLBState<Fr, Pl>,
    execution_cap: ExecutionCap<U>,
}

impl<Fr, Pl, Ctx, U> Maker<Ctx> for TLB<Fr, Pl, U>
where
    Pl: Stable,
    Ctx: Has<Time> + Has<ExecutionCap<U>>,
{
    fn make(ctx: &Ctx) -> Self {
        Self::new(ctx.select::<Time>().into(), ctx.select::<ExecutionCap<U>>())
    }
}

impl<Fr, Pl: Stable, U> TLB<Fr, Pl, U> {
    pub fn new(time: u64, conf: ExecutionCap<U>) -> Self {
        Self {
            state: TLBState::new(time),
            execution_cap: conf,
        }
    }
}

impl<Fr, Pl, U> TLB<Fr, Pl, U>
where
    Fr: Fragment<U = U> + OrderState + Ord + Copy,
    Pl: Pool + Stable + Copy,
    U: PartialOrd,
{
    fn on_transition(&mut self, tx: StateTrans<Fr>) {
        if let StateTrans::Active(fr) = tx {
            self.state.pre_add_fragment(fr);
        }
    }
}

impl<Fr, Pl, U> TemporalLiquidityBook<Fr, Pl> for TLB<Fr, Pl, U>
where
    Fr: Fragment<U = U> + OrderState + Copy + Ord + std::fmt::Debug,
    Pl: Pool + Stable + Copy + std::fmt::Debug,
    U: Monoid + PartialOrd + SubAssign + Sub<Output = U> + Copy,
{
    fn attempt(&mut self) -> Option<ExecutionRecipe<Fr, Pl>> {
        if let Some(best_fr) = self.state.pick_best_fr_either() {
            let mut recipe = IntermediateRecipe::new(best_fr);
            trace!(target: "tlb", "TLB::attempt: recipe {:?}", recipe);
            let mut execution_units_left = self.execution_cap.hard;
            loop {
                if let Some(rem) = &recipe.remainder {
                    let price_fragments = self.state.best_fr_price(!rem.target.side());
                    let price_in_pools = self.state.best_pool_price();
                    match (price_in_pools, price_fragments) {
                        (price_in_pools, Some(price_in_fragments))
                            if price_in_pools
                                .map(|p| price_in_fragments.better_than(p))
                                .unwrap_or(true)
                                && execution_units_left > self.execution_cap.safe_threshold() =>
                        {
                            let rem_side = rem.target.side();
                            if let Some(opposite_fr) = self.state.try_pick_fr(!rem_side, |fr| {
                                rem_side.wrap(rem.target.price()).overlaps(fr.price())
                                    && fr.marginal_cost_hint() <= execution_units_left
                            }) {
                                execution_units_left -= opposite_fr.marginal_cost_hint();
                                let make_match = |x: &Fr, y: &Fr| {
                                    let (ask, bid) = match x.side() {
                                        SideM::Bid => (y, x),
                                        SideM::Ask => (x, y),
                                    };
                                    settle_price(ask, bid, price_in_pools)
                                };
                                match fill_from_fragment(*rem, opposite_fr, make_match) {
                                    FillFromFragment {
                                        term_fill_lt,
                                        fill_rt: Either::Left(term_fill_rt),
                                    } => {
                                        recipe.push(TerminalInstruction::Fill(term_fill_lt));
                                        recipe.terminate(TerminalInstruction::Fill(term_fill_rt));
                                        self.on_transition(term_fill_lt.next_fr);
                                        self.on_transition(term_fill_rt.next_fr);
                                    }
                                    FillFromFragment {
                                        term_fill_lt,
                                        fill_rt: Either::Right(partial),
                                    } => {
                                        recipe.push(TerminalInstruction::Fill(term_fill_lt));
                                        recipe.set_remainder(partial);
                                        self.on_transition(term_fill_lt.next_fr);
                                        continue;
                                    }
                                }
                            }
                        }
                        (Some(_), _) if execution_units_left > U::empty() => {
                            let rem_side = rem.target.side();
                            if let Some(pool) = self.state.try_pick_pool(|pl| {
                                let real_price = pl.real_price(rem_side.wrap(rem.remaining_input));
                                trace!(target: "tlb", "TLD::attempt(): side: {}, real_price: {}, remaining_input: {}", rem_side, real_price, rem.remaining_input);
                                rem_side
                                    .wrap(rem.target.price())
                                    .overlaps(real_price)
                            }) {
                                let FillFromPool { term_fill, swap } = fill_from_pool(*rem, pool);
                                recipe.push(TerminalInstruction::Swap(swap));
                                recipe.terminate(TerminalInstruction::Fill(term_fill));
                                self.on_transition(term_fill.next_fr);
                                self.state.pre_add_pool(swap.transition);
                            }
                        }
                        _ => {
                            trace!(target: "tlb", "TLD::attempt(): No-OP");
                        }
                    }
                }
                break;
            }
            if let Some(ex_recipe) = ExecutionRecipe::try_from(recipe) {
                return Some(ex_recipe);
            }
            self.on_recipe_failed();
        }
        None
    }
}

fn requiring_settled_state<Fr, Pl, U, F>(book: &mut TLB<Fr, Pl, U>, f: F)
where
    Pl: Stable,
    F: Fn(&mut IdleState<Fr, Pl>),
{
    match book.state {
        TLBState::Idle(ref mut st) => f(st),
        // If there is an attempt to apply external mutations to TLB in a Preview state
        // this is a developer's error, so we fail explicitly.
        TLBState::PartialPreview(_) | TLBState::Preview(_) => {
            panic!("PartialPreview|Preview state cannot be externally mutated")
        }
    }
}

impl<Fr, Pl, U> ExternalTLBEvents<Fr, Pl> for TLB<Fr, Pl, U>
where
    Fr: Fragment + OrderState + Ord + Copy,
    Pl: Pool + Stable + Copy,
{
    fn advance_clocks(&mut self, new_time: u64) {
        requiring_settled_state(self, |st| st.advance_clocks(new_time))
    }

    fn add_fragment(&mut self, fr: Fr) {
        trace!(target: "tlb", "TLB::add_fragment()");
        requiring_settled_state(self, |st| st.add_fragment(fr))
    }

    fn remove_fragment(&mut self, fr: Fr) {
        requiring_settled_state(self, |st| st.remove_fragment(fr))
    }

    fn update_pool(&mut self, pool: Pl) {
        requiring_settled_state(self, |st| st.update_pool(pool))
    }

    fn remove_pool(&mut self, pool: Pl) {
        requiring_settled_state(self, |st| st.remove_pool(pool))
    }
}

impl<Fr, Pl, U> TLBFeedback<Fr, Pl> for TLB<Fr, Pl, U>
where
    Fr: Fragment + OrderState + Ord + Copy,
    Pl: Pool + Stable + Copy,
{
    fn on_recipe_succeeded(&mut self) {
        match &mut self.state {
            TLBState::Idle(_) => {}
            TLBState::PartialPreview(st) => {
                trace!(target: "tlb", "TLBState::PartialPreview: recipe succeeded");
                let new_st = st.commit();
                mem::swap(&mut self.state, &mut TLBState::Idle(new_st));
            }
            TLBState::Preview(st) => {
                trace!(target: "tlb", "TLBState::Preview: recipe succeeded");
                let new_st = st.commit();
                mem::swap(&mut self.state, &mut TLBState::Idle(new_st));
            }
        }
    }

    fn on_recipe_failed(&mut self) {
        match &mut self.state {
            TLBState::Idle(_) => {}
            TLBState::PartialPreview(st) => {
                trace!(target: "tlb", "TLBState::PartialPreview: recipe failed");
                let new_st = st.rollback();
                mem::swap(&mut self.state, &mut TLBState::Idle(new_st));
            }
            TLBState::Preview(st) => {
                trace!(target: "tlb", "TLBState::Preview: recipe failed");
                let new_st = st.rollback();
                mem::swap(&mut self.state, &mut TLBState::Idle(new_st));
            }
        }
    }
}

const MAX_BIAS_PERCENT: u128 = 3;

//                 P_settled
//                     |
// p: >.... P_x ......(.)...... P_i .... P_y.... >
//           |         |         |        |
//          ask      bias<=3%..pivot     bid
/// Settle execution price for two interleaving fragments.
fn settle_price<Fr: Fragment>(ask: &Fr, bid: &Fr, index_price: Option<AbsolutePrice>) -> AbsolutePrice {
    let price_ask = ask.price();
    let price_bid = bid.price();
    let price_ask_rat = price_ask.unwrap();
    let price_bid_rat = price_bid.unwrap();
    let d = price_bid_rat - price_ask_rat;
    let pivotal_price = if let Some(index_price) = index_price {
        truncated(index_price.unwrap(), price_ask_rat, price_bid_rat)
    } else {
        price_ask_rat + d / 2
    };
    let fee_ask = ask.fee() as i128;
    let fee_bid = bid.fee() as i128;
    let bias_percent = if fee_ask < fee_bid {
        -fee_ask * 100 / fee_bid
    } else {
        fee_bid * 100 / fee_ask
    };
    let max_deviation = pivotal_price * Ratio::new(MAX_BIAS_PERCENT, 100);
    let deviation = to_signed(max_deviation) * Ratio::new(bias_percent, 100);
    let corrected_price = to_unsigned(to_signed(pivotal_price) + deviation);
    AbsolutePrice::from(truncated(corrected_price, price_ask_rat, price_bid_rat))
}

fn truncated<I: PartialOrd>(value: I, low: I, high: I) -> I {
    if value >= low && value <= high {
        value
    } else if value < low {
        low
    } else {
        high
    }
}

fn to_signed(r: Ratio<u128>) -> Ratio<i128> {
    Ratio::new(*r.numer() as i128, *r.denom() as i128)
}

fn to_unsigned(r: Ratio<i128>) -> Ratio<u128> {
    Ratio::new(*r.numer() as u128, *r.denom() as u128)
}

struct FillFromFragment<Fr> {
    /// Terminal [Fill].
    term_fill_lt: Fill<Fr>,
    /// Either terminal [Fill] or [PartialFill].
    fill_rt: Either<Fill<Fr>, PartialFill<Fr>>,
}

fn fill_from_fragment<Fr, U, F>(lhs: PartialFill<Fr>, rhs: Fr, matchmaker: F) -> FillFromFragment<Fr>
where
    Fr: Fragment<U = U> + OrderState + Copy,
    U: PartialOrd,
    F: FnOnce(&Fr, &Fr) -> AbsolutePrice,
{
    match lhs.target.side() {
        SideM::Bid => {
            let mut bid = lhs;
            let ask = rhs;
            let price = matchmaker(&ask, &bid.target);
            let demand_base = linear_output(bid.remaining_input, Bid(price));
            let supply_base = ask.input();
            if supply_base > demand_base {
                let quote_input = bid.remaining_input;
                bid.accumulated_output += demand_base;
                let remaining_input = supply_base - demand_base;
                FillFromFragment {
                    term_fill_lt: bid.filled_unsafe(),
                    fill_rt: Either::Right(PartialFill::new(ask, remaining_input, quote_input)),
                }
            } else if supply_base < demand_base {
                let quote_executed = linear_output(supply_base, Ask(price));
                bid.remaining_input -= quote_executed;
                bid.accumulated_output += supply_base;
                let (next_ask, ask_budget_used, fee_used) =
                    ask.with_applied_swap(ask.input(), quote_executed);
                FillFromFragment {
                    term_fill_lt: Fill::new(ask, next_ask, quote_executed, ask_budget_used, fee_used),
                    fill_rt: Either::Right(bid),
                }
            } else {
                let quote_executed = linear_output(supply_base, Ask(price));
                bid.accumulated_output += demand_base;
                let (next_ask, ask_budget_used, fee_used) =
                    ask.with_applied_swap(ask.input(), quote_executed);
                FillFromFragment {
                    term_fill_lt: bid.filled_unsafe(),
                    fill_rt: Either::Left(Fill::new(
                        ask,
                        next_ask,
                        quote_executed,
                        ask_budget_used,
                        fee_used,
                    )),
                }
            }
        }
        SideM::Ask => {
            let mut ask = lhs;
            let bid = rhs;
            let price = matchmaker(&bid, &ask.target);
            let demand_base = linear_output(bid.input(), Bid(price));
            let supply_base = ask.remaining_input;
            println!("supply_base: {}, demand_base: {}", supply_base, demand_base);
            if supply_base > demand_base {
                println!("Ask sold: {} at price {}", demand_base, price);
                println!("Ask received: {}", bid.input());
                ask.remaining_input -= demand_base;
                ask.accumulated_output += bid.input();
                let (next_bid, bid_budget_used, fee_used) = bid.with_applied_swap(bid.input(), demand_base);
                FillFromFragment {
                    term_fill_lt: Fill::new(bid, next_bid, demand_base, bid_budget_used, fee_used),
                    fill_rt: Either::Right(ask),
                }
            } else if supply_base < demand_base {
                let quote_executed = linear_output(supply_base, Ask(price));
                ask.accumulated_output += quote_executed;
                FillFromFragment {
                    term_fill_lt: ask.filled_unsafe(),
                    fill_rt: Either::Right(PartialFill::new(bid, bid.input() - quote_executed, supply_base)),
                }
            } else {
                ask.accumulated_output += bid.input();
                let (next_bid, bid_budget_used, fee_used) = bid.with_applied_swap(bid.input(), demand_base);
                FillFromFragment {
                    term_fill_lt: ask.filled_unsafe(),
                    fill_rt: Either::Left(Fill::new(bid, next_bid, demand_base, bid_budget_used, fee_used)),
                }
            }
        }
    }
}

fn linear_output(input: u64, price: Side<AbsolutePrice>) -> u64 {
    match price {
        Bid(price) => (input as u128 * price.denom() / price.numer()) as u64,
        Ask(price) => (input as u128 * price.numer() / price.denom()) as u64,
    }
}

struct FillFromPool<Fr, Pl> {
    term_fill: Fill<Fr>,
    swap: Swap<Pl>,
}

fn fill_from_pool<Fr, Pl>(lhs: PartialFill<Fr>, pool: Pl) -> FillFromPool<Fr, Pl>
where
    Fr: Fragment + OrderState + Copy,
    Pl: Pool + Copy,
{
    match lhs.target.side() {
        SideM::Bid => {
            trace!(target: "tlb", "fill_from_pool: BID");
            let mut bid = lhs;
            let quote_input = bid.remaining_input;
            let (execution_amount, next_pool) = pool.swap(Side::Bid(quote_input));
            bid.accumulated_output += execution_amount;
            let swap = Swap {
                target: pool,
                transition: next_pool,
                side: SideM::Bid,
                input: quote_input,
                output: execution_amount,
            };
            FillFromPool {
                term_fill: bid.filled_unsafe(),
                swap,
            }
        }
        SideM::Ask => {
            trace!(target: "tlb", "fill_from_pool: ASK");
            let mut ask = lhs;
            let base_input = ask.remaining_input;
            let (execution_amount, next_pool) = pool.swap(Side::Ask(base_input));
            ask.accumulated_output += execution_amount;
            let swap = Swap {
                target: pool,
                transition: next_pool,
                side: SideM::Ask,
                input: base_input,
                output: execution_amount,
            };
            FillFromPool {
                term_fill: ask.filled_unsafe(),
                swap,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use either::Either;

    use crate::execution_engine::liquidity_book::fragment::StateTrans;
    use crate::execution_engine::liquidity_book::pool::Pool;
    use crate::execution_engine::liquidity_book::recipe::{
        ExecutionRecipe, Fill, IntermediateRecipe, PartialFill, Swap, TerminalInstruction,
    };
    use crate::execution_engine::liquidity_book::side::{Side, SideM};
    use crate::execution_engine::liquidity_book::state::tests::{SimpleCFMMPool, SimpleOrderPF};
    use crate::execution_engine::liquidity_book::time::TimeBounds;
    use crate::execution_engine::liquidity_book::types::AbsolutePrice;
    use crate::execution_engine::liquidity_book::{
        fill_from_fragment, fill_from_pool, settle_price, ExecutionCap, ExternalTLBEvents, FillFromFragment,
        FillFromPool, TemporalLiquidityBook, TLB,
    };
    use crate::execution_engine::types::StableId;

    #[test]
    fn recipe_fill_fragment_from_fragment() {
        // Assuming pair ADA/USDT @ 0.37
        let o1 = SimpleOrderPF::new(SideM::Ask, 2000, AbsolutePrice::new(36, 100), 1000);
        let o2 = SimpleOrderPF::new(SideM::Bid, 370, AbsolutePrice::new(37, 100), 990);
        let p1 = SimpleCFMMPool {
            pool_id: StableId::random(),
            reserves_base: 1000000000000000,
            reserves_quote: 370000000000000,
            fee_num: 997,
        };
        let mut book = TLB::new(
            0,
            ExecutionCap {
                soft: 10000,
                hard: 16000,
            },
        );
        book.add_fragment(o1);
        book.add_fragment(o2);
        book.update_pool(p1);
        let recipe = book.attempt();
        let p2 = recipe
            .clone()
            .unwrap()
            .instructions()
            .iter()
            .find_map(|i| match i {
                TerminalInstruction::Fill(_) => None,
                TerminalInstruction::Swap(swap) => Some(swap.transition),
            })
            .unwrap();
        let expected_recipe = IntermediateRecipe {
            terminal: vec![
                TerminalInstruction::Fill(Fill {
                    target_fr: o2,
                    next_fr: StateTrans::EOL,
                    removed_input: o2.input,
                    added_output: 1000,
                    budget_used: 990000,
                    fee_used: 990,
                }),
                TerminalInstruction::Swap(Swap {
                    target: p1,
                    transition: p2,
                    side: SideM::Ask,
                    input: 1000,
                    output: 368,
                }),
                TerminalInstruction::Fill(Fill {
                    target_fr: o1,
                    next_fr: StateTrans::EOL,
                    removed_input: o1.input,
                    added_output: 738,
                    budget_used: 738000,
                    fee_used: 1000,
                }),
            ],
            remainder: None,
        };
        assert_eq!(recipe, ExecutionRecipe::try_from(expected_recipe));
    }

    #[test]
    fn fill_fragment_from_fragment() {
        // Assuming pair ADA/USDT @ 0.37
        let fr1 = SimpleOrderPF {
            source: StableId::random(),
            side: SideM::Ask,
            input: 1000,
            accumulated_output: 0,
            price: AbsolutePrice::new(37, 100),
            fee: 1000,
            ex_budget: 0,
            cost_hint: 100,
            bounds: TimeBounds::None,
        };
        let fr2 = SimpleOrderPF {
            source: StableId::random(),
            side: SideM::Bid,
            input: 370,
            accumulated_output: 0,
            price: AbsolutePrice::new(37, 100),
            fee: 1000,
            ex_budget: 0,
            cost_hint: 100,
            bounds: TimeBounds::None,
        };
        let make_match =
            |x: &SimpleOrderPF, y: &SimpleOrderPF| settle_price(x, y, Some(AbsolutePrice::new(37, 100)));
        let FillFromFragment {
            term_fill_lt,
            fill_rt: term_fill_rt,
        } = fill_from_fragment(PartialFill::empty(fr1), fr2, make_match);
        assert_eq!(term_fill_lt.added_output, fr2.input);
        match term_fill_rt {
            Either::Left(fill_rt) => assert_eq!(fill_rt.added_output, fr1.input),
            Either::Right(_) => panic!(),
        }
    }

    #[test]
    fn fill_fragment_from_fragment_partial() {
        // Assuming pair ADA/USDT @ 0.37
        let p = AbsolutePrice::new(37, 100);
        let fr1 = SimpleOrderPF {
            source: StableId::random(),
            side: SideM::Ask,
            input: 1000,
            accumulated_output: 0,
            price: p,
            fee: 2000,
            ex_budget: 0,
            cost_hint: 100,
            bounds: TimeBounds::None,
        };
        let fr2 = SimpleOrderPF {
            source: StableId::random(),
            side: SideM::Bid,
            input: 210,
            accumulated_output: 0,
            price: p,
            fee: 2000,
            ex_budget: 0,
            cost_hint: 100,
            bounds: TimeBounds::None,
        };
        let make_match = |x: &SimpleOrderPF, y: &SimpleOrderPF| settle_price(x, y, Some(p));
        let FillFromFragment {
            term_fill_lt,
            fill_rt: term_fill_rt,
        } = fill_from_fragment(PartialFill::empty(fr1), fr2, make_match);
        assert_eq!(
            term_fill_lt.added_output,
            ((fr2.input as u128) * fr1.price.denom() / fr1.price.numer()) as u64
        );
        match term_fill_rt {
            Either::Right(fill_rt) => assert_eq!(fill_rt.accumulated_output, fr2.input),
            Either::Left(_) => panic!(),
        }
    }

    #[test]
    fn prefer_fragment_with_better_fee() {
        // Assuming pair ADA/USDT @ ask price 0.37, bid price 0.36
        let ask_fr = SimpleOrderPF {
            source: StableId::random(),
            side: SideM::Ask,
            input: 1000,
            accumulated_output: 0,
            price: AbsolutePrice::new(37, 100),
            fee: 1000,
            ex_budget: 0,
            cost_hint: 100,
            bounds: TimeBounds::None,
        };
        let bid_fr = SimpleOrderPF {
            source: StableId::random(),
            side: SideM::Bid,
            input: 360,
            accumulated_output: 0,
            price: AbsolutePrice::new(36, 100),
            fee: 2000,
            ex_budget: 0,
            cost_hint: 100,
            bounds: TimeBounds::None,
        };
        let make_match =
            |x: &SimpleOrderPF, y: &SimpleOrderPF| settle_price(x, y, Some(AbsolutePrice::new(37, 100)));
        let FillFromFragment {
            term_fill_lt,
            fill_rt: term_fill_rt,
        } = fill_from_fragment(PartialFill::empty(ask_fr), bid_fr, make_match);
        match term_fill_rt {
            Either::Left(_) => panic!(),
            Either::Right(part_fill_rt) => assert_eq!(part_fill_rt.accumulated_output, bid_fr.input),
        }
    }

    #[test]
    fn fill_reminder_from_pool() {
        // Assuming pair ADA/USDT @ ask price 0.360, real price in pool 0.364.
        let ask_fr = SimpleOrderPF {
            source: StableId::random(),
            side: SideM::Ask,
            input: 1000,
            accumulated_output: 0,
            price: AbsolutePrice::new(36, 100),
            fee: 1000,
            ex_budget: 0,
            cost_hint: 100,
            bounds: TimeBounds::None,
        };
        let pf = PartialFill {
            target: ask_fr,
            remaining_input: 500,
            accumulated_output: 180,
        };
        let pool = SimpleCFMMPool {
            pool_id: StableId::random(),
            reserves_base: 100000000000000,
            reserves_quote: 36600000000000,
            fee_num: 997,
        };
        let real_price_in_pool = pool.real_price(Side::Ask(pf.remaining_input));
        let FillFromPool { term_fill, swap } = fill_from_pool(pf, pool);
        assert_eq!(swap.input, pf.remaining_input);
        assert_eq!(
            (term_fill.added_output - pf.accumulated_output) as u128,
            pf.remaining_input as u128 * real_price_in_pool.numer() / real_price_in_pool.denom()
        );
    }

    #[test]
    fn match_price_biased_towards_best_fee() {
        let ask_price = AbsolutePrice::new(30, 100);
        let bid_price = AbsolutePrice::new(50, 100);
        let index_price = AbsolutePrice::new(40, 100);
        let ask_fr = SimpleOrderPF {
            source: StableId::random(),
            side: SideM::Ask,
            input: 1000,
            accumulated_output: 0,
            price: ask_price,
            fee: 4000,
            ex_budget: 0,
            cost_hint: 100,
            bounds: TimeBounds::None,
        };
        let bid_fr = SimpleOrderPF {
            source: StableId::random(),
            side: SideM::Bid,
            input: 360,
            accumulated_output: 0,
            price: bid_price,
            fee: 2000,
            ex_budget: 0,
            cost_hint: 100,
            bounds: TimeBounds::None,
        };
        let make_match = |x: &SimpleOrderPF, y: &SimpleOrderPF| settle_price(x, y, Some(index_price));
        let final_price = make_match(&ask_fr, &bid_fr);
        assert!(final_price.unwrap() - ask_price.unwrap() > bid_price.unwrap() - final_price.unwrap());
    }

    #[test]
    fn match_price_biased_towards_best_fee_() {
        let ask_price = AbsolutePrice::new(30, 100);
        let bid_price = AbsolutePrice::new(50, 100);
        let index_price = AbsolutePrice::new(51, 100);
        let ask_fr = SimpleOrderPF {
            source: StableId::random(),
            side: SideM::Ask,
            input: 1000,
            accumulated_output: 0,
            price: ask_price,
            fee: 4000,
            ex_budget: 0,
            cost_hint: 100,
            bounds: TimeBounds::None,
        };
        let bid_fr = SimpleOrderPF {
            source: StableId::random(),
            side: SideM::Bid,
            input: 360,
            accumulated_output: 0,
            price: bid_price,
            fee: 2000,
            ex_budget: 0,
            cost_hint: 100,
            bounds: TimeBounds::None,
        };
        let make_match = |x: &SimpleOrderPF, y: &SimpleOrderPF| settle_price(x, y, Some(index_price));
        let final_price = make_match(&ask_fr, &bid_fr);
        assert!(final_price.unwrap() - ask_price.unwrap() > bid_price.unwrap() - final_price.unwrap());
    }

    #[test]
    fn match_price_always_stays_within_bounds() {
        let ask_price = AbsolutePrice::new(37, 100);
        let bid_price = AbsolutePrice::new(37, 100);
        let index_price = AbsolutePrice::new(40, 100);
        let ask_fr = SimpleOrderPF {
            source: StableId::random(),
            side: SideM::Ask,
            input: 1000,
            accumulated_output: 0,
            price: ask_price,
            fee: 4000,
            ex_budget: 0,
            cost_hint: 100,
            bounds: TimeBounds::None,
        };
        let bid_fr = SimpleOrderPF {
            source: StableId::random(),
            side: SideM::Bid,
            input: 360,
            accumulated_output: 0,
            price: bid_price,
            fee: 2000,
            ex_budget: 0,
            cost_hint: 100,
            bounds: TimeBounds::None,
        };
        let make_match = |x: &SimpleOrderPF, y: &SimpleOrderPF| settle_price(x, y, Some(index_price));
        let final_price = make_match(&ask_fr, &bid_fr);
        assert_eq!(final_price, bid_price)
    }
}
