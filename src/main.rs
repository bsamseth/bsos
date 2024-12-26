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
    println!("Hello World{}", "!");

    bsos::init();

    x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use bsos::hlt_loop;

    println!("{}", info);
    hlt_loop()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    bsos::test_panic_handler(info)
}
