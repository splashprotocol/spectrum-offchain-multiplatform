pub enum ExecutionEff<T, K> {
    Updated(T),
    Eliminated(K),
}

impl<T, K> ExecutionEff<T, K> {
    pub fn bimap<T2, K2, FT, FK>(self, ft: FT, fk: FK) -> ExecutionEff<T2, K2>
    where
        FT: FnOnce(T) -> T2,
        FK: FnOnce(K) -> K2,
    {
        match self {
            ExecutionEff::Updated(u) => ExecutionEff::Updated(ft(u)),
            ExecutionEff::Eliminated(e) => ExecutionEff::Eliminated(fk(e)),
        }
    }

    pub fn map<T2, F>(self, f: F) -> ExecutionEff<T2, K>
    where
        F: FnOnce(T) -> T2,
    {
        match self {
            ExecutionEff::Updated(u) => ExecutionEff::Updated(f(u)),
            ExecutionEff::Eliminated(e) => ExecutionEff::Eliminated(e),
        }
    }

    pub fn map_eliminated<K2, F>(self, f: F) -> ExecutionEff<T, K2>
    where
        F: FnOnce(K) -> K2,
    {
        match self {
            ExecutionEff::Eliminated(e) => ExecutionEff::Eliminated(f(e)),
            ExecutionEff::Updated(u) => ExecutionEff::Updated(u),
        }
    }
}