//@ compile-flags: -O

#![crate_type = "lib"]

#[no_mangle]
pub fn test() -> usize {
    // CHECK-LABEL: @test(
    // CHECK: ret {{i64|i32}} 165
    let values = [23, 16, 54, 3, 60, 9];
    let mut acc = 0;
    for item in values {
        acc += item;
    }
    acc
}
