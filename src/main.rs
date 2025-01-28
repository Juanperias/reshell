#![no_std]
#![no_main]

mod dos;
mod fmt;
mod shell;

use core::panic::PanicInfo;

#[no_mangle]
extern "C" fn start() {
    println!("Welcome to the retro shell");
    
    shell::run_shell();
}

#[panic_handler]
fn panic_handler(panic: &PanicInfo) -> ! {
    loop {}
}

