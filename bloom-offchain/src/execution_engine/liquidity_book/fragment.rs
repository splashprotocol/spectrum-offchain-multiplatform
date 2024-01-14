use crate::execution_engine::liquidity_book::side::SideM;
use crate::execution_engine::liquidity_book::time::TimeBounds;
use crate::execution_engine::liquidity_book::types::{AbsolutePrice, ExecutionCost, FeePerOutput};

/// Order as a state machine.
pub trait OrderState: Sized {
    fn with_updated_time(self, time: u64) -> StateTrans<Self>;
    fn with_updated_liquidity(self, removed_input: u64, added_output: u64) -> StateTrans<Self>;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum StateTrans<T> {
    /// Next state is available.
    Active(T),
    /// Order is exhausted.
    EOL,
}

impl<T> StateTrans<T> {
    pub fn map<B, F>(self, f: F) -> StateTrans<B>
    where
        F: FnOnce(T) -> B,
    {
        match self {
            StateTrans::Active(t) => StateTrans::Active(f(t)),
            StateTrans::EOL => StateTrans::EOL,
        }
    }
}

/// Immutable discrete fragment of liquidity available at a specified timeframe at a specified price.
/// Fragment is a projection of an order [OrderState] at a specific point on time axis.
pub trait Fragment {
    /// Side of the fragment relative to pair it maps to.
    fn side(&self) -> SideM;
    fn input(&self) -> u64;
    /// Price of base asset in quote asset.
    fn price(&self) -> AbsolutePrice;
    fn fee(&self) -> FeePerOutput;
    fn cost_hint(&self) -> ExecutionCost;
    fn time_bounds(&self) -> TimeBounds<u64>;
}
