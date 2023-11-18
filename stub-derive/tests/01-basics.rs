use stub_core::Stub;
use stub_derive::Stub;

#[derive(Debug, Stub, PartialEq)]
struct A {
    f1: bool,
    f2: u8,
}

#[derive(Debug, Stub, PartialEq)]
struct B {
    f1: A,
    f2: Option<&'static str>,
}

#[test]
fn feilds_are_derived() {
    let a = A::stub();
    let b = B::stub();

    assert_eq!(a.f1, bool::stub());
    assert_eq!(a.f2, u8::stub());

    assert_eq!(b.f1, A::stub());
    let b2 = B {
        f1: A::stub(),
        f2: Option::stub(),
    };

    assert_eq!(b2, b);
}
