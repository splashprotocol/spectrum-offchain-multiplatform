use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::marker::PhantomData;
use std::sync::Arc;

use async_trait::async_trait;
use cml_chain::crypto::hash::hash_transaction;
use cml_chain::transaction::{Transaction, TransactionOutput};
use futures::{Sink, SinkExt};
use log::trace;
use tokio::sync::Mutex;

use cardano_chain_sync::data::LedgerTxEvent;
use cardano_mempool_sync::data::MempoolUpdate;
use spectrum_cardano_lib::OutputRef;
use spectrum_offchain::box_resolver::persistence::EntityRepo;
use spectrum_offchain::combinators::EitherOrBoth;
use spectrum_offchain::data::unique_entity::{Confirmed, StateUpdate, Unconfirmed};
use spectrum_offchain::data::OnChainEntity;
use spectrum_offchain::event_sink::event_handler::EventHandler;
use spectrum_offchain::ledger::TryFromLedger;

pub struct ConfirmedUpdateHandler<TSink, TEntity, TRepo>
where
    TEntity: OnChainEntity,
{
    pub topic: TSink,
    pub entities: Arc<Mutex<TRepo>>,
    pub pd: PhantomData<TEntity>,
}

impl<TSink, TEntity, TRepo> ConfirmedUpdateHandler<TSink, TEntity, TRepo>
where
    TEntity: OnChainEntity + TryFromLedger<TransactionOutput, OutputRef> + Clone,
    TEntity::TEntityId: Clone,
{
    pub fn new(topic: TSink, entities: Arc<Mutex<TRepo>>) -> Self {
        Self {
            topic,
            entities,
            pd: Default::default(),
        }
    }
}

async fn extract_transitions<TEntity, TRepo>(
    entities: Arc<Mutex<TRepo>>,
    tx: Transaction,
) -> Vec<EitherOrBoth<TEntity, TEntity>>
where
    TEntity: OnChainEntity + TryFromLedger<TransactionOutput, OutputRef> + Clone,
    TEntity::TEntityId: Clone,
    TEntity::TStateId: From<OutputRef> + Copy,
    TRepo: EntityRepo<TEntity>,
{
    let mut consumed_entities = HashMap::<TEntity::TEntityId, TEntity>::new();
    for i in &tx.body.inputs {
        let state_id = TEntity::TStateId::from(OutputRef::from((i.transaction_id, i.index)));
        let entities = entities.lock().await;
        if entities.may_exist(state_id).await {
            if let Some(entity) = entities.get_state(state_id).await {
                let entity_id = entity.get_self_ref();
                consumed_entities.insert(entity_id, entity);
            }
        }
    }
    let mut created_entities = HashMap::<TEntity::TEntityId, TEntity>::new();
    let tx_hash = hash_transaction(&tx.body);
    for (i, o) in tx.body.outputs.iter().enumerate() {
        let o_ref = OutputRef::from((tx_hash, i as u64));
        if let Some(entity) = TEntity::try_from_ledger(o.clone(), o_ref) {
            let entity_id = entity.get_self_ref();
            created_entities.insert(entity_id.clone(), entity);
        }
    }
    let consumed_keys = consumed_entities.keys().cloned().collect::<HashSet<_>>();
    let created_keys = created_entities.keys().cloned().collect::<HashSet<_>>();

    consumed_keys
        .union(&created_keys)
        .flat_map(|k| {
            EitherOrBoth::try_from((consumed_entities.remove(k), created_entities.remove(k)))
                .map(|x| vec![x])
                .unwrap_or(Vec::new())
        })
        .collect()
}

#[async_trait(?Send)]
impl<TSink, TEntity, TRepo> EventHandler<LedgerTxEvent> for ConfirmedUpdateHandler<TSink, TEntity, TRepo>
where
    TSink: Sink<Confirmed<StateUpdate<TEntity>>> + Unpin,
    TEntity: OnChainEntity + TryFromLedger<TransactionOutput, OutputRef> + Clone + Debug,
    TEntity::TEntityId: Clone,
    TEntity::TStateId: From<OutputRef> + Copy,
    TRepo: EntityRepo<TEntity>,
{
    async fn try_handle(&mut self, ev: LedgerTxEvent) -> Option<LedgerTxEvent> {
        let res = match ev {
            LedgerTxEvent::TxApplied(tx) => {
                let transitions = extract_transitions(Arc::clone(&self.entities), tx.clone()).await;
                let num_transitions = transitions.len();
                let is_success = num_transitions > 0;
                for tr in transitions {
                    let _ = self.topic.feed(Confirmed(StateUpdate::Transition(tr))).await;
                }
                if is_success {
                    trace!(target: "offchain_lm", "[{}] entities parsed from applied tx", num_transitions);
                    None
                } else {
                    Some(LedgerTxEvent::TxApplied(tx))
                }
            }
            LedgerTxEvent::TxUnapplied(tx) => {
                let transitions = extract_transitions(Arc::clone(&self.entities), tx.clone()).await;
                let num_transitions = transitions.len();
                let is_success = num_transitions > 0;
                for tr in transitions {
                    let _ = self
                        .topic
                        .feed(Confirmed(StateUpdate::TransitionRollback(tr.swap())))
                        .await;
                }
                if is_success {
                    trace!(target: "offchain_lm", "[{}] entities parsed from unapplied tx", num_transitions);
                    None
                } else {
                    Some(LedgerTxEvent::TxUnapplied(tx))
                }
            }
        };
        let _ = self.topic.flush().await;
        res
    }
}

pub struct UnconfirmedUpgradeHandler<TSink, TEntity, TRepo>
where
    TEntity: OnChainEntity,
{
    pub topic: TSink,
    pub entities: Arc<Mutex<TRepo>>,
    pub blacklisted_entities: HashSet<TEntity::TEntityId>,
    pub pd: PhantomData<TEntity>,
}

#[async_trait(?Send)]
impl<TSink, TEntity, TRepo> EventHandler<MempoolUpdate<Transaction>>
    for UnconfirmedUpgradeHandler<TSink, TEntity, TRepo>
where
    TSink: Sink<Unconfirmed<StateUpdate<TEntity>>> + Unpin,
    TEntity: OnChainEntity + TryFromLedger<TransactionOutput, OutputRef> + Clone + Debug,
    TEntity::TEntityId: Clone,
    TEntity::TStateId: From<OutputRef> + Copy,
    TRepo: EntityRepo<TEntity>,
{
    async fn try_handle(&mut self, ev: MempoolUpdate<Transaction>) -> Option<MempoolUpdate<Transaction>> {
        let res = match ev {
            MempoolUpdate::TxAccepted(tx) => {
                let transitions = extract_transitions(Arc::clone(&self.entities), tx.clone()).await;
                let is_success = !transitions.is_empty();
                for tr in transitions {
                    let _ = self.topic.feed(Unconfirmed(StateUpdate::Transition(tr))).await;
                }
                if is_success {
                    Some(MempoolUpdate::TxAccepted(tx))
                } else {
                    None
                }
            }
        };
        let _ = self.topic.flush().await;
        res
    }
}
