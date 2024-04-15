use std::{fmt::Formatter, time::Duration};

use cml_chain::utils::BigInteger;
use cml_chain::{
    address::EnterpriseAddress,
    certs::StakeCredential,
    plutus::{ConstrPlutusData, ExUnits, PlutusData},
    transaction::{DatumOption, TransactionOutput},
    PolicyId, Value,
};
use cml_crypto::{PublicKey, RawBytesEncoding, ScriptHash};
use cml_multi_era::babbage::BabbageTransactionOutput;
use spectrum_cardano_lib::plutus_data::DatumExtension;
use spectrum_cardano_lib::transaction::TransactionOutputExtension;
use spectrum_cardano_lib::types::TryFromPData;
use spectrum_cardano_lib::OutputRef;
use spectrum_offchain::ledger::TryFromLedger;
use spectrum_offchain_cardano::deployment::{test_address, DeployedScriptHash};
use uplc_pallas_codec::utils::{Int, PlutusBytes};

use spectrum_cardano_lib::{
    plutus_data::{ConstrPlutusDataExtension, IntoPlutusData, PlutusDataExtension},
    Token,
};
use spectrum_offchain::{
    data::{Has, Identifier, Stable},
    ledger::IntoLedger,
};
use spectrum_offchain_cardano::parametrized_validators::apply_params_validator;

use crate::deployment::ProtocolValidator;
use crate::entities::Snapshot;
use crate::protocol_config::{GTAuthName, GTAuthPolicy, VEFactoryAuthName};
use crate::{
    constants::{MAX_LOCK_TIME_SECONDS, MINT_WEIGHTING_POWER_SCRIPT, VOTING_ESCROW_SCRIPT},
    protocol_config::{NodeMagic, OperatorCreds, VEFactoryAuthPolicy},
    time::{NetworkTime, ProtocolEpoch},
};

pub type VotingEscrowSnapshot = Snapshot<VotingEscrow, OutputRef>;

#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Debug)]
pub struct VotingEscrowId(Token);

impl Identifier for VotingEscrowId {
    type For = VotingEscrowSnapshot;
}

#[derive(Copy, Clone, Debug)]
pub struct VotingEscrow {
    pub gov_token_amount: u64,
    pub gt_policy: PolicyId,
    pub locked_until: Lock,
    pub ve_factory_auth_policy: PolicyId,
    pub max_ex_fee: u32,
    pub version: u32,
    pub last_wp_epoch: u32,
    pub last_gp_deadline: u32,
}

impl VotingEscrow {
    pub fn voting_power(&self, current_posix_time: u64) -> u64 {
        match self.locked_until {
            Lock::Def(network_time) => {
                if network_time < current_posix_time {
                    0
                } else {
                    self.gov_token_amount * (network_time - current_posix_time) / 1000 / MAX_LOCK_TIME_SECONDS
                }
            }
            Lock::Indef(d) => self.gov_token_amount * d.as_secs() / MAX_LOCK_TIME_SECONDS,
        }
    }
}

impl<C> TryFromLedger<BabbageTransactionOutput, C> for VotingEscrowSnapshot
where
    C: Has<VEFactoryAuthPolicy>
        + Has<VEFactoryAuthName>
        + Has<GTAuthPolicy>
        + Has<GTAuthName>
        + Has<OutputRef>
        + Has<DeployedScriptHash<{ ProtocolValidator::VotingEscrow as u8 }>>,
{
    fn try_from_ledger(repr: &BabbageTransactionOutput, ctx: &C) -> Option<Self> {
        if test_address(repr.address(), ctx) {
            let value = repr.value().clone();
            let VotingEscrowConfig {
                locked_until,
                owner,
                max_ex_fee,
                version,
                last_wp_epoch,
                last_gp_deadline,
            } = VotingEscrowConfig::try_from_pd(repr.datum()?.into_pd()?)?;

            let ve_factory_auth_policy = ctx.select::<VEFactoryAuthPolicy>().0;
            let ve_factory_auth_qty = value
                .multiasset
                .get(&ve_factory_auth_policy, &ctx.select::<VEFactoryAuthName>().0)?;
            assert_eq!(ve_factory_auth_qty, 1);
            let gt_policy = ctx.select::<GTAuthPolicy>().0;
            let gov_token_amount = value.multiasset.get(&gt_policy, &ctx.select::<GTAuthName>().0)?;

            let voting_escrow = VotingEscrow {
                gov_token_amount,
                gt_policy,
                locked_until,
                ve_factory_auth_policy,
                max_ex_fee,
                version,
                last_wp_epoch,
                last_gp_deadline,
            };
            let output_ref = ctx.select::<OutputRef>();
            return Some(Snapshot::new(voting_escrow, output_ref));
        }
        None
    }
}

