#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
mod vga_buffer;
use core::panic::PanicInfo;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    println!("Welcome to Runix{}", "!");
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}
