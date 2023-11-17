use crate::Stub;

impl<T: Stub> Stub for Option<T> {
    fn stub() -> Self {
        Option::Some(T::stub())
    }
}

impl<T: Stub> Stub for Box<T> {
    fn stub() -> Self {
        std::boxed::Box::new(T::stub())
    }
}

impl<T> Stub for Box<[T]> {
    fn stub() -> Self {
        std::boxed::Box::new([])
    }
}
