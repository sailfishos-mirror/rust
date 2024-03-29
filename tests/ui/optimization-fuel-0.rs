//@ run-pass

#![crate_name="foo"]

use std::mem::size_of;

//@ compile-flags: -Z fuel=foo=0

#[allow(dead_code)]
struct S1(u8, u16, u8);
#[allow(dead_code)]
struct S2(u8, u16, u8);

fn main() {
    assert_eq!(size_of::<S1>(), 6);
    assert_eq!(size_of::<S2>(), 6);
}
