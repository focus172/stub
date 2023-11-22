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

impl<T> Stub for Vec<T> {
    fn stub() -> Self {
        Vec::new()
    }
}

impl<T> Stub for std::collections::HashSet<T> {
    fn stub() -> Self {
        std::collections::HashSet::new()
    }
}
