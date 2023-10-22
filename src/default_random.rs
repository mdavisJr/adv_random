#[cfg(feature="rand")]
use rand::{thread_rng, Rng};
#[cfg(feature="rand")]
use rand::distributions::{Distribution, Uniform};
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
        let mut rng = thread_rng();
        let range = Uniform::from(min..=max);
        return range.sample(&mut rng);
    }

    fn get_bool(&self) -> bool {
        let mut rng = thread_rng();
        return rng.gen();
    }
}