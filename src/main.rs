#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(runix::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use runix::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Welcome to Runix {}", "#!");

    runix::init();

    // x86_64::instructions::interrupts::int3();
    #[cfg(test)]
    test_main();
    println!("It did not crash!");
    runix::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    runix::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    runix::test_panic_handler(info)
}
