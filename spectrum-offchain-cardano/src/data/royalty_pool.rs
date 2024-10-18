use crate::data::order::PoolNft;
use crate::data::pool::{Lq, Rx, Ry};
use cml_chain::plutus::PlutusData;
use spectrum_cardano_lib::plutus_data::ConstrPlutusDataExtension;
use spectrum_cardano_lib::plutus_data::PlutusDataExtension;
use spectrum_cardano_lib::types::TryFromPData;
use spectrum_cardano_lib::TaggedAssetClass;
use std::fmt::Debug;

#[derive(Debug)]
pub struct RoyaltyPoolConfig {
    pub pool_nft: TaggedAssetClass<PoolNft>,
    pub asset_x: TaggedAssetClass<Rx>,
    pub asset_y: TaggedAssetClass<Ry>,
    pub asset_lq: TaggedAssetClass<Lq>,
    pub lp_fee_num: u64,
    pub treasury_fee_num: u64,
    pub royalty_fee_num: u64,
    pub treasury_x: u64,
    pub treasury_y: u64,
    pub royalty_x: u64,
    pub royalty_y: u64,
    pub royalty_pub_key_hash_256: Vec<u8>,
    pub royalty_nonce: u64,
}

pub struct RoyaltyPoolDatumMapping {
    pub pool_nft: usize,
    pub asset_x: usize,
    pub asset_y: usize,
    pub asset_lq: usize,
    pub lp_fee_num: usize,
    pub treasury_fee_num: usize,
    pub royalty_fee_num: usize,
    pub treasury_x: usize,
    pub treasury_y: usize,
    pub royalty_x: usize,
    pub royalty_y: usize,
    pub royalty_pub_key_hash256: usize,
    pub royalty_nonce: usize,
}

pub const ROYALTY_DATUM_MAPPING: RoyaltyPoolDatumMapping = RoyaltyPoolDatumMapping {
    pool_nft: 0,
    asset_x: 1,
    asset_y: 2,
    asset_lq: 3,
    lp_fee_num: 4,
    treasury_fee_num: 5,
    royalty_fee_num: 6,
    treasury_x: 7,
    treasury_y: 8,
    royalty_x: 9,
    royalty_y: 10,
    royalty_pub_key_hash256: 13,
    royalty_nonce: 14,
};

impl TryFromPData for RoyaltyPoolConfig {
    fn try_from_pd(data: PlutusData) -> Option<Self> {
        let mut cpd = data.into_constr_pd()?;
        Some(Self {
            pool_nft: TaggedAssetClass::try_from_pd(cpd.take_field(ROYALTY_DATUM_MAPPING.pool_nft)?)?,
            asset_x: TaggedAssetClass::try_from_pd(cpd.take_field(ROYALTY_DATUM_MAPPING.asset_x)?)?,
            asset_y: TaggedAssetClass::try_from_pd(cpd.take_field(ROYALTY_DATUM_MAPPING.asset_y)?)?,
            asset_lq: TaggedAssetClass::try_from_pd(cpd.take_field(ROYALTY_DATUM_MAPPING.asset_lq)?)?,
            lp_fee_num: cpd.take_field(ROYALTY_DATUM_MAPPING.lp_fee_num)?.into_u64()?,
            treasury_fee_num: cpd
                .take_field(ROYALTY_DATUM_MAPPING.treasury_fee_num)?
                .into_u64()?,
            royalty_fee_num: cpd
                .take_field(ROYALTY_DATUM_MAPPING.royalty_fee_num)?
                .into_u64()?,
            treasury_x: cpd.take_field(ROYALTY_DATUM_MAPPING.treasury_x)?.into_u64()?,
            treasury_y: cpd.take_field(ROYALTY_DATUM_MAPPING.treasury_y)?.into_u64()?,
            royalty_x: cpd.take_field(ROYALTY_DATUM_MAPPING.royalty_x)?.into_u64()?,
            royalty_y: cpd.take_field(ROYALTY_DATUM_MAPPING.royalty_y)?.into_u64()?,
            royalty_pub_key_hash_256: cpd
                .take_field(ROYALTY_DATUM_MAPPING.royalty_pub_key_hash256)?
                .into_bytes()?,
            royalty_nonce: cpd.take_field(ROYALTY_DATUM_MAPPING.royalty_nonce)?.into_u64()?,
        })
    }
}

