#![no_std]
#![no_main]

mod dos;
mod fmt;

use core::panic::PanicInfo;

#[no_mangle]
extern "C" fn start() -> ! {
    println!("Welcome to the retro shell");
    println!("it works");

    loop {
        if crate::dos::keyboard::read() as char == 'c' {
            dos::program::exit();
        }
    }
}


#[panic_handler]
fn panic_handler(panic: &PanicInfo) -> ! {
    loop {}
}
