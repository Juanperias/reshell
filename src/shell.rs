use crate::dos::keyboard;
use crate::dos::put::putc;
use crate::print;

pub fn run_shell() {
    print!("> ");
    loop {
        let key = keyboard::read();

        if key == 13 {
            putc(b'\r');
            putc(b'\n');
            print!("> ");
        } else {
            putc(key);
        }
    }
}

