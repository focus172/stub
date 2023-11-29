mod fns;
mod irg;
mod std;
mod tup;
mod wrp;

/// The minimum valid type for a given type.
pub trait Stub {
    fn stub() -> Self;
}
