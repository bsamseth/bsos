#![no_std]
#![no_main]

use core::panic::PanicInfo;

use bsos::println;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, world from a macro{}", "!");
    panic!("Some panic message");

    loop {}
}
