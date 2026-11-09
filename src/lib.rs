#![allow(clippy::needless_return)]
//The examples in the README are tested by cargo test.  They need the default random number generator.
#![cfg_attr(feature="rand", doc = include_str!("../README.md"))]

pub mod random;
pub mod settings;
pub mod rules;
pub mod random_trait;
pub mod default_random;
pub mod seeded_random;
pub mod uniform_check;
pub mod prelude;


#[cfg(test)]
mod tests;
