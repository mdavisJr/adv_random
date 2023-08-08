mod random;
mod settings;
mod default_random;
pub mod rules;

pub use self::random::{random_numbers, random_string};
pub use self::settings::Settings;
pub use self::rand_trait::{RandomTrait, get_random_trait, set_random_trait, get_random_vec_item, shuffle_vec};
pub use self::default_random::DefaultRandom;
