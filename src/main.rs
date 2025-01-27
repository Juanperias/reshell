#![no_std]
#![no_main]

mod dos;
mod fmt;

use core::panic::PanicInfo;

#[no_mangle]
extern "C" fn start() -> ! {
    println!("Welcome to the retro shell");
    print!(">");

    loop {
        let key = dos::keyboard::read();
        if key != 0 {
            if key == b'\r' { 
                dos::program::exit()
            }
            dos::put::putc(key); 
        }
    }
}

#[panic_handler]
fn panic_handler(panic: &PanicInfo) -> ! {
    loop {}
}

