#![no_std]
#![no_main]

mod dos;
mod fmt;

use core::panic::PanicInfo;
use dos::put::puts;

#[no_mangle]
extern "C" fn start() -> ! { 
    puts("Welcome to the retro shell!"); 
    dos::program::exit();

    loop {}
}


#[panic_handler]
fn panic_handler(panic: &PanicInfo) -> ! {
    loop {}
}
