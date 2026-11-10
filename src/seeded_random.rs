#[cfg(feature="rand")]
use rand::prelude::*;
#[cfg(feature="rand")]
use std::sync::{Arc, Mutex};
#[cfg(feature="rand")]
use crate::random_trait::RandomTrait;


#[cfg(feature="rand")]
#[derive(Debug, Clone)]
/// A random number generator that gives the same numbers every time for the same seed, for tests and runs that have to repeat.
pub struct SeededRandom {
    rng: Arc<Mutex<StdRng>>,
}

#[cfg(feature="rand")]
impl SeededRandom {
    /// Starts it with this seed.
    pub fn new(seed: u64) -> SeededRandom {
        return SeededRandom { rng: Arc::new(Mutex::new(StdRng::seed_from_u64(seed))) };
    }
}

#[cfg(feature="rand")]
impl RandomTrait for SeededRandom {
    fn get_number(&self, min: usize, max: usize) -> usize {
        return self.rng.lock().unwrap().random_range(min..=max);
    }

    fn get_bool(&self) -> bool {
        return self.rng.lock().unwrap().random();
    }
}
