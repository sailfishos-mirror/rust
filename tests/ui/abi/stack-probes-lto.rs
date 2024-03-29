//@ revisions: aarch64 x32 x64
//@ run-pass
//@[aarch64] only-aarch64
//@[aarch64] min-llvm-version: 18
//@[x32] only-x86
//@[x64] only-x86_64
//@ ignore-sgx no processes
//@ ignore-musl FIXME #31506
//@ ignore-fuchsia no exception handler registered for segfault
//@ compile-flags: -C lto
//@ no-prefer-dynamic
//@ ignore-nto Crash analysis impossible at SIGSEGV in QNX Neutrino

include!("stack-probes.rs");
