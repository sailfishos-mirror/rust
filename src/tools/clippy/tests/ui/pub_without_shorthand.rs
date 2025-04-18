
//@aux-build:proc_macros.rs
#![feature(custom_inner_attributes)]
#![allow(clippy::needless_pub_self, unused)]
#![warn(clippy::pub_without_shorthand)]
#![no_main]
#![rustfmt::skip] // rustfmt will remove `in`, understandable
                  // but very annoying for our purposes!

#[macro_use]
extern crate proc_macros;

pub(self) fn a() {}
pub(in self) fn b() {}
//~^ pub_without_shorthand

pub fn c() {}
mod a {
    pub(in super) fn d() {}
    //~^ pub_without_shorthand
    pub(super) fn e() {}
    pub(self) fn f() {}
    pub(crate) fn k() {}
    pub(in crate) fn m() {}
    //~^ pub_without_shorthand
    mod b {
        pub(in crate::a) fn l() {}
    }
}

external! {
    pub(self) fn g() {}
    pub(in self) fn h() {}
}
with_span! {
    span
    pub(self) fn i() {}
    pub(in self) fn j() {}
}

// not really anything more to test. just a really simple lint overall
