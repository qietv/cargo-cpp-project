// Dummy Rust file to satisfy Cargo's requirement for a Rust source file.

// Use the C/C++ main function instead of a Rust-generated entry point.
#![no_main]

// Do not link Rust's standard library.
#![no_std]

// Required by no_std; no Rust application code should reach this handler.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}