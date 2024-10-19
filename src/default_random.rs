#[cfg(feature="rand")]
use rand::{thread_rng, Rng};
#[cfg(feature="rand")]
#[cfg(feature="rand")]
use crate::random_trait::RandomTrait;


#[cfg(feature="rand")]
#[derive(Debug, Clone)]
pub struct DefaultRandom {}

#[cfg(feature="rand")]
impl DefaultRandom {
    pub fn new() -> Box<dyn RandomTrait> {
        return Box::new(DefaultRandom {  });
    }
}

#[cfg(feature="rand")]
impl RandomTrait for DefaultRandom {
    fn get_number(&self, min: usize, max: usize) -> usize {
        return thread_rng().gen_range(min..=max); 
    }

    fn get_bool(&self) -> bool {
        return thread_rng().gen();
    }
}