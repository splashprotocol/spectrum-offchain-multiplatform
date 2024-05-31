use std::collections::hash_map::Entry;
use std::collections::{btree_map, BTreeMap, BTreeSet, HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use std::mem;
use std::ops::Add;

use log::trace;

use spectrum_offchain::data::Stable;

use crate::execution_engine::liquidity_book::fragment::{Fragment, OrderState, StateTrans};
use crate::execution_engine::liquidity_book::pool::{Pool, PoolQuality};
use crate::execution_engine::liquidity_book::side::{Side, SideM};
use crate::execution_engine::liquidity_book::stashing_option::StashingOption;
use crate::execution_engine::liquidity_book::types::AbsolutePrice;
use crate::execution_engine::liquidity_book::weight::Weighted;

pub(crate) trait VersionedState<Fr, Pl: Stable> {
    /// Commit preview changes.
    fn commit(&mut self) -> IdleState<Fr, Pl>;
    /// Discard preview changes.
    fn rollback(&mut self, stashing_opt: StashingOption<Fr>) -> IdleState<Fr, Pl>;
}

#[derive(Debug, Clone, Eq, PartialEq)]
/// State with no uncommitted changes.
pub struct IdleState<Fr, Pl: Stable> {
    fragments: Chronology<Fr>,
    pools: Pools<Pl>,
}

impl<Fr, Pl: Stable> IdleState<Fr, Pl> {
    fn new(time_now: u64) -> Self {
        Self {
            fragments: Chronology::new(time_now),
            pools: Pools::new(),
        }
    }
}

impl<Fr, Pl> IdleState<Fr, Pl>
where
    Fr: Fragment + OrderState + Ord + Copy,
    Pl: Pool + Stable + Copy,
{
    pub fn advance_clocks(&mut self, new_time: u64) {
        self.fragments.advance_clocks(new_time)
    }

    pub fn add_fragment(&mut self, fr: Fr) {
        self.fragments.add_fragment(fr);
    }

    pub fn remove_fragment(&mut self, fr: Fr) {
        trace!("Removing fragment");
        self.fragments.remove_fragment(fr);
    }

    pub fn update_pool(&mut self, pool: Pl) {
        self.pools.update_pool(pool);
    }

    pub fn remove_pool(&mut self, pool: Pl) {
        self.pools.remove_pool(pool);
    }
}

/// Changed state that reflects only consumption of fragments and full preview of pools.
/// We use this one when no preview fragments/pools are generated to avoid
/// overhead of copying active frontier projection.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PartialPreviewState<Fr, Pl: Stable> {
    fragments_preview: Chronology<Fr>,
    consumed_active_fragments: Vec<Fr>,
    stashed_active_fragments: Vec<Fr>,
    pools_intact: Pools<Pl>,
    pools_preview: Pools<Pl>,
}

impl<Fr, Pl: Stable> PartialPreviewState<Fr, Pl> {
    pub fn new(time_now: u64) -> Self {
        Self {
            fragments_preview: Chronology::new(time_now),
            consumed_active_fragments: vec![],
            stashed_active_fragments: vec![],
            pools_intact: Pools::new(),
            pools_preview: Pools::new(),
        }
    }
}

impl<Fr, Pl: Stable> VersionedState<Fr, Pl> for PartialPreviewState<Fr, Pl>
where
    Fr: Fragment + Ord + Hash,
{
    fn commit(&mut self) -> IdleState<Fr, Pl> {
        trace!(target: "state", "PartialPreviewState::commit");
        let mut fresh_settled_st = IdleState::new(0);
        mem::swap(&mut fresh_settled_st.fragments, &mut self.fragments_preview);
        mem::swap(&mut fresh_settled_st.pools, &mut self.pools_preview);
        fresh_settled_st
    }

    fn rollback(&mut self, stashing_opt: StashingOption<Fr>) -> IdleState<Fr, Pl> {
        trace!(target: "state", "PartialPreviewState::rollback");
        // Return consumed fragments to reconstruct initial state.
        let mut stashed_this_time = HashSet::new();
        match stashing_opt {
            StashingOption::Stash(mut to_stash) => {
                self.stashed_active_fragments.append(&mut to_stash);
                for fr in to_stash {
                    stashed_this_time.insert(fr);
                }
            }
            StashingOption::Unstash => {
                let stashed_fragments = mem::take(&mut self.stashed_active_fragments);
                for fr in stashed_fragments {
                    self.fragments_preview.active.insert(fr);
                }
            }
        }
        while let Some(fr) = self.consumed_active_fragments.pop() {
            if stashed_this_time.contains(&fr) {
                self.stashed_active_fragments.push(fr);
            } else {
                self.fragments_preview.active.insert(fr);
            }
        }
        let mut fresh_idle_st = IdleState::new(0);
        // Move reconstructed initial fragments into idle state.
        mem::swap(&mut self.fragments_preview, &mut fresh_idle_st.fragments);
        mem::swap(&mut self.pools_intact, &mut fresh_idle_st.pools);
        fresh_idle_st
    }
}

/// State with areas of uncommitted changes.
/// This state offers consistent projections of active frontier for both
/// consumption and production of new fragments/pools.
/// Comes with overhead of cloning active frontier/pools upon construction.
#[derive(Debug, Clone)]
pub struct PreviewState<Fr, Pl: Stable> {
    /// Fragments before changes.
    fragments_intact: Chronology<Fr>,
    /// Active fragments with changes pre-applied.
    active_fragments_preview: Fragments<Fr>,
    stashed_active_fragments: Vec<Fr>,
    /// Set of new inactive fragments.
    inactive_fragments_changeset: Vec<(u64, Fr)>,
    /// Pools before changes.
    pools_intact: Pools<Pl>,
    /// Active pools with changes pre-applied.
    pools_preview: Pools<Pl>,
}

