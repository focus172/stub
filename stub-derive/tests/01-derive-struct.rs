use stub_core::Stub;
use stub_derive::Stub;

#[derive(Stub)]
struct A {
    f1: bool,
    f2: u8,
}

#[test]
fn can_do_things() {
    let a = A::stub();

    assert_eq!(a.f1, bool::stub());
    assert_eq!(a.f2, u8::stub());
}
