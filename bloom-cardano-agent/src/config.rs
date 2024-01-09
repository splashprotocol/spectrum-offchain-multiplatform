use bloom_offchain_cardano::operator_address::RewardAddress;
use cardano_chain_sync::client::Point;
use cardano_explorer::data::ExplorerConfig;
use spectrum_offchain_cardano::ref_scripts::ReferenceSources;

#[derive(serde::Deserialize)]
#[serde(bound = "'de: 'a")]
#[serde(rename_all = "camelCase")]
pub struct AppConfig<'a> {
    pub chain_sync: ChainSyncConfig<'a>,
    pub node: NodeConfig<'a>,
    pub tx_submission_buffer_size: usize,
    pub batcher_private_key: &'a str, //todo: store encrypted
    pub ref_scripts: ReferenceSources,
    pub explorer: ExplorerConfig<'a>,
    pub reward_address: RewardAddress,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeConfig<'a> {
    pub path: &'a str,
    pub magic: u64,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChainSyncConfig<'a> {
    pub starting_point: Point,
    pub db_path: &'a str,
}