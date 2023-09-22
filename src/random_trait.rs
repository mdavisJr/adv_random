use once_cell::sync::OnceCell;

#[cfg(feature="rand")]
use crate::default_random::DefaultRandom;

static RANDOM_TRAIT: OnceCell<Box<dyn RandomTrait + Send + Sync>> = OnceCell::new();

#[cfg(feature="rand")]
pub fn get_random_trait() -> &'static Box<dyn RandomTrait + Send + Sync> {
    let value: &Box<dyn RandomTrait + Send + Sync> = RANDOM_TRAIT.get_or_init(|| {
        Box::new(DefaultRandom{})
    });
    return value;
}

#[cfg(not(feature="rand"))]
pub fn get_random_trait() -> &'static Box<dyn RandomTrait + Send + Sync> {
    let value: &Box<dyn RandomTrait + Send + Sync> = match RANDOM_TRAIT.get() {
        Some(v) => v,
        None => panic!("Please call set_random_trait function or use feature \"rand\""),
    };
    return value;
}

#[cfg(not(feature="rand"))]
pub fn set_random_trait(random_trait: Box<dyn RandomTrait + Send + Sync>) {
    let _ = RANDOM_TRAIT.set(random_trait);
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