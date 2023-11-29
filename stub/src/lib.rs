#![doc = include_str!("../../README.md")]
//!
//! # Purpose
//! Stub is very similar to the [`Default`] trait except that it is implimented
//! for many types that [`Default`] is not. This is beacuse it would not make
//! logic sence for their to exist a default, however, there does exist a logic
//! stub for most types.
//!
//! This is useful for builder patters where you want to be able to get a valid
//! types so you can start modifying its feilds. It is recomened that you
//! only use this in test cases as often times the way the trait is implimented
//! on some obscure types can have side effects on your binary size.
//!

pub use stub_core::Stub;
// #[cfg(feature = "derive")]
pub use stub_derive::Stub;

/// Shorthand for alluding feilds.
/// ```rust
/// use stub::*;
///
/// #[derive(Stub)]
/// struct Person {
///     name: String,
///     age: u8,
///     money: u8,
/// }
///
/// # fn main() {
/// let character = Person {
///     name: String::from("Joe"),
///     ..stub()
/// };
/// # }
/// ```
///
/// # Examples
///
/// ```
/// use stub::stub;
///
/// assert_eq!(stub::<usize>(), 0);
/// ```
pub fn stub<T: Stub>() -> T {
    T::stub()
}
