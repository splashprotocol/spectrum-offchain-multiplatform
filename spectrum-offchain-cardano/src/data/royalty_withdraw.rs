use crate::data::deposit::DepositOrderValidation;
use crate::data::order::ClassicalOrder;
use crate::data::pool::{CFMMPoolAction, Rx, Ry};
use crate::data::royalty_pool::ROYALTY_DATUM_MAPPING;
use crate::data::{OnChainOrderId, PoolId};
use crate::deployment::ProtocolValidator::RoyaltyPoolV1RoyaltyWithdraw;
use crate::deployment::{
    test_address, DeployedScriptInfo, DeployedValidator, DeployedValidatorErased, RequiresValidator,
};
use cml_chain::plutus::PlutusData;
use cml_chain::transaction::TransactionOutput;
use cml_core::serialization::RawBytesEncoding;
use cml_crypto::{Ed25519KeyHash, Ed25519Signature, PublicKey};
use spectrum_cardano_lib::plutus_data::{ConstrPlutusDataExtension, DatumExtension, PlutusDataExtension};
use spectrum_cardano_lib::types::TryFromPData;
use spectrum_cardano_lib::{AssetClass, OutputRef, TaggedAmount, TaggedAssetClass};
use spectrum_offchain::data::Has;
use spectrum_offchain::ledger::TryFromLedger;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RoyaltyWithdrawConfig {
    pub pool_nft: PoolId,
    pub withdraw_royalty_x: TaggedAmount<Rx>,
    pub withdraw_royalty_y: TaggedAmount<Ry>,
    pub royalty_pub_key_hash: [u8; 28],
    pub royalty_pub_key: String,
    pub signature: String,
}

pub struct RoyaltyWithdrawDatumMapping {
    pub pool_nft: usize,
    pub withdraw_royalty_x: usize,
    pub withdraw_royalty_y: usize,
    pub royalty_pub_key_hash: usize,
    pub royalty_pub_key: usize,
    pub signature: usize,
}

pub const ROYALTY_WITHDRAW_DATUM_MAPPING: RoyaltyWithdrawDatumMapping = RoyaltyWithdrawDatumMapping {
    pool_nft: 0,
    withdraw_royalty_x: 1,
    withdraw_royalty_y: 2,
    royalty_pub_key_hash: 3,
    royalty_pub_key: 4,
    signature: 5,
};

impl TryFromPData for RoyaltyWithdrawConfig {
    fn try_from_pd(data: PlutusData) -> Option<Self> {
        let mut cpd = data.into_constr_pd()?;
        Some(Self {
            pool_nft: PoolId(
                AssetClass::try_from_pd(cpd.take_field(ROYALTY_WITHDRAW_DATUM_MAPPING.pool_nft)?)?
                    .into_token()?,
            ),
            withdraw_royalty_x: TaggedAmount::new(
                cpd.take_field(ROYALTY_WITHDRAW_DATUM_MAPPING.withdraw_royalty_x)?
                    .into_u64()?,
            ),
            withdraw_royalty_y: TaggedAmount::new(
                cpd.take_field(ROYALTY_WITHDRAW_DATUM_MAPPING.withdraw_royalty_y)?
                    .into_u64()?,
            ),
            royalty_pub_key_hash: cpd
                .take_field(ROYALTY_WITHDRAW_DATUM_MAPPING.royalty_pub_key_hash)?
                .into_bytes()?
                .try_into()
                .ok()?,
            royalty_pub_key: hex::encode(
                cpd.take_field(ROYALTY_WITHDRAW_DATUM_MAPPING.royalty_pub_key)?
                    .into_bytes()?,
            ),
            signature: hex::encode(
                cpd.take_field(ROYALTY_WITHDRAW_DATUM_MAPPING.signature)?
                    .into_bytes()?,
            ),
        })
    }
}

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
        if test_address(repr.address(), ctx) {
            let pd = repr.datum().clone()?.into_pd()?;
            let conf = RoyaltyWithdrawConfig::try_from_pd(pd)?;
            let royalty_withdraw = RoyaltyWithdraw {
                pool_nft: conf.pool_nft,
                withdraw_royalty_x: conf.withdraw_royalty_x,
                withdraw_royalty_y: conf.withdraw_royalty_y,
                royalty_pub_key_hash: Ed25519KeyHash::from(conf.royalty_pub_key_hash),
                royalty_pub_key: PublicKey::from_raw_hex(conf.royalty_pub_key.as_str()).ok()?,
                signature: Ed25519Signature::from_raw_hex(conf.signature.as_str()).ok()?,
            };
            return Some(Self {
                id: OnChainOrderId::from(ctx.select::<OutputRef>()),
                pool_id: royalty_withdraw.pool_nft,
                order: royalty_withdraw,
            });
        };
        None
    }
}
