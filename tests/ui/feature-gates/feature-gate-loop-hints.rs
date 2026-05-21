fn main() {
    #[unroll] //~ ERROR the `#[loop_hints]` attribute is an experimental feature
    for _ in 0..10 {}
}