mod tests {
    use crate::creds::OperatorCred;
    use crate::data::cfmm_pool::ConstFnPool;
    use crate::data::pool::{AnyPool, PoolValidation};
    use crate::data::royalty_pool::RoyaltyPoolConfig;
    use crate::deployment::ProtocolValidator::{
        ConstFnPoolFeeSwitch, ConstFnPoolFeeSwitchBiDirFee, ConstFnPoolFeeSwitchV2, ConstFnPoolV1,
        ConstFnPoolV2, LimitOrderV1, RoyaltyPoolV1,
    };
    use crate::deployment::{DeployedScriptInfo, DeployedValidators, ProtocolScriptHashes};
    use crate::handler_context::{ConsumedIdentifiers, ConsumedInputs, ProducedIdentifiers};
    use cml_chain::transaction::TransactionOutput;
    use cml_core::serialization::Deserialize;
    use cml_crypto::{Ed25519KeyHash, TransactionHash};
    use spectrum_cardano_lib::{OutputRef, Token};
    use spectrum_offchain::data::Has;
    use spectrum_offchain::ledger::TryFromLedger;
    use spectrum_offchain::small_set::SmallVec;
    use type_equalities::IsEqual;

    struct Context {
        oref: OutputRef,
        royalty_pool: DeployedScriptInfo<{ RoyaltyPoolV1 as u8 }>,
        const_fn_pool_v1: DeployedScriptInfo<{ ConstFnPoolV1 as u8 }>,
        const_fn_pool_v2: DeployedScriptInfo<{ ConstFnPoolV2 as u8 }>,
        fee_switch_v1: DeployedScriptInfo<{ ConstFnPoolFeeSwitch as u8 }>,
        fee_switch_v2: DeployedScriptInfo<{ ConstFnPoolFeeSwitchV2 as u8 }>,
        fee_switch_bi_dir: DeployedScriptInfo<{ ConstFnPoolFeeSwitchBiDirFee as u8 }>,
        cred: OperatorCred,
        consumed_inputs: ConsumedInputs,
        consumed_identifiers: ConsumedIdentifiers<Token>,
        produced_identifiers: ProducedIdentifiers<Token>,
        pool_validation: PoolValidation,
    }

    impl Has<DeployedScriptInfo<{ RoyaltyPoolV1 as u8 }>> for Context {
        fn select<U: IsEqual<DeployedScriptInfo<{ RoyaltyPoolV1 as u8 }>>>(
            &self,
        ) -> DeployedScriptInfo<{ RoyaltyPoolV1 as u8 }> {
            self.royalty_pool
        }
    }

    impl Has<DeployedScriptInfo<{ ConstFnPoolV1 as u8 }>> for Context {
        fn select<U: IsEqual<DeployedScriptInfo<{ ConstFnPoolV1 as u8 }>>>(
            &self,
        ) -> DeployedScriptInfo<{ ConstFnPoolV1 as u8 }> {
            self.const_fn_pool_v1
        }
    }

    impl Has<DeployedScriptInfo<{ ConstFnPoolV2 as u8 }>> for Context {
        fn select<U: IsEqual<DeployedScriptInfo<{ ConstFnPoolV2 as u8 }>>>(
            &self,
        ) -> DeployedScriptInfo<{ ConstFnPoolV2 as u8 }> {
            self.const_fn_pool_v2
        }
    }

    impl Has<DeployedScriptInfo<{ ConstFnPoolFeeSwitch as u8 }>> for Context {
        fn select<U: IsEqual<DeployedScriptInfo<{ ConstFnPoolFeeSwitch as u8 }>>>(
            &self,
        ) -> DeployedScriptInfo<{ ConstFnPoolFeeSwitch as u8 }> {
            self.fee_switch_v1
        }
    }

