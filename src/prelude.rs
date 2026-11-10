//! Everything most programs need, in one line:  use adv_random::prelude::*;

pub use crate::random::{CurrentData, Log, RandomError, RandomErrorType, RandomNumbers};
pub use crate::random_trait::RandomTrait;
pub use crate::rules::*;
pub use crate::settings::{Settings, SettingsBuilder};

#[cfg(feature = "rand")]
pub use crate::default_random::DefaultRandom;
#[cfg(feature = "rand")]
pub use crate::seeded_random::SeededRandom;