impl<Fr, Pl: Stable> PreviewState<Fr, Pl> {
    fn new(time_now: u64) -> Self {
        Self {
            fragments_intact: Chronology::new(time_now),
            active_fragments_preview: Fragments::new(),
            inactive_fragments_changeset: vec![],
            stashed_active_fragments: vec![],
            pools_intact: Pools::new(),
            pools_preview: Pools::new(),
        }
    }
}

impl<Fr, Pl> VersionedState<Fr, Pl> for PreviewState<Fr, Pl>
where
    Fr: Fragment + Ord + Hash,
    Pl: Stable,
{
    fn commit(&mut self) -> IdleState<Fr, Pl> {
        trace!(target: "state", "PreviewState::commit");
        // Commit pools preview if available.
        mem::swap(&mut self.pools_intact, &mut self.pools_preview);
        // Commit active fragments preview if available.
        mem::swap(
            &mut self.fragments_intact.active,
            &mut self.active_fragments_preview,
        );
        // Commit inactive fragments.
        while let Some((t, fr)) = self.inactive_fragments_changeset.pop() {
            match self.fragments_intact.inactive.entry(t) {
                btree_map::Entry::Vacant(entry) => {
                    let mut frs = Fragments::new();
                    frs.insert(fr);
                    entry.insert(frs);
                }
                btree_map::Entry::Occupied(mut entry) => {
                    entry.get_mut().insert(fr);
                }
            }
        }
        let mut fresh_settled_st = IdleState::new(self.fragments_intact.time_now);
        mem::swap(&mut fresh_settled_st.fragments, &mut self.fragments_intact);
        mem::swap(&mut fresh_settled_st.pools, &mut self.pools_intact);
        fresh_settled_st
    }

    fn rollback(&mut self, stashing_opt: StashingOption<Fr>) -> IdleState<Fr, Pl> {
        trace!(target: "state", "PreviewState::rollback");
        match stashing_opt {
            StashingOption::Stash(mut to_stash) => {
                self.stashed_active_fragments.append(&mut to_stash);
                for fr in to_stash {
                    self.fragments_intact.active.remove(&fr);
                }
            }
            StashingOption::Unstash => {
                let stashed_fragments = mem::take(&mut self.stashed_active_fragments);
                for fr in stashed_fragments {
                    self.fragments_intact.active.insert(fr);
                }
            }
        }
        let mut fresh_settled_st = IdleState::new(self.fragments_intact.time_now);
        mem::swap(&mut fresh_settled_st.fragments, &mut self.fragments_intact);
        mem::swap(&mut fresh_settled_st.pools, &mut self.pools_intact);
        fresh_settled_st
    }
}

/// The idea of TLB state automata is to minimize overhead of maintaining preview of modified state.
#[derive(Debug, Clone)]
pub enum TLBState<Fr, Pl: Stable> {
    /// State with no uncommitted changes.
    ///
    ///              Idle
    ///              |  \
    /// PartialPreview   Preview
    Idle(IdleState<Fr, Pl>),
    /// Modified state that reflects only consumption of fragments and full preview of pools.
    ///
    ///          PartialPreview
    ///              |  \
    ///           Idle   Preview
    PartialPreview(PartialPreviewState<Fr, Pl>),
    /// State with areas of uncommitted changes: consumption and production of fragments/pools.
    ///
    ///             Preview
    ///                |
    ///              Idle
    Preview(PreviewState<Fr, Pl>),
}

impl<Fr, Pl: Stable> Display for TLBState<Fr, Pl> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                TLBState::Idle(inner) => format!(
                    "TLBState::Idle(active: {} asks, {} bids)",
                    inner.fragments.active.asks.len(),
                    inner.fragments.active.bids.len()
                ),
                TLBState::PartialPreview(inner) => format!(
                    "TLBState::PartialPreview(active: {} asks, {} bids)",
                    inner.fragments_preview.active.asks.len(),
                    inner.fragments_preview.active.bids.len()
                ),
                TLBState::Preview(inner) => format!(
                    "TLBState::Preview(active: {} asks, {} bids)",
                    inner.active_fragments_preview.asks.len(),
                    inner.active_fragments_preview.bids.len()
                ),
            }
            .as_str(),
        )
    }
}

impl<Fr, Pl: Stable> TLBState<Fr, Pl> {
    pub fn new(time: u64) -> Self {
        Self::Idle(IdleState::new(time))
    }
}

impl<Fr, Pl: Stable> TLBState<Fr, Pl>
where
    Fr: Fragment + Ord + Copy,
{
    fn active_fragments(&self) -> &Fragments<Fr> {
        match self {
            TLBState::Idle(st) => &st.fragments.active,
            TLBState::PartialPreview(st) => &st.fragments_preview.active,
            TLBState::Preview(st) => &st.active_fragments_preview,
        }
    }
}

