use crate::Stub;

impl<T: Stub, E> Stub for Result<T, E> {
    fn stub() -> Self {
        Result::Ok(T::stub())
    }
}
