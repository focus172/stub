//! The minimum valid type for a given type.
//!
//! Similar to the [`Default`] trait except that it is implimented for many
//! types that [`Default`] is not. This is beacuse it would not make logic sence
//! for their to exist a default, however, there does exist a logic stub for
//! most types.
//!
//! This is useful for builder patters where you want to be able to get a valid
//! types so you can start modifying its feilds. It is recomened that you
//! only use this in test cases as often times the way the trait is implimented
//! on some obscure types can have side effects on your binary size.
//!

mod fns;
mod irg;
mod std;
mod tup;
mod wrp;

/// The minimum valid type for a given type.
pub trait Stub {
    fn stub() -> Self;
}

pub fn stub<T: Stub>() -> T {
    T::stub()
}
