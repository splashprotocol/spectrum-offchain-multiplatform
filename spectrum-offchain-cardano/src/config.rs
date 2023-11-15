use serde::Deserialize;

use cardano_chain_sync::client::Point;
use cardano_explorer::data::ExplorerConfig;

#[derive(Deserialize)]
#[serde(bound = "'de: 'a")]
#[serde(rename_all = "camelCase")]
pub struct AppConfig<'a> {
    pub chain_sync: ChainSyncConfig,
    pub node: NodeConfig<'a>,
    pub tx_submission_buffer_size: usize,
    pub batcher_private_key: &'a str, //todo: store encrypted
    pub ref_scripts: RefScriptsConfig,
    pub explorer: ExplorerConfig<'a>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeConfig<'a> {
    pub path: &'a str,
    pub magic: u64,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChainSyncConfig {
    pub starting_point: Point,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RefScriptsConfig {
    pub pool_v1_ref: String,
    pub pool_v2_ref: String,
    pub swap_ref: String,
    pub deposit_ref: String,
    pub redeem_ref: String,
}