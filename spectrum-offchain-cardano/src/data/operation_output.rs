use cml_chain::address::{BaseAddress, EnterpriseAddress};
use cml_chain::assets::MultiAsset;
use cml_chain::certs::StakeCredential;
use cml_chain::genesis::network_info::NetworkInfo;
use cml_chain::transaction::{ConwayFormatTxOut, TransactionOutput};
use cml_chain::{Coin, Value};
use cml_crypto::Ed25519KeyHash;

use spectrum_cardano_lib::{TaggedAmount, TaggedAssetClass};
use spectrum_offchain::ledger::IntoLedger;

use crate::data::order::Quote;

#[derive(Debug, Clone)]
pub struct SwapOutput {
    pub quote_asset: TaggedAssetClass<Quote>,
    pub quote_amount: TaggedAmount<Quote>,
    pub ada_residue: Coin,
    pub redeemer_pkh: Ed25519KeyHash,
    pub redeemer_stake_pkh: Option<Ed25519KeyHash>,
}

impl IntoLedger<TransactionOutput, ()> for SwapOutput {
    fn into_ledger(self, _ctx: ()) -> TransactionOutput {
        let addr = if let Some(stake_pkh) = self.redeemer_stake_pkh {
            BaseAddress::new(
                //todo: network id from config
                NetworkInfo::mainnet().network_id(),
                StakeCredential::new_pub_key(self.redeemer_pkh),
                StakeCredential::new_pub_key(stake_pkh),
            )
            .to_address()
        } else {
            EnterpriseAddress::new(
                //todo: network id from config
                NetworkInfo::mainnet().network_id(),
                StakeCredential::new_pub_key(self.redeemer_pkh),
            )
        };

        let mut ma = MultiAsset::new();

        let ada_from_quote = if self.quote_asset.is_native() {
            self.quote_amount.untag()
        } else {
            let (policy, name) = self.quote_asset.untag().into_token().unwrap();
            ma.set(policy, name.into(), self.quote_amount.untag());
            0
        };

        let ada = self.ada_residue + ada_from_quote;

        TransactionOutput::new_conway_format_tx_out(ConwayFormatTxOut {
            address: addr,
            amount: Value::new(ada, ma),
            datum_option: None,
            script_reference: None,
            encodings: None,
        })
    }
}
