use std::{fmt::Display, hash::Hash};

use spectrum_offchain::data::{EntitySnapshot, Stable};

pub mod offchain;
pub mod onchain;

pub struct Snapshot<T, V>(T, V);
impl<T, V> Snapshot<T, V> {
    pub fn new(t: T, v: V) -> Self {
        Self(t, v)
    }

    pub fn get(&self) -> &T {
        &self.0
    }

    pub fn unwrap(self) -> T {
        self.0
    }

    pub fn version(&self) -> &V {
        &self.1
    }
}

impl<T, V> Stable for Snapshot<T, V>
where
    T: Stable,
{
    type StableId = T::StableId;

    fn stable_id(&self) -> Self::StableId {
        self.0.stable_id()
    }
}

impl<T, V> EntitySnapshot for Snapshot<T, V>
where
    T: Stable,
    V: Display + Hash + Eq + Copy,
{
    type Version = V;

    fn version(&self) -> Self::Version {
        self.1
    }
}