impl<Fr, Pl> TLBState<Fr, Pl>
where
    Fr: Fragment + Ord + Copy,
    Pl: Stable + Copy,
{
    fn move_into_partial_preview(&mut self, target: &mut PartialPreviewState<Fr, Pl>) {
        match self {
            // Transit into PartialPreview if state is untouched yet
            TLBState::Idle(st) => {
                trace!(target: "state", "TLBState::move_into_partial_preview: MOVING FROM IDLE");
                // Move untouched fragments/pools sets into fresh state.
                mem::swap(&mut target.fragments_preview, &mut st.fragments);
                mem::swap(&mut target.pools_intact, &mut st.pools);
                // Initialize pools preview with a copy of untouched pools.
                mem::swap(&mut target.pools_preview, &mut target.pools_intact.clone());
            }
            TLBState::PartialPreview(_) | TLBState::Preview(_) => {
                trace!(target: "state", "TLBState::move_into_partial_preview: NO-OP");
            }
        }
    }

    fn move_into_preview(&mut self, target: &mut PreviewState<Fr, Pl>) {
        match self {
            TLBState::Idle(st) => {
                trace!(target: "state", "TLBState::move_into_preview from IDLE");
                // Move untouched fragments/pools into preview state.
                mem::swap(&mut target.fragments_intact, &mut st.fragments);
                mem::swap(&mut target.pools_intact, &mut st.pools);
                // Move active fragments/pools to use as a preview.
                let mut active_fragments = target.fragments_intact.active.clone();
                mem::swap(&mut target.active_fragments_preview, &mut active_fragments);
                let mut pools = target.pools_intact.clone();
                mem::swap(&mut target.pools_preview, &mut pools);
            }
            TLBState::PartialPreview(st) => {
                trace!(target: "state", "TLBState::move_into_preview from PARTIAL_PREVIEW");
                // Copy active fragments/pools to use as a preview.
                let mut active_fragments = st.fragments_preview.active.clone();
                mem::swap(&mut target.active_fragments_preview, &mut active_fragments);
                mem::swap(&mut target.pools_preview, &mut st.pools_preview);
                // Return consumed fragments to reconstruct initial state.
                while let Some(fr) = st.consumed_active_fragments.pop() {
                    st.fragments_preview.active.insert(fr);
                }
                // Move untouched state into preview.
                mem::swap(&mut target.fragments_intact, &mut st.fragments_preview);
                mem::swap(&mut target.pools_intact, &mut st.pools_intact);
            }
            TLBState::Preview(_) => {}
        }
    }
}

impl<Fr, Pl, U> TLBState<Fr, Pl>
where
    Fr: Fragment<U = U> + Ord + Copy + Debug,
    Pl: Pool + Stable + Copy,
    U: PartialOrd,
{
    pub fn show_state(&self) -> String
    where
        Pl::StableId: Display,
        Pl: Display,
        Fr: Display,
    {
        let pools = self.pools().show_state();
        let fragments = self.active_fragments().show_state();
        format!("Fragments(active): {}, Pools: {}", fragments, pools)
    }

    pub fn best_fr_price(&self, side: SideM) -> Option<Side<AbsolutePrice>> {
        let active_fragments = self.active_fragments();
        let side_store = match side {
            SideM::Bid => &active_fragments.bids,
            SideM::Ask => &active_fragments.asks,
        };
        side_store.first().map(|fr| side.wrap(fr.price()))
    }

    /// Pick best fragment from either side
    pub fn pick_best_fr_either(&mut self, index_price: Option<AbsolutePrice>) -> Option<Fr> {
        trace!(target: "state", "pick_best_fr_either");
        self.pick_active_fr(|fragments| pick_best_fr_either(fragments, index_price))
    }

    /// Pick best fragment from the specified side if it matches the specified condition.
    pub fn try_pick_fr<F>(&mut self, side: SideM, test: F) -> Option<Fr>
    where
        F: FnOnce(&Fr) -> bool,
    {
        trace!(target: "state", "try_pick_fr");
        self.pick_active_fr(|af| try_pick_fr(af, side, test))
    }

    /// Add preview fragment [Fr].
    pub fn pre_add_fragment(&mut self, fr: Fr) {
        trace!(target: "state", "pre_add_fragment");
        let time = self.current_time();
        match (self, fr.time_bounds().lower_bound()) {
            // We have to transit to preview state.
            (this @ TLBState::Idle(_) | this @ TLBState::PartialPreview(_), lower_bound) => {
                let mut preview_st = PreviewState::new(time);
                this.move_into_preview(&mut preview_st);
                // Add fr into preview.
                match lower_bound {
                    Some(lower_bound) if lower_bound > time => {
                        preview_st.inactive_fragments_changeset.push((lower_bound, fr));
                    }
                    _ => preview_st.active_fragments_preview.insert(fr),
                }
                mem::swap(this, &mut TLBState::Preview(preview_st));
            }
            (TLBState::Preview(ref mut preview_st), lower_bound) => match lower_bound {
                Some(lb) if lb > time => preview_st.inactive_fragments_changeset.push((lb, fr)),
                _ => preview_st.active_fragments_preview.insert(fr),
            },
        }
    }

    /// Add preview pool [Pl].
    pub fn pre_add_pool(&mut self, pool: Pl) {
        match self {
            this @ TLBState::Idle(_) | this @ TLBState::PartialPreview(_) => {
                let mut preview_st = PreviewState::new(0);
                this.move_into_preview(&mut preview_st);
                // Add pool into preview.
                preview_st.pools_preview.update_pool(pool);
                mem::swap(this, &mut TLBState::Preview(preview_st));
            }
            TLBState::Preview(ref mut state) => state.pools_preview.update_pool(pool),
        }
    }

    /// Pick active fragment ensuring TLB is in proper state.
    fn pick_active_fr<F>(&mut self, f: F) -> Option<Fr>
    where
        F: FnOnce(&mut Fragments<Fr>) -> Option<Fr>,
    {
        let mut needs_transition = false;
        let res = match self {
            // Transit into PartialPreview if state is untouched yet
            TLBState::Idle(idle_st) => {
                let active_fragments = &mut idle_st.fragments.active;
                if let Some(choice) = f(active_fragments) {
                    needs_transition = true;
                    Some(choice)
                } else {
                    None
                }
            }
            TLBState::PartialPreview(busy_st) => {
                let active_fragments = &mut busy_st.fragments_preview.active;
                if let Some(choice) = f(active_fragments) {
                    busy_st.consumed_active_fragments.push(choice);
                    Some(choice)
                } else {
                    None
                }
            }
            TLBState::Preview(preview_st) => {
                let active_fragments = &mut preview_st.active_fragments_preview;
                f(active_fragments)
            }
        };

        if needs_transition {
            let mut busy_st = PartialPreviewState::new(0);
            self.move_into_partial_preview(&mut busy_st);
            busy_st.consumed_active_fragments.push(res.unwrap());
            mem::swap(self, &mut TLBState::PartialPreview(busy_st));
        }

        res
    }

    fn current_time(&self) -> u64 {
        match self {
            TLBState::Idle(st) => st.fragments.time_now,
            TLBState::PartialPreview(st) => st.fragments_preview.time_now,
            TLBState::Preview(st) => st.fragments_intact.time_now,
        }
    }
}

