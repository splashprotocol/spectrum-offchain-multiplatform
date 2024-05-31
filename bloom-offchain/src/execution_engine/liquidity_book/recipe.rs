use std::cmp::max;
use std::fmt::{Debug, Display, Formatter};

use log::{info, trace};
use num_rational::Ratio;

use crate::execution_engine::bundled::Bundled;
use crate::execution_engine::liquidity_book::fragment::{Fragment, OrderState, StateTrans};
use crate::execution_engine::liquidity_book::side::SideM;
use crate::execution_engine::liquidity_book::types::{FeeAsset, InputAsset, OutputAsset};

/// A recipe ready to be executed.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LinkedExecutionRecipe<Fr, Pl, Src>(pub Vec<LinkedTerminalInstruction<Fr, Pl, Src>>);

/// A recipe ready to be executed.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ExecutionRecipe<Fr, Pl>(Vec<TerminalInstruction<Fr, Pl>>);

impl<Fr, Pl> ExecutionRecipe<Fr, Pl> {
    pub fn try_from(rec: IntermediateRecipe<Fr, Pl>) -> Result<Self, Option<Vec<Fr>>>
    where
        Fr: Fragment + OrderState + Copy + Display,
        Pl: Display,
    {
        if rec.is_complete() {
            let unsatisfied_fragments = rec.unsatisfied_fragments();
            if unsatisfied_fragments.is_empty() {
                let IntermediateRecipe {
                    mut terminal,
                    remainder,
                } = rec;
                if let Some(rem) = remainder {
                    terminal.push(TerminalInstruction::Fill(rem.into()));
                }
                Ok(Self(terminal))
            } else {
                Err(Some(unsatisfied_fragments))
            }
        } else {
            Err(None)
        }
    }

    pub fn instructions(self) -> Vec<TerminalInstruction<Fr, Pl>> {
        self.0
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IntermediateRecipe<Fr, Pl> {
    pub terminal: Vec<TerminalInstruction<Fr, Pl>>,
    pub remainder: Option<PartialFill<Fr>>,
}

impl<Fr: Display, Pl: Display> Display for IntermediateRecipe<Fr, Pl> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut terminal_instructions = String::new();

        self.terminal.iter().for_each(|terminal| {
            terminal_instructions.push_str(&terminal.to_string());
            terminal_instructions.push_str(", ");
        });

        let mut remainder = String::new();

        match &self.remainder {
            None => remainder.push_str("None"),
            Some(partial_fill) => remainder.push_str(&partial_fill.to_string()),
        }

        f.write_str(&*format!(
            "IntermediateRecipe(terminal={}, remainder={})",
            terminal_instructions, remainder
        ))
    }
}

