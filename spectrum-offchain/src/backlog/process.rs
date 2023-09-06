use std::sync::Arc;

use futures::{Stream, StreamExt};
use log::trace;
use tokio::sync::Mutex;

use crate::backlog::HotBacklog;
use crate::data::order::{OrderLink, OrderUpdate};
use crate::data::SpecializedOrder;
use crate::partitioning::Partitioned;

/// Create backlog stream that drives processing of order events.
pub fn hot_backlog_stream<'a, const N: usize, S, TOrd, TBacklog>(
    backlog: Arc<Partitioned<N, TOrd::TPoolId, Mutex<TBacklog>>>,
    upstream: S,
) -> impl Stream<Item = ()> + 'a
where
    S: Stream<Item = OrderUpdate<TOrd, OrderLink<TOrd>>> + 'a,
    TOrd: SpecializedOrder + 'a,
    TOrd::TOrderId: Clone,
    TBacklog: HotBacklog<TOrd> + 'a,
{
    trace!(target: "offchain", "Watching for Backlog events..");
    upstream.then(move |upd| {
        let backlog = Arc::clone(&backlog);
        async move {
            match upd {
                OrderUpdate::NewOrder(pending_order) => {
                    let backlog_mux = backlog.get(pending_order.get_pool_ref());
                    let mut backlog = backlog_mux.lock().await;
                    backlog.put(pending_order)
                }
                OrderUpdate::OrderEliminated(order_link) => {
                    let backlog_mux = backlog.get(order_link.pool_id);
                    let mut backlog = backlog_mux.lock().await;
                    backlog.remove(order_link.order_id)
                }
            }
        }
    })
}