impl<Fr, Pl> TLBState<Fr, Pl>
where
    Fr: Fragment + Ord + Copy,
    Pl: Pool + Stable + Copy,
{
    pub fn best_pool_price(&self) -> Option<AbsolutePrice> {
        self.pools()
            .pools
            .values()
            .max_by_key(|p| p.quality())
            .map(|p| p.static_price())
    }

    pub fn try_select_pool(&self, trade_hint: Side<u64>) -> Option<(AbsolutePrice, Pl::StableId)> {
        let pools = self
            .pools()
            .pools
            .values()
            .map(|p| {
                let pr = p.real_price(trade_hint);
                (pr, p.stable_id())
            })
            .collect::<Vec<_>>();
        match trade_hint {
            Side::Bid(_) => pools.into_iter().min_by_key(|(p, _)| *p),
            Side::Ask(_) => pools.into_iter().max_by_key(|(p, _)| *p),
        }
    }

    pub fn try_pick_pool<F>(&mut self, test: F) -> Option<Pl>
    where
        F: Fn(&Pl) -> bool,
    {
        self.pick_pool(|pools| {
            for id in pools.quality_index.values() {
                match pools.pools.entry(*id) {
                    Entry::Occupied(pl) if test(pl.get()) => return Some(pl.remove()),
                    _ => {}
                }
            }
            None
        })
    }

    pub fn take_pool(&mut self, pid: &Pl::StableId) -> Option<Pl> {
        self.pick_pool(|pools| pools.pools.remove(pid))
    }

    /// Pick pool ensuring TLB is in proper state.
    fn pick_pool<F>(&mut self, f: F) -> Option<Pl>
    where
        F: FnOnce(&mut Pools<Pl>) -> Option<Pl>,
    {
        match self {
            // Transit into PartialPreview if state is untouched yet
            this @ TLBState::Idle(_) => {
                let mut busy_st = PartialPreviewState::new(0);
                this.move_into_partial_preview(&mut busy_st);
                let pools_preview = &mut busy_st.pools_preview;
                let result = f(pools_preview);
                mem::swap(this, &mut TLBState::PartialPreview(busy_st));
                result
            }
            TLBState::PartialPreview(busy_st) => {
                let pools_preview = &mut busy_st.pools_preview;
                f(pools_preview)
            }
            TLBState::Preview(preview_st) => {
                let pools_preview = &mut preview_st.pools_preview;
                f(pools_preview)
            }
        }
    }

    fn pools(&self) -> &Pools<Pl> {
        match self {
            TLBState::Idle(st) => &st.pools,
            TLBState::PartialPreview(st) => &st.pools_preview,
            TLBState::Preview(st) => &st.pools_preview,
        }
    }
}

fn pick_best_fr_either<Fr, U>(
    active_frontier: &mut Fragments<Fr>,
    index_price: Option<AbsolutePrice>,
) -> Option<Fr>
where
    Fr: Fragment<U = U> + Ord + Copy,
    U: PartialOrd,
{
    trace!("Picking best fragment");
    let best_bid = active_frontier.bids.pop_first();
    let best_ask = active_frontier.asks.pop_first();
    match (best_bid, best_ask) {
        (Some(bid), Some(ask)) => {
            let bid_is_underpriced = index_price.map(|ip| bid.price() < ip).unwrap_or(false);
            let ask_is_overpriced = index_price.map(|ip| ask.price() > ip).unwrap_or(false);
            let bid_is_heavier = bid.weight() >= ask.weight();
            if (bid_is_heavier && !bid_is_underpriced) || ask_is_overpriced {
                active_frontier.asks.insert(ask);
                trace!(
                    "All BIDs: {}",
                    active_frontier
                        .bids
                        .iter()
                        .map(|i| i.price().to_string())
                        .fold("".to_string(), |acc, x| acc.add(format!("{}, ", x).as_str()))
                );
                Some(bid)
            } else {
                active_frontier.bids.insert(bid);
                trace!(
                    "All ASKs: {}",
                    active_frontier
                        .asks
                        .iter()
                        .map(|i| i.price().to_string())
                        .fold("".to_string(), |x, axx| x.add(format!("{}, ", axx).as_str()))
                );
                Some(ask)
            }
        }
        (Some(any), None) | (None, Some(any)) => {
            trace!(
                "All BIDs: {}",
                active_frontier
                    .bids
                    .iter()
                    .map(|i| i.price().to_string())
                    .fold("".to_string(), |acc, x| acc.add(format!("{}, ", x).as_str()))
            );
            trace!(
                "All ASKs: {}",
                active_frontier
                    .asks
                    .iter()
                    .map(|i| i.price().to_string())
                    .fold("".to_string(), |x, axx| x.add(format!("{}, ", axx).as_str()))
            );
            Some(any)
        }
        _ => {
            trace!(target: "state", "No best fragment");
            None
        }
    }
}

fn try_pick_fr<Fr, F>(active_frontier: &mut Fragments<Fr>, side: SideM, test: F) -> Option<Fr>
where
    Fr: Fragment + Copy + Ord,
    F: FnOnce(&Fr) -> bool,
{
    let side = match side {
        SideM::Bid => &mut active_frontier.bids,
        SideM::Ask => &mut active_frontier.asks,
    };
    if let Some(best_fr) = side.pop_first() {
        if test(&best_fr) {
            return Some(best_fr);
        } else {
            side.insert(best_fr);
        }
    }
    None
}