    impl Has<DeployedScriptInfo<{ ConstFnPoolFeeSwitchV2 as u8 }>> for Context {
        fn select<U: IsEqual<DeployedScriptInfo<{ ConstFnPoolFeeSwitchV2 as u8 }>>>(
            &self,
        ) -> DeployedScriptInfo<{ ConstFnPoolFeeSwitchV2 as u8 }> {
            self.fee_switch_v2
        }
    }

    impl Has<DeployedScriptInfo<{ ConstFnPoolFeeSwitchBiDirFee as u8 }>> for Context {
        fn select<U: IsEqual<DeployedScriptInfo<{ ConstFnPoolFeeSwitchBiDirFee as u8 }>>>(
            &self,
        ) -> DeployedScriptInfo<{ ConstFnPoolFeeSwitchBiDirFee as u8 }> {
            self.fee_switch_bi_dir
        }
    }

    impl Has<PoolValidation> for Context {
        fn select<U: IsEqual<PoolValidation>>(&self) -> PoolValidation {
            self.pool_validation
        }
    }

    const POOL_UTXO: &str = "a300581d7035684cc4ef3cd83588efca4f5216cca923f50db5cc83d90070536ad801821a05f5e100a3581c4b3459fd18a1dbabe207cd19c9951a9fac9f5c0f9c384e3d97efba26a14574657374441a05f5e100581cd8be588d8f9531905dd9e7c9ab197af9f2cd363f4fbc0ab09c5e62baa1436e667401581c8fa6738a791b1cd5f85a5b741dc0bd8a18c223bcda6c89dd52ceaa04a1426c711b7ffffffff4143dff028201d81858d8d8799fd8799f581cd8be588d8f9531905dd9e7c9ab197af9f2cd363f4fbc0ab09c5e62ba436e6674ffd8799f4040ffd8799f581c4b3459fd18a1dbabe207cd19c9951a9fac9f5c0f9c384e3d97efba26457465737444ffd8799f581c8fa6738a791b1cd5f85a5b741dc0bd8a18c223bcda6c89dd52ceaa04426c71ff1927101a000186a0192710000000009fd8799fd87a9f581cfb119a292524f162fe89d02bc9a2d0e0d8a1764f729aff3f99e4709bffffff405820f0bb1b4a3ffa2e954def5ed167b3723103465f6ab85b19057ed79ca14e18406a00ff";

    #[test]
    fn try_read() {
        const TX: &str = "a035c1cb245735680dcb3c46a9a3e692fbf550c8a5d7c4ada1471f97cc92dc55";
        const IX: u64 = 0;
        const ORDER_IX: u64 = 0;
        let oref = OutputRef::new(TransactionHash::from_hex(TX).unwrap(), IX);
        let raw_deployment = std::fs::read_to_string("/Users/aleksandr/IdeaProjects/spectrum-offchain-multiplatform/bloom-cardano-agent/resources/preprod.deployment.json").expect("Cannot load deployment file");
        let deployment: DeployedValidators =
            serde_json::from_str(&raw_deployment).expect("Invalid deployment file");
        let scripts = ProtocolScriptHashes::from(&deployment);
        let ctx = Context {
            oref,
            royalty_pool: scripts.royalty_pool_v1,
            const_fn_pool_v1: scripts.const_fn_pool_v1,
            const_fn_pool_v2: scripts.const_fn_pool_v2,
            fee_switch_v1: scripts.const_fn_pool_fee_switch,
            fee_switch_v2: scripts.const_fn_pool_fee_switch_v2,
            fee_switch_bi_dir: scripts.const_fn_pool_fee_switch_bidir_fee,
            cred: OperatorCred(Ed25519KeyHash::from([0u8; 28])),
            consumed_inputs: SmallVec::new(vec![oref].into_iter()).into(),
            consumed_identifiers: Default::default(),
            produced_identifiers: Default::default(),
            pool_validation: PoolValidation {
                min_n2t_lovelace: 10,
                min_t2t_lovelace: 10,
            },
        };
        let bearer = TransactionOutput::from_cbor_bytes(&*hex::decode(POOL_UTXO).unwrap()).unwrap();
        let ord = ConstFnPool::try_from_ledger(&bearer, &ctx).expect("LimitOrder expected");
        println!("Pool: {:?}", ord);
    }
}
