use crate::data::deposit::{ClassicalOnChainDeposit, Deposit, DepositOrderValidation};
use crate::data::order::{ClassicalOrder, OrderType};
use crate::data::pool::{CFMMPoolAction, Lq, Rx, Ry};
use crate::data::redeem::ClassicalOnChainRedeem;
use crate::data::{OnChainOrderId, PoolId};
use crate::deployment::ProtocolValidator::{
    BalanceFnPoolDeposit, BalanceFnPoolRedeem, ConstFnFeeSwitchPoolDeposit, ConstFnFeeSwitchPoolRedeem,
    ConstFnPoolDeposit, ConstFnPoolRedeem, RoyaltyPoolV1RoyaltyWithdraw, StableFnPoolT2TDeposit,
    StableFnPoolT2TRedeem,
};
use crate::deployment::{
    test_address, DeployedScriptInfo, DeployedValidator, DeployedValidatorErased, RequiresValidator,
};
use cml_chain::transaction::TransactionOutput;
use spectrum_cardano_lib::{OutputRef, TaggedAmount};
use spectrum_offchain::data::Has;
use spectrum_offchain::ledger::TryFromLedger;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct RoyaltyWithdraw {
    pub pool_nft: PoolId,
    pub withdraw_royalty_x: TaggedAmount<Rx>,
    pub withdraw_royalty_y: TaggedAmount<Ry>,
    pub signature: [u8; 32],
}

pub type OnChainRoyaltyWithdraw = ClassicalOrder<OnChainOrderId, RoyaltyWithdraw>;

impl<Ctx> RequiresValidator<Ctx> for OnChainRoyaltyWithdraw
where
    Ctx: Has<DeployedValidator<{ RoyaltyPoolV1RoyaltyWithdraw as u8 }>>,
{
    fn get_validator(&self, ctx: &Ctx) -> DeployedValidatorErased {
        ctx.get().erased()
    }
}

impl Into<CFMMPoolAction> for OnChainRoyaltyWithdraw {
    fn into(self) -> CFMMPoolAction {
        CFMMPoolAction::RoyaltyWithdraw
    }
}

impl<Ctx> TryFromLedger<TransactionOutput, Ctx> for OnChainRoyaltyWithdraw
where
    Ctx: Has<OutputRef>
        + Has<DeployedScriptInfo<{ RoyaltyPoolV1RoyaltyWithdraw as u8 }>>
        + Has<DepositOrderValidation>,
{
    fn try_from_ledger(repr: &TransactionOutput, ctx: &Ctx) -> Option<Self> {
        if test_address(repr.address(), ctx) {};
        None
    }
}
