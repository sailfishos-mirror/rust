// Test that niche finding works with captured coroutine upvars.

//@ run-pass

#![feature(coroutines)]

use std::mem::size_of_val;

fn take<T>(_: T) {}

fn main() {
    let x = false;
    let gen1 = || {
        yield;
        take(x);
    };

    assert_eq!(size_of_val(&gen1), size_of_val(&Some(gen1)));
}
