#![no_std]
#![no_main]

mod dos;
mod fmt;

use core::panic::PanicInfo;

#[no_mangle]
extern "C" fn start() -> ! {
    println!("Welcome to the retro shell");
    println!("it works");    

    dos::program::exit();

    loop {}
}


#[panic_handler]
fn panic_handler(panic: &PanicInfo) -> ! {
    loop {}
}
