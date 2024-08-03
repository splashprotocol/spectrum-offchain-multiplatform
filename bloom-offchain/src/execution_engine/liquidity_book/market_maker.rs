use crate::execution_engine::liquidity_book::core::{MakeInProgress, Next, Unit};
use crate::execution_engine::liquidity_book::side::OnSide;
use crate::execution_engine::liquidity_book::types::AbsolutePrice;
use derive_more::{Display, Div, From, Into, Mul};
use num_rational::Ratio;
use std::cmp::Ordering;

/// Price of a theoretical 0-swap in pool.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Div, Mul, From, Into, Display)]
pub struct SpotPrice(AbsolutePrice);

impl SpotPrice {
    pub fn unwrap(self) -> Ratio<u128> {
        self.0.unwrap()
    }
}

#[derive(Copy, Clone, Debug)]
pub struct AbsoluteReserves {
    pub base: u64,
    pub quote: u64,
}

/// Pooled liquidity.
pub trait MarketMaker {
    type U;
    /// Static price (regardless swap vol) in this pool.
    fn static_price(&self) -> SpotPrice;
    /// Real price of swap.
    fn real_price(&self, input: OnSide<u64>) -> Option<AbsolutePrice>;
    /// Quality of the pool.
    fn quality(&self) -> PoolQuality;
    /// How much (approximately) execution of this fragment will cost.
    fn marginal_cost_hint(&self) -> Self::U;
    /// How much base and quote asset is available.
    fn liquidity(&self) -> AbsoluteReserves;
    /// Is this MM active at the moment or not.
    fn is_active(&self) -> bool;
}

/// Pooled liquidity.
pub trait MakerBehavior: Sized {
    /// Output of a swap.
    fn swap(self, input: OnSide<u64>) -> Next<Self, Unit>;
}

pub struct Excess {
    pub base: u64,
    pub quote: u64,
}

pub trait MakerBalance: Sized {
    fn balance(&self, that: Self) -> Option<(Self, Excess)>;
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
