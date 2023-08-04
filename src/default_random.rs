use rand::{thread_rng, Rng};
use rand::distributions::{Distribution, Uniform};
use crate::random_trait::RandomTrait;



#[derive(Debug, Clone)]
pub struct DefaultRandom {}

impl DefaultRandom {
    pub fn new() -> Box<dyn RandomTrait> {
        return Box::new(DefaultRandom {  });
    }
}

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