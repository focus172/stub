pub use stub_core::Stub;
pub use stub_derive::Stub;

/// Shorthand for alluding feilds.
/// ```rust
/// use stub::*;
///
/// #[derive(Stub)]
/// struct A {
///     field: u8,
///     other: String
/// }
///
/// let a = A {
///     field: 4,
///     ..stub()
/// };
/// ```
pub fn stub<T: Stub>() -> T {
    T::stub()
}
