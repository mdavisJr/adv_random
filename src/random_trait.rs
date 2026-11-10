/// Returns None when vec is empty
pub fn get_random_vec_item<'a, T>(random: &dyn RandomTrait, vec: &'a [T]) -> Option<&'a T> {
    if vec.is_empty() {
        return None;
    }
    return Some(&vec[random.get_number(0, vec.len() - 1)]);
}

/// Shuffles the items with the random number generator.
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


/// A random number generator.  Implement it to use your own: `DefaultRandom` and `SeededRandom` do.
pub trait RandomTrait: std::fmt::Debug {
    /// A random number from min to max, both included.
    fn get_number(&self, min: usize, max: usize) -> usize;
    /// A random true or false.
    fn get_bool(&self) -> bool;
}
