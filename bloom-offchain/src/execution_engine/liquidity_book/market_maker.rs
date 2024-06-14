use crate::execution_engine::liquidity_book::side::Side;
use crate::execution_engine::liquidity_book::types::AbsolutePrice;
use derive_more::{Display, Div, From, Into, Mul};
use num_rational::Ratio;
use std::cmp::Ordering;
use crate::execution_engine::liquidity_book::core::Make;

/// Price of a theoretical 0-swap in pool.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Div, Mul, From, Into, Display)]
pub struct StaticPrice(AbsolutePrice);

impl StaticPrice {
    pub fn unwrap(self) -> Ratio<u128> {
        self.0.unwrap()
    }
}

/// Pooled liquidity.
pub trait MarketMaker {
    type U;
    // Take liquidity corresponding to specified `input` from maker.
    fn take(self, input: Side<u64>) -> (Make, Self);
    // Fuse maker with the given liquidity `bin`. Inverse of `take`.
    fn fuse(self, make: Make) -> Self;
    /// Static price (regardless swap vol) in this pool.
    fn static_price(&self) -> StaticPrice;
    /// Real price of swap.
    fn real_price(&self, input: Side<u64>) -> AbsolutePrice;
    /// Output of a swap.
    fn swap(self, input: Side<u64>) -> (u64, Self);
    /// Quality of the pool.
    fn quality(&self) -> PoolQuality;
    /// How much (approximately) execution of this fragment will cost.
    fn marginal_cost_hint(&self) -> Self::U;
    // Is this maker active at the moment or not.
    fn is_active(&self) -> bool;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Into, From, Display)]
pub struct PoolQuality(u128);

impl PartialOrd for PoolQuality {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<u64> for PoolQuality {
    fn from(value: u64) -> Self {
        PoolQuality(value as u128)
    }
}

impl Ord for PoolQuality {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0).reverse()
    }
}