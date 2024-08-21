use cml_multi_era::babbage::BabbageTransactionOutput;
use either::Either;

use bloom_offchain::execution_engine::bundled::Bundled;
use bloom_offchain_cardano::orders::adhoc::{AdhocFeeStructure, AdhocOrder};
use bloom_offchain_cardano::orders::limit::LimitOrderBounds;
use spectrum_cardano_lib::output::FinalizedTxOut;
use spectrum_cardano_lib::{OutputRef, Token};
use spectrum_offchain::data::order::SpecializedOrder;
use spectrum_offchain::data::{Baked, EntitySnapshot, Has, Stable, Tradable};
use spectrum_offchain::ledger::TryFromLedger;
use spectrum_offchain_cardano::creds::OperatorCred;
use spectrum_offchain_cardano::data::degen_quadratic_pool::DegenQuadraticPool;
use spectrum_offchain_cardano::data::deposit::DepositOrderBounds;
use spectrum_offchain_cardano::data::order::ClassicalAMMOrder;
use spectrum_offchain_cardano::data::pair::PairId;
use spectrum_offchain_cardano::data::pool::PoolBounds;
use spectrum_offchain_cardano::data::redeem::RedeemOrderBounds;
use spectrum_offchain_cardano::deployment::DeployedScriptInfo;
use spectrum_offchain_cardano::deployment::ProtocolValidator::{
    BalanceFnPoolDeposit, BalanceFnPoolRedeem, ConstFnFeeSwitchPoolDeposit, ConstFnFeeSwitchPoolRedeem,
    ConstFnFeeSwitchPoolSwap, ConstFnPoolDeposit, ConstFnPoolRedeem, ConstFnPoolSwap, DegenQuadraticPoolV1,
    LimitOrderV1, StableFnPoolT2TDeposit, StableFnPoolT2TRedeem,
};
use spectrum_offchain_cardano::utxo::ConsumedInputs;

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct AtomicCardanoEntity(pub Bundled<ClassicalAMMOrder, FinalizedTxOut>);

impl SpecializedOrder for AtomicCardanoEntity {
    type TOrderId = OutputRef;
    type TPoolId = Token;

    fn get_self_ref(&self) -> Self::TOrderId {
        self.0.get_self_ref()
    }

    fn get_pool_ref(&self) -> Self::TPoolId {
        self.0.get_pool_ref()
    }
}

impl<C> TryFromLedger<BabbageTransactionOutput, C> for AtomicCardanoEntity
where
    C: Copy
        + Has<OperatorCred>
        + Has<OutputRef>
        + Has<DeployedScriptInfo<{ ConstFnPoolSwap as u8 }>>
        + Has<DeployedScriptInfo<{ ConstFnPoolDeposit as u8 }>>
        + Has<DeployedScriptInfo<{ ConstFnPoolRedeem as u8 }>>
        + Has<DeployedScriptInfo<{ ConstFnFeeSwitchPoolSwap as u8 }>>
        + Has<DeployedScriptInfo<{ ConstFnFeeSwitchPoolDeposit as u8 }>>
        + Has<DeployedScriptInfo<{ ConstFnFeeSwitchPoolRedeem as u8 }>>
        + Has<DeployedScriptInfo<{ BalanceFnPoolDeposit as u8 }>>
        + Has<DeployedScriptInfo<{ BalanceFnPoolRedeem as u8 }>>
        + Has<DeployedScriptInfo<{ StableFnPoolT2TDeposit as u8 }>>
        + Has<DeployedScriptInfo<{ StableFnPoolT2TRedeem as u8 }>>
        + Has<DepositOrderBounds>
        + Has<RedeemOrderBounds>,
{
    fn try_from_ledger(repr: &BabbageTransactionOutput, ctx: &C) -> Option<Self> {
        ClassicalAMMOrder::try_from_ledger(repr, ctx).map(|inner| {
            Self(Bundled(
                inner,
                FinalizedTxOut::new(repr.clone(), ctx.select::<OutputRef>()),
            ))
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct EvolvingCardanoEntity(
    pub Bundled<Either<Baked<AdhocOrder, OutputRef>, Baked<DegenQuadraticPool, OutputRef>>, FinalizedTxOut>,
);

impl Stable for EvolvingCardanoEntity {
    type StableId = Token;
    fn stable_id(&self) -> Self::StableId {
        self.0.stable_id()
    }
    fn is_quasi_permanent(&self) -> bool {
        self.0.is_quasi_permanent()
    }
}

impl EntitySnapshot for EvolvingCardanoEntity {
    type Version = OutputRef;
    fn version(&self) -> Self::Version {
        self.0.version()
    }
}

impl Tradable for EvolvingCardanoEntity {
    type PairId = PairId;
    fn pair_id(&self) -> Self::PairId {
        self.0.pair_id()
    }
}

impl<C> TryFromLedger<BabbageTransactionOutput, C> for EvolvingCardanoEntity
where
    C: Copy
        + Has<OperatorCred>
        + Has<OutputRef>
        + Has<ConsumedInputs>
        + Has<DeployedScriptInfo<{ LimitOrderV1 as u8 }>>
        + Has<DeployedScriptInfo<{ DegenQuadraticPoolV1 as u8 }>>
        + Has<LimitOrderBounds>
        + Has<DepositOrderBounds>
        + Has<PoolBounds>
        + Has<AdhocFeeStructure>,
{
    fn try_from_ledger(repr: &BabbageTransactionOutput, ctx: &C) -> Option<Self> {
        <Either<Baked<AdhocOrder, OutputRef>, Baked<DegenQuadraticPool, OutputRef>>>::try_from_ledger(
            repr, ctx,
        )
        .map(|inner| {
            Self(Bundled(
                inner,
                FinalizedTxOut::new(repr.clone(), ctx.select::<OutputRef>()),
            ))
        })
    }
}