impl Stable for VotingEscrow {
    type StableId = PolicyId;
    fn stable_id(&self) -> Self::StableId {
        self.ve_factory_auth_policy
    }
    fn is_quasi_permanent(&self) -> bool {
        true
    }
}

pub struct VotingEscrowConfig {
    pub locked_until: Lock,
    pub owner: Vec<u8>,
    pub max_ex_fee: u32,
    pub version: u32,
    pub last_wp_epoch: u32,
    pub last_gp_deadline: u32,
}

impl IntoPlutusData for VotingEscrowConfig {
    fn into_pd(self) -> PlutusData {
        let mut constr = ConstrPlutusData::new(0, vec![self.locked_until.into_pd()]);
        constr.set_field(1, PlutusData::new_bytes(self.owner));
        constr.set_field(2, PlutusData::new_integer(self.max_ex_fee.into()));
        constr.set_field(3, PlutusData::new_integer(self.version.into()));
        constr.set_field(4, PlutusData::new_integer(self.last_wp_epoch.into()));
        constr.set_field(5, PlutusData::new_integer(self.last_gp_deadline.into()));
        PlutusData::ConstrPlutusData(constr)
    }
}

impl TryFromPData for VotingEscrowConfig {
    fn try_from_pd(data: PlutusData) -> Option<Self> {
        let mut cpd = data.into_constr_pd()?;
        let locked_until = Lock::try_from_pd(cpd.take_field(0)?)?;
        let owner = cpd.take_field(1)?.into_bytes()?;
        let max_ex_fee = cpd.take_field(2)?.into_u64()? as u32;
        let version = cpd.take_field(3)?.into_u64()? as u32;
        let last_wp_epoch = cpd.take_field(4)?.into_u64()? as u32;
        let last_gp_deadline = cpd.take_field(5)?.into_u64()? as u32;

        Some(Self {
            locked_until,
            owner,
            max_ex_fee,
            version,
            last_wp_epoch,
            last_gp_deadline,
        })
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Lock {
    Def(NetworkTime),
    Indef(Duration),
}

impl IntoPlutusData for Lock {
    fn into_pd(self) -> PlutusData {
        match self {
            Lock::Def(n) => PlutusData::ConstrPlutusData(ConstrPlutusData::new(
                0,
                vec![PlutusData::new_integer(n.into())],
            )),
            Lock::Indef(d) => PlutusData::ConstrPlutusData(ConstrPlutusData::new(
                1,
                vec![PlutusData::new_integer(d.as_millis().into())],
            )),
        }
    }
}

impl TryFromPData for Lock {
    fn try_from_pd(data: PlutusData) -> Option<Self> {
        let mut cpd = data.into_constr_pd()?;
        if let Some(fields) = cpd.take_field(0) {
            let fields = fields.into_vec()?;
            if let Some(pd) = fields.first() {
                let n = pd.into_u64()?;
                return Some(Lock::Def(n));
            }
        } else if let Some(fields) = cpd.take_field(1) {
            let fields = fields.into_vec()?;
            if let Some(pd) = fields.first() {
                let millis = pd.into_u64()?;
                return Some(Lock::Indef(Duration::from_millis(millis)));
            }
        }

        None
    }
}

pub fn unsafe_update_ve_state(data: &mut PlutusData, last_poll_epoch: ProtocolEpoch) {
    let cpd = data.get_constr_pd_mut().unwrap();
    cpd.set_field(4, PlutusData::new_integer(last_poll_epoch.into()))
}
pub enum VotingEscrowAction {
    /// Apply governance action.
    Governance,
    /// Add budget (ADA) to funds execution of Gov actions or increase lock time.
    AddBudgetOrExtend,
    /// Redeem liqudity for voting power.
    Redeem { ve_factory_in_ix: u32 },
}

impl IntoPlutusData for VotingEscrowAction {
    fn into_pd(self) -> PlutusData {
        match self {
            VotingEscrowAction::Governance => PlutusData::ConstrPlutusData(ConstrPlutusData::new(0, vec![])),
            VotingEscrowAction::AddBudgetOrExtend => {
                PlutusData::ConstrPlutusData(ConstrPlutusData::new(1, vec![]))
            }
            VotingEscrowAction::Redeem { ve_factory_in_ix } => PlutusData::ConstrPlutusData(
                ConstrPlutusData::new(2, vec![PlutusData::Integer(BigInteger::from(ve_factory_in_ix))]),
            ),
        }
    }
}

pub struct VotingEscrowAuthorizedAction {
    pub action: VotingEscrowAction,
    /// Hash of the script authorized to witness the TX.
    pub witness: ScriptHash,
    /// Version to which the action can be applied.
    pub version: u32,
    /// Proof that the owner did authorize the action with the specified version of the voting escrow.
    pub signature: Vec<u8>,
}

pub struct RedeemerVotingEscrowAuthorizedActionMapping {
    pub action: usize,
    /// Hash of the script authorized to witness the TX.
    pub witness: usize,
    /// Version to which the action can be applied.
    pub version: usize,
    /// Proof that the owner did authorize the action with the specified version of the voting escrow.
    pub signature: usize,
}

const VEAA_REDEEMER_MAPPING: RedeemerVotingEscrowAuthorizedActionMapping =
    RedeemerVotingEscrowAuthorizedActionMapping {
        action: 0,
        witness: 1,
        version: 2,
        signature: 3,
    };

impl IntoPlutusData for VotingEscrowAuthorizedAction {
    fn into_pd(self) -> PlutusData {
        let mut cpd = ConstrPlutusData::new(VEAA_REDEEMER_MAPPING.action as u64, vec![self.action.into_pd()]);
        cpd.set_field(
            VEAA_REDEEMER_MAPPING.witness,
            PlutusData::new_bytes(self.witness.to_raw_bytes().to_vec()),
        );
        cpd.set_field(
            VEAA_REDEEMER_MAPPING.version,
            PlutusData::new_integer(BigInteger::from(self.version)),
        );
        cpd.set_field(
            VEAA_REDEEMER_MAPPING.signature,
            PlutusData::new_bytes(self.signature),
        );
        PlutusData::ConstrPlutusData(cpd)
    }
}

pub const VOTING_ESCROW_EX_UNITS: ExUnits = ExUnits {
    mem: 500_000,
    steps: 200_000_000,
    encodings: None,
};

pub const WEIGHTING_POWER_EX_UNITS: ExUnits = ExUnits {
    mem: 500_000,
    steps: 200_000_000,
    encodings: None,
};

pub const ORDER_WITNESS_EX_UNITS: ExUnits = ExUnits {
    mem: 500_000,
    steps: 200_000_000,
    encodings: None,
};

pub enum MintAction {
    MintPower {
        binder: u32,
        ve_in_ix: u32,
        proposal_in_ix: u32,
    },
    Burn,
}

impl IntoPlutusData for MintAction {
    fn into_pd(self) -> PlutusData {
        match self {
            MintAction::MintPower {
                binder,
                ve_in_ix,
                proposal_in_ix,
            } => PlutusData::ConstrPlutusData(ConstrPlutusData::new(
                0,
                vec![
                    PlutusData::Integer(BigInteger::from(binder)),
                    PlutusData::Integer(BigInteger::from(ve_in_ix)),
                    PlutusData::Integer(BigInteger::from(proposal_in_ix)),
                ],
            )),
            MintAction::Burn => PlutusData::ConstrPlutusData(ConstrPlutusData::new(0, vec![])),
        }
    }
}

pub const MIN_ADA_IN_BOX: u64 = 1_000_000;

pub fn compute_mint_weighting_power_policy_id(
    zeroth_epoch_start: u64,
    proposal_auth_policy: PolicyId,
    gt_policy: PolicyId,
) -> PolicyId {
    let params_pd = uplc::PlutusData::Array(vec![
        uplc::PlutusData::BigInt(uplc::BigInt::Int(Int::from(zeroth_epoch_start as i64))),
        uplc::PlutusData::BoundedBytes(PlutusBytes::from(proposal_auth_policy.to_raw_bytes().to_vec())),
        uplc::PlutusData::BoundedBytes(PlutusBytes::from(gt_policy.to_raw_bytes().to_vec())),
    ]);
    apply_params_validator(params_pd, MINT_WEIGHTING_POWER_SCRIPT)
}

pub fn compute_voting_escrow_policy_id(ve_factory_auth_policy: PolicyId) -> PolicyId {
    let params_pd = uplc::PlutusData::Array(vec![uplc::PlutusData::BoundedBytes(PlutusBytes::from(
        ve_factory_auth_policy.to_raw_bytes().to_vec(),
    ))]);
    apply_params_validator(params_pd, VOTING_ESCROW_SCRIPT)
}
