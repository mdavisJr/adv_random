#[cfg(feature = "rand")]
use crate::random_trait::RandomTrait;
#[cfg(feature = "rand")]
use rand::prelude::*;

#[cfg(feature = "rand")]
#[derive(Debug, Clone, Default)]
/// The random number generator from `rand`, which settings use when they are given no other one.  Needs the `rand` feature.
pub struct DefaultRandom {}

#[cfg(feature = "rand")]
impl DefaultRandom {
    /// A new one.
    pub fn new() -> DefaultRandom {
        return DefaultRandom {};
    }
}

#[cfg(feature = "rand")]
impl RandomTrait for DefaultRandom {
    fn get_number(&self, min: usize, max: usize) -> usize {
        return rand::rng().random_range(min..=max);
    }

    fn get_bool(&self) -> bool {
        return rand::rng().random();
    }
}
