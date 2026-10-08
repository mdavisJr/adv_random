#[cfg(feature="rand")]
use rand::prelude::*;
#[cfg(feature="rand")]
use std::sync::{Arc, Mutex};
#[cfg(feature="rand")]
use crate::random_trait::RandomTrait;


#[cfg(feature="rand")]
#[derive(Debug, Clone)]
pub struct SeededRandom {
    rng: Arc<Mutex<StdRng>>,
}

#[cfg(feature="rand")]
impl SeededRandom {
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
