//Returns None when vec is empty
pub fn get_random_vec_item<'a, T>(random: &dyn RandomTrait, vec: &'a [T]) -> Option<&'a T> {
    if vec.is_empty() {
        return None;
    }
    return Some(&vec[random.get_number(0, vec.len() - 1)]);
}

pub fn shuffle_vec<T>(random: &dyn RandomTrait, vector: &mut [T])
{
    if vector.is_empty() {
        return;
    }
    let len = vector.len() - 1;
    for i in 0..=len {
        vector.swap(i, random.get_number(i, len));
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