// Dummy Rust file to satisfy Cargo's requirement for a Rust source file.

// We use C/C++'s main function.
#![no_main]

// We use C/C++'s standard library.
#![no_std]

// Panic handler for no_std environment to make cargo happy.
// Will be never called since we don't have any Rust code.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}