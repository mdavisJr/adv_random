#![allow(clippy::needless_return)]
//Every public item has documentation on docs.rs, and CI fails on this warning
#![warn(missing_docs)]
//The examples in the README are tested by cargo test.  They need the default random number generator.
#![cfg_attr(feature="rand", doc = include_str!("../README.md"))]
//Without rand, the crate says what it is, without the README's examples
#![cfg_attr(
    not(feature = "rand"),
    doc = "Create random numbers, passwords, or strings based on rules.  The examples in the README need the default `rand` feature."
)]

/// The default random number generator, from `rand`.
pub mod default_random;
pub mod prelude;
/// Generating numbers, and the results and errors.
pub mod random;
/// The trait for random number generators.
pub mod random_trait;
/// The rules numbers can have, and the traits for making your own.
pub mod rules;
/// A random number generator that gives the same numbers for the same seed.
pub mod seeded_random;
/// The settings for generating numbers, and their builder.
pub mod settings;
pub mod uniform_check;

#[cfg(test)]
mod tests;
