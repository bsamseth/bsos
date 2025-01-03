#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(bsos::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(clippy::empty_loop)]

use core::panic::PanicInfo;

use bsos::{hlt_loop, println};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    bsos::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    println!("test_println output");
}
