use crate::data::SpecializedOrder;

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub enum OrderUpdate<TNewOrd, TElimOrd> {
    NewOrder(TNewOrd),
    OrderEliminated(TElimOrd),
}

#[derive(Debug, Clone)]
pub struct OrderLink<TOrd: SpecializedOrder> {
    pub order_id: TOrd::TOrderId,
    pub pool_id: TOrd::TPoolId,
}

impl<TOrd: SpecializedOrder> From<TOrd> for OrderLink<TOrd> {
    fn from(o: TOrd) -> Self {
        Self {
            order_id: o.get_self_ref(),
            pool_id: o.get_pool_ref(),
        }
    }
}

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub struct PendingOrder<TOrd> {
    pub order: TOrd,
    pub timestamp: i64,
}

impl<TOrd> From<ProgressingOrder<TOrd>> for PendingOrder<TOrd> {
    fn from(po: ProgressingOrder<TOrd>) -> Self {
        Self {
            order: po.order,
            timestamp: po.timestamp,
        }
    }
}

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub struct SuspendedOrder<TOrd> {
    pub order: TOrd,
    pub timestamp: i64,
}

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub struct ProgressingOrder<TOrd> {
    pub order: TOrd,
    pub timestamp: i64,
}