/// Liquidity fragments spread across time axis.
#[derive(Debug, Clone, Eq, PartialEq)]
struct Chronology<Fr> {
    time_now: u64,
    active: Fragments<Fr>,
    inactive: BTreeMap<u64, Fragments<Fr>>,
}

impl<Fr> Chronology<Fr> {
    pub fn new(time_now: u64) -> Self {
        Self {
            time_now,
            active: Fragments::new(),
            inactive: BTreeMap::new(),
        }
    }
}

impl<Fr> Chronology<Fr>
where
    Fr: Fragment + OrderState + Ord + Copy,
{
    fn advance_clocks(&mut self, new_time: u64) {
        let new_slot = self
            .inactive
            .remove(&new_time)
            .unwrap_or_else(|| Fragments::new());
        let Fragments { asks, bids } = mem::replace(&mut self.active, new_slot);
        for fr in asks {
            if let StateTrans::Active(next_fr) = fr.with_updated_time(new_time) {
                self.active.asks.insert(next_fr);
            }
        }
        for fr in bids {
            if let StateTrans::Active(next_fr) = fr.with_updated_time(new_time) {
                self.active.bids.insert(next_fr);
            }
        }
        self.time_now = new_time;
    }

    fn remove_fragment(&mut self, fr: Fr) {
        if let Some(lower_bound) = fr.time_bounds().lower_bound() {
            if lower_bound > self.time_now {
                match self.inactive.entry(lower_bound) {
                    btree_map::Entry::Occupied(e) => {
                        match fr.side() {
                            SideM::Bid => e.into_mut().bids.remove(&fr),
                            SideM::Ask => e.into_mut().asks.remove(&fr),
                        };
                    }
                    btree_map::Entry::Vacant(_) => {}
                }
                return;
            }
        }
        trace!("Removing fragment from active frontier");
        match fr.side() {
            SideM::Bid => {
                self.active.bids.remove(&fr);
                trace!(
                    "All BIDs after removal: {}",
                    self.active
                        .bids
                        .iter()
                        .map(|i| i.price().to_string())
                        .fold("".to_string(), |acc, x| acc.add(format!("{}, ", x).as_str()))
                );
            }
            SideM::Ask => {
                self.active.asks.remove(&fr);
                trace!(
                    "All ASKs after removal: {}",
                    self.active
                        .bids
                        .iter()
                        .map(|i| i.price().to_string())
                        .fold("".to_string(), |acc, x| acc.add(format!("{}, ", x).as_str()))
                );
            }
        };
    }

    fn add_fragment(&mut self, fr: Fr) {
        match fr.time_bounds().lower_bound() {
            Some(lower_bound) if lower_bound > self.time_now => match self.inactive.entry(lower_bound) {
                btree_map::Entry::Vacant(e) => {
                    let mut fresh_fragments = Fragments::new();
                    fresh_fragments.insert(fr);
                    e.insert(fresh_fragments);
                }
                btree_map::Entry::Occupied(e) => {
                    e.into_mut().insert(fr);
                }
            },
            _ => {
                self.active.insert(fr);
                trace!(
                    "All BIDs after addition: {}",
                    self.active
                        .bids
                        .iter()
                        .map(|i| i.price().to_string())
                        .fold("".to_string(), |acc, x| acc.add(format!("{}, ", x).as_str()))
                );
                trace!(
                    "All ASKs after addition: {}",
                    self.active
                        .asks
                        .iter()
                        .map(|i| i.price().to_string())
                        .fold("".to_string(), |acc, x| acc.add(format!("{}, ", x).as_str()))
                );
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Fragments<Fr> {
    asks: BTreeSet<Fr>,
    bids: BTreeSet<Fr>,
}

impl<Fr> Fragments<Fr> {
    fn new() -> Self {
        Self {
            asks: BTreeSet::new(),
            bids: BTreeSet::new(),
        }
    }
}

impl<Fr> Fragments<Fr>
where
    Fr: Fragment + Ord,
{
    pub fn insert(&mut self, fr: Fr) {
        match fr.side() {
            SideM::Bid => self.bids.insert(fr),
            SideM::Ask => self.asks.insert(fr),
        };
    }

    pub fn remove(&mut self, fr: &Fr) {
        match fr.side() {
            SideM::Bid => self.bids.remove(fr),
            SideM::Ask => self.asks.remove(fr),
        };
    }

    pub fn show_state(&self) -> String
    where
        Fr: Display,
    {
        let asks = self
            .asks
            .iter()
            .map(|v| v.to_string())
            .fold("".to_string(), |acc, x| acc.add(format!("{}, ", x).as_str()));
        let bids = self
            .bids
            .iter()
            .map(|v| v.to_string())
            .fold("".to_string(), |acc, x| acc.add(format!("{}, ", x).as_str()));
        format!("asks: {}, bids: {}", asks, bids)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Pools<Pl: Stable> {
    pools: HashMap<Pl::StableId, Pl>,
    quality_index: BTreeMap<PoolQuality, Pl::StableId>,
}

impl<Pl: Stable> Pools<Pl> {
    pub fn new() -> Self {
        Self {
            pools: HashMap::new(),
            quality_index: BTreeMap::new(),
        }
    }

    pub fn show_state(&self) -> String
    where
        Pl::StableId: Display,
        Pl: Display,
    {
        self.pools
            .iter()
            .map(|(k, v)| format!("{} -> {}", k, v))
            .fold("".to_string(), |acc, x| acc.add(format!("{}, ", x).as_str()))
    }
}

impl<Pl> Pools<Pl>
where
    Pl: Pool + Stable + Copy,
{
    pub fn update_pool(&mut self, pool: Pl) {
        if let Some(old_pool) = self.pools.insert(pool.stable_id(), pool) {
            trace!(target: "state", "removing old pool {}", old_pool.stable_id());
            self.quality_index.remove(&old_pool.quality());
        }
        trace!(target: "state", "adding new pool id: {}, quality: {:?}", pool.stable_id(), pool.quality());
        self.quality_index.insert(pool.quality(), pool.stable_id());
    }
    pub fn remove_pool(&mut self, pool: Pl) {
        self.pools.remove(&pool.stable_id());
        self.quality_index.remove(&pool.quality());
    }
}

#[cfg(test)]
pub mod tests {
    use std::cmp::Ordering;
    use std::fmt::{Debug, Display, Formatter};

    use spectrum_offchain::data::Stable;

    use crate::execution_engine::liquidity_book::fragment::{Fragment, OrderState, StateTrans};
    use crate::execution_engine::liquidity_book::pool::Pool;
    use crate::execution_engine::liquidity_book::side::{Side, SideM};
    use crate::execution_engine::liquidity_book::state::{
        IdleState, PoolQuality, StashingOption, TLBState, VersionedState,
    };
    use crate::execution_engine::liquidity_book::time::TimeBounds;
    use crate::execution_engine::liquidity_book::types::{
        AbsolutePrice, ExBudgetUsed, ExCostUnits, ExFeeUsed, OutputAsset,
    };
    use crate::execution_engine::types::StableId;

    #[test]
    fn add_inactive_fragment() {
        let time_now = 1000u64;
        let ord = SimpleOrderPF::default_with_bounds(TimeBounds::After(time_now + 100));
        let mut s0 = IdleState::<_, SimpleCFMMPool>::new(time_now);
        s0.fragments.add_fragment(ord);
        assert_eq!(TLBState::Idle(s0).pick_best_fr_either(None), None);
    }

    #[test]
    fn pop_active_fragment() {
        let time_now = 1000u64;
        let ord = SimpleOrderPF::default_with_bounds(TimeBounds::None);
        let mut s0 = IdleState::<_, SimpleCFMMPool>::new(time_now);
        s0.fragments.add_fragment(ord);
        let mut s0_wrapped = TLBState::Idle(s0);
        assert_eq!(s0_wrapped.pick_best_fr_either(None), Some(ord));
        assert_eq!(s0_wrapped.pick_best_fr_either(None), None);
    }

    #[test]
    fn fragment_activation() {
        let time_now = 1000u64;
        let delta = 100u64;
        let ord = SimpleOrderPF::default_with_bounds(TimeBounds::After(time_now + delta));
        let mut s0 = IdleState::<_, SimpleCFMMPool>::new(time_now);
        s0.fragments.add_fragment(ord);
        assert_eq!(TLBState::Idle(s0.clone()).pick_best_fr_either(None), None);
        s0.fragments.advance_clocks(time_now + delta);
        assert_eq!(TLBState::Idle(s0).pick_best_fr_either(None), Some(ord));
    }

    #[test]
    fn fragment_deactivation() {
        let time_now = 1000u64;
        let delta = 100u64;
        let ord = SimpleOrderPF::default_with_bounds(TimeBounds::Until(time_now + delta));
        let mut s0 = IdleState::<_, SimpleCFMMPool>::new(time_now);
        s0.fragments.add_fragment(ord);
        assert_eq!(TLBState::Idle(s0.clone()).pick_best_fr_either(None), Some(ord));
        s0.fragments.advance_clocks(time_now + delta + 1);
        assert_eq!(TLBState::Idle(s0).pick_best_fr_either(None), None);
    }

    #[test]
    fn choose_best_fragment_bid_is_underpriced() {
        let time_now = 1000u64;
        let index_price = AbsolutePrice::new(1, 35);
        let ask = SimpleOrderPF::new(SideM::Ask, 1000, index_price, 100);
        let bid = SimpleOrderPF::new(SideM::Bid, 1000, AbsolutePrice::new(1, 40), 200);
        let mut s0 = IdleState::<_, SimpleCFMMPool>::new(time_now);
        s0.fragments.add_fragment(ask);
        s0.fragments.add_fragment(bid);
        assert_eq!(
            TLBState::Idle(s0).pick_best_fr_either(Some(index_price)),
            Some(ask)
        );
    }

    #[test]
    fn choose_best_fragment_ask_is_overpriced() {
        let time_now = 1000u64;
        let index_price = AbsolutePrice::new(1, 35);
        let ask = SimpleOrderPF::new(SideM::Ask, 1000, AbsolutePrice::new(1, 30), 100);
        let bid = SimpleOrderPF::new(SideM::Bid, 1000, index_price, 200);
        let mut s0 = IdleState::<_, SimpleCFMMPool>::new(time_now);
        s0.fragments.add_fragment(ask);
        s0.fragments.add_fragment(bid);
        assert_eq!(
            TLBState::Idle(s0).pick_best_fr_either(Some(index_price)),
            Some(bid)
        );
    }

    #[test]
    fn choose_best_fragment_both_orders_price_is_off() {
        let time_now = 1000u64;
        let index_price = AbsolutePrice::new(1, 35);
        let ask = SimpleOrderPF::new(SideM::Ask, 1000, AbsolutePrice::new(1, 30), 100);
        let bid = SimpleOrderPF::new(SideM::Bid, 1000, AbsolutePrice::new(1, 40), 200);
        let mut s0 = IdleState::<_, SimpleCFMMPool>::new(time_now);
        s0.fragments.add_fragment(ask);
        s0.fragments.add_fragment(bid);
        assert_eq!(
            TLBState::Idle(s0).pick_best_fr_either(Some(index_price)),
            Some(bid)
        );
    }

    #[test]
    fn settled_state_to_preview_active_fr() {
        let time_now = 1000u64;
        let delta = 100u64;
        let o1 = SimpleOrderPF::default_with_bounds(TimeBounds::Until(time_now + delta));
        let o2 = SimpleOrderPF::default_with_bounds(TimeBounds::None);
        let mut s0 = IdleState::<_, SimpleCFMMPool>::new(time_now);
        s0.fragments.add_fragment(o1);
        let s0_copy = s0.clone();
        let mut state = TLBState::Idle(s0);
        state.pre_add_fragment(o2);
        match state {
            TLBState::Preview(st) => {
                assert_eq!(st.fragments_intact, s0_copy.fragments);
                let preview = st.active_fragments_preview;
                assert!(preview.bids.contains(&o1) || preview.asks.contains(&o1));
                assert!(preview.bids.contains(&o2) || preview.asks.contains(&o2));
                dbg!(preview);
            }
            _ => panic!(),
        }
    }

    #[test]
    fn settled_state_to_preview_inactive_fr() {
        let time_now = 1000u64;
        let delta = 100u64;
        let o1 = SimpleOrderPF::default_with_bounds(TimeBounds::Until(time_now + delta));
        let o2 = SimpleOrderPF::default_with_bounds(TimeBounds::After(time_now + delta));
        let mut s0 = IdleState::<_, SimpleCFMMPool>::new(time_now);
        s0.fragments.add_fragment(o1);
        let s0_copy = s0.clone();
        let mut state = TLBState::Idle(s0);
        state.pre_add_fragment(o2);
        match state {
            TLBState::Preview(st) => {
                assert_eq!(st.fragments_intact, s0_copy.fragments);
                assert_eq!(
                    st.inactive_fragments_changeset,
                    vec![(o2.bounds.lower_bound().unwrap(), o2)]
                );
            }
            _ => panic!(),
        }
    }

    #[test]
    fn commit_preview_changes() {
        let time_now = 1000u64;
        let delta = 100u64;
        let o1 = SimpleOrderPF::default_with_bounds(TimeBounds::Until(time_now + delta));
        let o2 = SimpleOrderPF::default_with_bounds(TimeBounds::None);
        let mut s0 = IdleState::<_, SimpleCFMMPool>::new(time_now);
        s0.fragments.add_fragment(o1);
        let _s0_copy = s0.clone();
        let mut state = TLBState::Idle(s0);
        state.pre_add_fragment(o2);
        match state {
            TLBState::Preview(mut s1) => {
                let s1_copy = s1.clone();
                let s2 = s1.commit();
                for (t, fr) in s1_copy.inactive_fragments_changeset {
                    assert!(s2
                        .fragments
                        .inactive
                        .get(&t)
                        .map(|frs| frs.asks.contains(&fr) || frs.bids.contains(&fr))
                        .unwrap_or(false));
                }
                for fr in &s1_copy.active_fragments_preview.bids {
                    assert!(s2.fragments.active.bids.contains(&fr))
                }
                for fr in &s1_copy.active_fragments_preview.asks {
                    assert!(s2.fragments.active.asks.contains(&fr))
                }
            }
            _ => panic!(),
        }
    }

    #[test]
    fn rollback_preview_changes_deletion() {
        let time_now = 1000u64;
        let delta = 100u64;
        let o1 = SimpleOrderPF::default_with_bounds(TimeBounds::Until(time_now + delta));
        let o2 = SimpleOrderPF::default_with_bounds(TimeBounds::None);
        let o3 = SimpleOrderPF::default_with_bounds(TimeBounds::None);
        let mut s0 = IdleState::<_, SimpleCFMMPool>::new(time_now);
        s0.fragments.add_fragment(o1);
        s0.fragments.add_fragment(o2);
        let s0_copy = s0.clone();
        let mut state = TLBState::Idle(s0);
        // One new fragment added into the preview.
        state.pre_add_fragment(o3);
        // One old fragment removed from the preview.
        assert!(matches!(state.pick_best_fr_either(None), Some(_)));
        match state {
            TLBState::Preview(mut s1) => {
                let s2 = s1.rollback(StashingOption::Unstash);
                assert_eq!(s2.fragments, s0_copy.fragments);
                assert_eq!(s2.pools, s0_copy.pools);
            }
            _ => panic!(),
        }
    }

    #[test]
    fn rollback_part_preview_changes_deletion() {
        let time_now = 1000u64;
        let delta = 100u64;
        let o1 = SimpleOrderPF::default_with_bounds(TimeBounds::Until(time_now + delta));
        let o2 = SimpleOrderPF::default_with_bounds(TimeBounds::None);
        let mut s0 = IdleState::<_, SimpleCFMMPool>::new(time_now);
        s0.fragments.add_fragment(o1);
        s0.fragments.add_fragment(o2);
        let s0_copy = s0.clone();
        let mut state = TLBState::Idle(s0);
        // One old fragment removed from the preview.
        assert!(matches!(state.pick_best_fr_either(None), Some(_)));
        match state {
            TLBState::PartialPreview(mut s1) => {
                let s2 = s1.rollback(StashingOption::Unstash);
                assert_eq!(s2.fragments, s0_copy.fragments);
                assert_eq!(s2.pools, s0_copy.pools);
            }
            _ => panic!(),
        }
    }

    /// Order that supports partial filling.
    #[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
    pub struct SimpleOrderPF {
        pub source: StableId,
        pub side: SideM,
        pub input: u64,
        pub accumulated_output: u64,
        pub min_marginal_output: u64,
        pub price: AbsolutePrice,
        pub fee: u64,
        pub ex_budget: u64,
        pub cost_hint: ExCostUnits,
        pub bounds: TimeBounds<u64>,
    }

    impl Display for SimpleOrderPF {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.write_str(&*format!(
                "Ord(input={}, price={}, side={}, fee={})",
                self.input, self.price, self.side, self.fee
            ))
        }
    }

    impl PartialOrd for SimpleOrderPF {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for SimpleOrderPF {
        fn cmp(&self, other: &Self) -> Ordering {
            self.price.cmp(&other.price).then(self.source.cmp(&other.source))
        }
    }

    impl SimpleOrderPF {
        pub fn new(side: SideM, input: u64, price: AbsolutePrice, fee: u64) -> Self {
            Self {
                source: StableId::random(),
                side,
                input,
                accumulated_output: 0,
                min_marginal_output: 0,
                price,
                fee,
                ex_budget: 0,
                cost_hint: 10,
                bounds: TimeBounds::None,
            }
        }
        pub fn make(
            side: SideM,
            input: u64,
            price: AbsolutePrice,
            fee: u64,
            accumulated_output: u64,
            min_marginal_output: u64,
        ) -> Self {
            Self {
                source: StableId::random(),
                side,
                input,
                accumulated_output,
                min_marginal_output,
                price,
                fee,
                ex_budget: 0,
                cost_hint: 10,
                bounds: TimeBounds::None,
            }
        }
        pub fn default_with_bounds(bounds: TimeBounds<u64>) -> Self {
            Self {
                source: StableId::random(),
                side: SideM::Ask,
                input: 1000_000_000,
                accumulated_output: 0,
                min_marginal_output: 0,
                price: AbsolutePrice::new(1, 100),
                fee: 100,
                ex_budget: 0,
                cost_hint: 0,
                bounds,
            }
        }
    }

    impl Fragment for SimpleOrderPF {
        type U = u64;

        fn side(&self) -> SideM {
            self.side
        }

        fn input(&self) -> u64 {
            self.input
        }

        fn price(&self) -> AbsolutePrice {
            self.price
        }

        fn marginal_cost_hint(&self) -> ExCostUnits {
            self.cost_hint
        }

        fn time_bounds(&self) -> TimeBounds<u64> {
            self.bounds
        }

        fn linear_fee(
            &self,
            input_consumed: crate::execution_engine::liquidity_book::types::InputAsset<u64>,
        ) -> crate::execution_engine::liquidity_book::types::FeeAsset<u64> {
            self.fee * input_consumed / self.input
        }

        fn min_marginal_output(&self) -> OutputAsset<u64> {
            self.min_marginal_output
        }

        fn fee(&self) -> crate::execution_engine::liquidity_book::types::FeeAsset<u64> {
            self.fee
        }
    }

    impl OrderState for SimpleOrderPF {
        fn with_updated_time(self, time: u64) -> StateTrans<Self> {
            if self.bounds.contain(&time) {
                StateTrans::Active(self)
            } else {
                StateTrans::EOL
            }
        }

        fn with_applied_swap(
            mut self,
            removed_input: u64,
            added_output: u64,
        ) -> (StateTrans<Self>, ExBudgetUsed, ExFeeUsed) {
            self.input -= removed_input;
            self.accumulated_output += added_output;
            let budget_used = added_output * self.fee;
            let next_st = if self.input > 0 {
                StateTrans::Active(self)
            } else {
                StateTrans::EOL
            };
            (next_st, budget_used, ExFeeUsed::from(self.fee))
        }
    }

    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct SimpleCFMMPool {
        pub pool_id: StableId,
        pub reserves_base: u64,
        pub reserves_quote: u64,
        pub fee_num: u64,
    }

    impl Display for SimpleCFMMPool {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.write_str(&*format!("Pool(price={})", self.static_price()))
        }
    }

    impl Debug for SimpleCFMMPool {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.write_str(&*self.to_string())
        }
    }

    impl Stable for SimpleCFMMPool {
        type StableId = StableId;
        fn stable_id(&self) -> Self::StableId {
            self.pool_id
        }
        fn is_quasi_permanent(&self) -> bool {
            true
        }
    }

    impl Pool for SimpleCFMMPool {
        type U = u64;

        fn static_price(&self) -> AbsolutePrice {
            AbsolutePrice::new(self.reserves_quote, self.reserves_base)
        }

        fn real_price(&self, input: Side<u64>) -> AbsolutePrice {
            match input {
                Side::Bid(quote_input) => {
                    let (base_output, _) = self.swap(Side::Bid(quote_input));
                    AbsolutePrice::new(quote_input, base_output)
                }
                Side::Ask(base_input) => {
                    let (quote_output, _) = self.swap(Side::Ask(base_input));
                    AbsolutePrice::new(quote_output, base_input)
                }
            }
        }

        fn swap(mut self, input: Side<u64>) -> (u64, Self) {
            match input {
                Side::Bid(quote_input) => {
                    let base_output =
                        ((self.reserves_base as u128) * (quote_input as u128) * (self.fee_num as u128)
                            / ((self.reserves_quote as u128) * 1000u128
                                + (quote_input as u128) * (self.fee_num as u128)))
                            as u64;
                    self.reserves_quote += quote_input;
                    self.reserves_base -= base_output;
                    (base_output, self)
                }
                Side::Ask(base_input) => {
                    let quote_output =
                        ((self.reserves_quote as u128) * (base_input as u128) * (self.fee_num as u128)
                            / ((self.reserves_base as u128) * 1000u128
                                + (base_input as u128) * (self.fee_num as u128)))
                            as u64;
                    self.reserves_base += base_input;
                    self.reserves_quote -= quote_output;
                    (quote_output, self)
                }
            }
        }

        fn quality(&self) -> PoolQuality {
            PoolQuality::from(self.reserves_quote + self.reserves_base)
        }

        fn marginal_cost_hint(&self) -> Self::U {
            10
        }
    }
}
