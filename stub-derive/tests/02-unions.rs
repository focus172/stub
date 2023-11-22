use stub_core::Stub;
use stub_derive::Stub;

#[derive(Stub)]
struct A(&'static str, usize);
