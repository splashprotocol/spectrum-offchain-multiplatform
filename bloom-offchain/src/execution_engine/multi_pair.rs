use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;

use crate::maker::Maker;

#[derive(Debug, Clone)]
pub struct MultiPair<PairId, R, Ctx>(HashMap<PairId, R>, Ctx);

impl<PairId, R, Ctx> MultiPair<PairId, R, Ctx> {
    pub fn new(context: Ctx, resource_type_hint: PhantomData<R>) -> Self {
        Self(HashMap::new(), context)
    }
}

impl<PairId, R, Ctx> MultiPair<PairId, R, Ctx>
where
    PairId: Copy + Eq + Hash,
    R: Maker<Ctx>,
    Ctx: Clone,
{
    pub fn with_resource_mut<F, T>(&mut self, pair: &PairId, f: F) -> T
    where
        F: FnOnce(&mut R) -> T,
    {
        f(self.get_mut(pair))
    }

    pub fn get_mut(&mut self, pair: &PairId) -> &mut R {
        if self.0.contains_key(pair) {
            self.0.get_mut(pair).unwrap()
        } else {
            self.0.insert(*pair, Maker::make(&self.1));
            self.get_mut(pair)
        }
    }
}