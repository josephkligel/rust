//! # Cargo_and_Crates
//!
//! `cargo_and_crates` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = cargo_and_crates::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
