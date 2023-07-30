use once_cell::sync::OnceCell;

use crate::default_random::DefaultRandom;

static RANDOM_TRAIT: OnceCell<Box<dyn RandomTrait + Send + Sync>> = OnceCell::new();

pub fn get_random_trait() -> &'static Box<dyn RandomTrait + Send + Sync> {
    let value: &Box<dyn RandomTrait + Send + Sync> = RANDOM_TRAIT.get_or_init(|| {
        Box::new(DefaultRandom{})
    });
    return value;
}

pub fn get_random_vec_item<T>(vec: &[T]) -> &T {
    return &vec[get_random_trait().get_number(0, vec.len() - 1)];
}

// pub fn shuffle<T>(list: &mut [T]) {
//     list.shuffle(&mut thread_rng());
// }

pub fn shuffle_vec<T>(vector: &mut [T])
{
    let len = vector.len() - 1;
    for i in 0..=len {
        vector.swap(i, get_random_trait().get_number(i, len));
    }
}

pub fn set_random_trait(random_trait: Option<Box<dyn RandomTrait + Send + Sync>>) {
    if let Err(_) = RANDOM_TRAIT.set(
        match random_trait {
            Some(v) => v,
            None => Box::new(DefaultRandom{})
        }
    ) {
        panic!("already set");
    }
}


pub trait RandomTrait: RandomTraitClone + std::fmt::Debug {
    fn get_number(&self, min: usize, max: usize) -> usize;
    fn get_bool(&self) -> bool;
}

pub trait RandomTraitClone {
    fn clone_box(&self) -> Box<dyn RandomTrait>;
}

impl<T> RandomTraitClone for T
where
    T: 'static + RandomTrait + Clone,
{
    fn clone_box(&self) -> Box<dyn RandomTrait> {
        Box::new(self.clone())
    }
}


impl Clone for Box<dyn RandomTrait> {
    fn clone(&self) -> Box<dyn RandomTrait> {
        self.clone_box()
    }
}