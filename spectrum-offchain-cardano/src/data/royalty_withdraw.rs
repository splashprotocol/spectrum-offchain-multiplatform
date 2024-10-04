use crate::data::deposit::DepositOrderValidation;
use crate::data::order::ClassicalOrder;
use crate::data::pool::{CFMMPoolAction, Rx, Ry};
use crate::data::{OnChainOrderId, PoolId};
use crate::deployment::ProtocolValidator::RoyaltyPoolV1RoyaltyWithdraw;
use crate::deployment::{
    test_address, DeployedScriptInfo, DeployedValidator, DeployedValidatorErased, RequiresValidator,
};
use cml_chain::transaction::TransactionOutput;
use cml_crypto::{Ed25519KeyHash, Ed25519Signature, PublicKey};
use spectrum_cardano_lib::{OutputRef, TaggedAmount};
use spectrum_offchain::data::Has;
use spectrum_offchain::ledger::TryFromLedger;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RoyaltyWithdraw {
    pub pool_nft: PoolId,
    pub withdraw_royalty_x: TaggedAmount<Rx>,
    pub withdraw_royalty_y: TaggedAmount<Ry>,
    pub royalty_pub_key_hash: Ed25519KeyHash,
    pub royalty_pub_key: PublicKey,
    pub signature: Ed25519Signature,
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