impl<Fr: Display, Pl: Display> IntermediateRecipe<Fr, Pl>
where
    Fr: Fragment + Copy,
{
    pub fn new(fr: Fr) -> Self {
        Self {
            terminal: Vec::new(),
            remainder: Some(PartialFill::empty(fr)),
        }
    }

    pub fn empty() -> Self {
        Self {
            terminal: Vec::new(),
            remainder: None,
        }
    }

    pub fn push(&mut self, instruction: TerminalInstruction<Fr, Pl>) {
        self.terminal.push(instruction)
    }

    pub fn terminate(&mut self, instruction: TerminalInstruction<Fr, Pl>) {
        self.push(instruction);
        self.remainder = None;
    }

    pub fn set_remainder(&mut self, remainder: PartialFill<Fr>) {
        self.remainder = Some(remainder);
    }

    pub fn is_complete(&self) -> bool {
        let terminal_fragments = self.terminal.len();
        trace!("[is_complete] Check is_complete for {}", self.to_string());
        info!("terminal_fragments: {:?}", terminal_fragments);
        info!("self.remainder.is_some(): {:?}", self.remainder.is_some());

        terminal_fragments >= 2 || (terminal_fragments > 0 && self.remainder.is_some())
    }

    pub fn unsatisfied_fragments(&self) -> Vec<Fr> {
        let not_ok_terminal_fills = self.terminal.iter().filter_map(|x| match x {
            TerminalInstruction::Fill(fill) if fill.added_output < fill.target_fr.min_marginal_output() => {
                Some(fill.target_fr)
            }
            _ => None,
        });
        let not_ok_non_terminal_fills = self
            .remainder
            .as_ref()
            .filter(|fill| fill.accumulated_output < fill.target.min_marginal_output())
            .map(|fill| fill.target);
        not_ok_terminal_fills.chain(not_ok_non_terminal_fills).collect()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum LinkedTerminalInstruction<Fr, Pl, Src> {
    Fill(LinkedFill<Fr, Src>),
    Swap(LinkedSwap<Pl, Src>),
}

impl<Fr, Pl, Src> LinkedTerminalInstruction<Fr, Pl, Src> {
    pub fn scale_budget(&mut self, scale: Ratio<u64>) -> i64 {
        match self {
            LinkedTerminalInstruction::Fill(fill) => {
                let old_val = fill.budget_used;
                let new_val = fill.budget_used * scale.numer() / scale.denom();
                fill.budget_used = new_val;
                let delta = new_val as i64 - old_val as i64;
                delta
            }
            LinkedTerminalInstruction::Swap(_) => 0,
        }
    }

    pub fn correct_budget(&mut self, val: i64) -> i64 {
        match self {
            LinkedTerminalInstruction::Fill(fill) => {
                let old_val = fill.budget_used as i64;
                let new_val = max(old_val + val, 0);
                fill.budget_used = new_val as u64;
                let delta = new_val - old_val;
                delta
            }
            LinkedTerminalInstruction::Swap(_) => 0,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TerminalInstruction<Fr, Pl> {
    Fill(Fill<Fr>),
    Swap(Swap<Pl>),
}

impl<Fr: Display, Pl: Display> Display for TerminalInstruction<Fr, Pl> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let formatted_entity = match self {
            TerminalInstruction::Fill(fill) => format!("{}", fill),
            TerminalInstruction::Swap(swap) => format!("{}", swap),
        };
        f.write_str(&*format!("TerminalInstruction({})", formatted_entity))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LinkedFill<Fr, Src> {
    pub target_fr: Bundled<Fr, Src>,
    pub next_fr: StateTrans<Fr>,
    pub removed_input: InputAsset<u64>,
    pub added_output: OutputAsset<u64>,
    pub budget_used: FeeAsset<u64>,
    pub fee_used: FeeAsset<u64>,
}

impl<Fr, Src> LinkedFill<Fr, Src> {
    pub fn from_fill(fill: Fill<Fr>, target_src: Src) -> Self {
        Self {
            target_fr: Bundled(fill.target_fr, target_src),
            next_fr: fill.next_fr,
            removed_input: fill.removed_input,
            added_output: fill.added_output,
            budget_used: fill.budget_used,
            fee_used: fill.fee_used,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Fill<Fr> {
    pub target_fr: Fr,
    /// Next fragment [Fr] resulted from this transaction.
    pub next_fr: StateTrans<Fr>,
    /// Input asset removed as a result of this transaction.
    pub removed_input: InputAsset<u64>,
    /// Output asset added as a result of this transaction.
    pub added_output: OutputAsset<u64>,
    /// Overall execution budget used.
    pub budget_used: FeeAsset<u64>,
    /// Execution fee charged.
    pub fee_used: FeeAsset<u64>,
}

impl<Fr: Display> Display for Fill<Fr> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*format!(
            "Fill(target_fr={}, next_fr={}, removed_input={}, added_output={}, budget_used={}, fee_used={})",
            self.target_fr,
            self.next_fr,
            self.removed_input,
            self.added_output,
            self.budget_used,
            self.fee_used
        ))
    }
}

impl<Fr: Fragment> Fill<Fr> {
    pub fn new(
        target: Fr,
        transition: StateTrans<Fr>,
        added_output: OutputAsset<u64>,
        budget_used: FeeAsset<u64>,
        fee_used: FeeAsset<u64>,
    ) -> Self {
        Self {
            removed_input: target.input(),
            budget_used,
            next_fr: transition,
            target_fr: target,
            added_output,
            fee_used,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct PartialFill<Fr> {
    pub target: Fr,
    pub remaining_input: u64,
    pub accumulated_output: u64,
}

impl<Fr: Display> Display for PartialFill<Fr> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*format!(
            "PartialFill(target={}, remaining_input={}, accumulated_output={})",
            self.target, self.remaining_input, self.accumulated_output
        ))
    }
}

impl<Fr> PartialFill<Fr>
where
    Fr: Fragment + OrderState + Copy,
{
    /// Force fill target fragment.
    /// Does not guarantee that the fragment is actually fully satisfied.
    pub fn filled_unsafe(self) -> Fill<Fr> {
        let (tx, budget_used, fee_used) = self
            .target
            .with_applied_swap(self.target.input(), self.accumulated_output);
        Fill {
            target_fr: self.target,
            next_fr: tx,
            removed_input: self.target.input(),
            added_output: self.accumulated_output,
            budget_used,
            fee_used,
        }
    }
}

impl<Fr: Fragment> From<PartialFill<Fr>> for Fill<Fr>
where
    Fr: OrderState + Copy,
{
    fn from(value: PartialFill<Fr>) -> Self {
        let removed = value.target.input() - value.remaining_input;
        let added = value.accumulated_output;
        let (transition, budget_used, fee_used) = value.target.with_applied_swap(removed, added);
        Self {
            removed_input: removed,
            next_fr: transition,
            added_output: added,
            target_fr: value.target,
            budget_used,
            fee_used,
        }
    }
}

impl<Fr> PartialFill<Fr>
where
    Fr: Fragment,
{
    pub fn new(target: Fr, remaining_input: u64, added_output: u64) -> Self {
        Self {
            target,
            remaining_input,
            accumulated_output: added_output,
        }
    }

    pub fn empty(fr: Fr) -> Self {
        Self {
            remaining_input: fr.input(),
            target: fr,
            accumulated_output: 0,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LinkedSwap<Pl, Src> {
    pub target: Bundled<Pl, Src>,
    pub transition: Pl,
    pub side: SideM,
    pub input: u64,
    pub output: u64,
}

impl<Pl, Src> LinkedSwap<Pl, Src> {
    pub fn from_swap(swap: Swap<Pl>, target_src: Src) -> Self {
        Self {
            target: Bundled(swap.target, target_src),
            transition: swap.transition,
            side: swap.side,
            input: swap.input,
            output: swap.output,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Swap<Pl> {
    pub target: Pl,
    pub transition: Pl,
    pub side: SideM,
    pub input: u64,
    pub output: u64,
}

impl<Pl: Display> Display for Swap<Pl> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*format!(
            "Swap(target={}, transition={}, side={}, input={}, output={})",
            self.target, self.transition, self.side, self.input, self.output
        ))
    }
}
