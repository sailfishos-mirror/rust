//@ run-pass
//@ pretty-expanded FIXME #23616

pub fn main() { let x = (); match x { () => { } } }
