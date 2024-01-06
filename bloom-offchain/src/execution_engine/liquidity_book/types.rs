use derive_more::{Display, Div, Mul};
use num_rational::Ratio;

use crate::execution_engine::liquidity_book::side::{Side, SideM};

pub type ExecutionCost = u32;

/// Price of input asset denominated in units of output asset (Output/Input).
pub type RelativePrice = Ratio<u128>;

pub type FeePerOutput = Ratio<u128>;

/// Price of base asset denominated in units of quote asset.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Div, Mul, Display)]
pub struct AbsolutePrice(Ratio<u128>);

impl AbsolutePrice {
    #[inline]
    pub fn new(numer: u64, denom: u64) -> AbsolutePrice {
        Self(Ratio::new(numer as u128, denom as u128))
    }

    pub fn from_price(side: SideM, price: RelativePrice) -> Self {
        Self(match side {
            // In case of bid the price in order is base/quote, so we inverse it.
            SideM::Bid => price.pow(-1),
            SideM::Ask => price,
        })
    }

    #[inline]
    pub const fn numer(&self) -> &u128 {
        &self.0.numer()
    }

    #[inline]
    pub const fn denom(&self) -> &u128 {
        &self.0.denom()
    }
}

impl Side<AbsolutePrice> {
    /// Compare prices on opposite sides.
    pub fn overlaps(self, that: AbsolutePrice) -> bool {
        match self {
            // Bid price must be higher than Ask price to overlap.
            Side::Bid(this) => this >= that,
            // Ask price must be lower than Bid side to overlap.
            Side::Ask(this) => this <= that,
        }
    }

    /// Compare prices on the same side.
    pub fn better_than(self, that: AbsolutePrice) -> bool {
        match self {
            // If we compare Bid prices, then we favor highest price.
            Side::Bid(this) => this >= that,
            // If we compare Ask prices, then we favor lowest price.
            Side::Ask(this) => this <= that,
        }
    }
}

pub type BatcherFeePerQuote = Ratio<u64>;

pub type LPFee = Ratio<u64>;
