#![allow(dead_code)]

use stub_core::Stub;
use stub_derive::Stub;

#[derive(Stub)]
enum A {
    F1,
    #[stub]
    F2,
    F3(bool),
}

#[derive(Stub)]
enum B {
    F1,
    F2,
    #[stub]
    F3(bool, A),
    F4,
}
