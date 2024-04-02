use cml_multi_era::babbage::BabbageTransactionOutput;
use log::trace;

use bloom_derivation::{Fragment, Stable, Tradable};
use bloom_offchain::execution_engine::liquidity_book::fragment::{OrderState, StateTrans};
use bloom_offchain::execution_engine::liquidity_book::side::SideM;
use bloom_offchain::execution_engine::liquidity_book::types::{ExBudgetUsed, ExFeeUsed};
use spectrum_cardano_lib::NetworkTime;
use spectrum_offchain::data::Has;
use spectrum_offchain::ledger::TryFromLedger;
use spectrum_offchain_cardano::creds::OperatorCred;
use spectrum_offchain_cardano::deployment::DeployedScriptHash;
use spectrum_offchain_cardano::deployment::ProtocolValidator::LimitOrderV1;
use spectrum_offchain_cardano::utxo::ConsumedInputs;

use crate::orders::limit::LimitOrder;

pub mod limit;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Fragment, Stable, Tradable)]
pub enum AnyOrder {
    Spot(LimitOrder),
}

impl OrderState for AnyOrder {
    fn with_updated_time(self, time: u64) -> StateTrans<Self> {
        match self {
            AnyOrder::Spot(spot) => spot.with_updated_time(time).map(AnyOrder::Spot),
        }
    }
    fn with_applied_swap(
        self,
        removed_input: u64,
        added_output: u64,
    ) -> (StateTrans<Self>, ExBudgetUsed, ExFeeUsed) {
        match self {
            AnyOrder::Spot(spot) => {
                let (tx, budget, fee) = spot.with_applied_swap(removed_input, added_output);
                (tx.map(AnyOrder::Spot), budget, fee)
            }
        }
    }
}

impl<C> TryFromLedger<BabbageTransactionOutput, C> for AnyOrder
where
    C: Has<OperatorCred> + Has<ConsumedInputs> + Has<DeployedScriptHash<{ LimitOrderV1 as u8 }>>,
{
    fn try_from_ledger(repr: &BabbageTransactionOutput, ctx: &C) -> Option<Self> {
        LimitOrder::try_from_ledger(repr, ctx).map(|s| AnyOrder::Spot(s))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Stateful<O, S> {
    pub order: O,
    pub state: S,
}

impl<O, S> Stateful<O, S> {
    pub fn new(order: O, state: S) -> Self {
        Self { order, state }
    }
}

/// State of order compatible with TLB.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct TLBCompatibleState {
    /// Side of the order relative to pair it maps to.
    pub side: SideM,
    pub time_now: NetworkTime,
}
