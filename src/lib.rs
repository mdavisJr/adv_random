#![allow(clippy::needless_return)]
//The examples in the README are tested by cargo test.  They need the default random number generator.
#![cfg_attr(feature="rand", doc = include_str!("../README.md"))]

/// Generating numbers, and the results and errors.
pub mod random;
/// The settings for generating numbers, and their builder.
pub mod settings;
/// The rules numbers can have, and the traits for making your own.
pub mod rules;
/// The trait for random number generators.
pub mod random_trait;
/// The default random number generator, from `rand`.
pub mod default_random;
/// A random number generator that gives the same numbers for the same seed.
pub mod seeded_random;
pub mod uniform_check;
pub mod prelude;


#[cfg(test)]
mod tests;
