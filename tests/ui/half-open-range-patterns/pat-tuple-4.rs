//@ check-pass

#![feature(exclusive_range_pattern)]

fn main() {
    const PAT: u8 = 1;

    match 0 {
        (.. PAT) => {}
        _ => {}
    }
}
