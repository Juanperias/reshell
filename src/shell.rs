use crate::dos::keyboard;
use crate::dos::put::{puts, putc};
use crate::{print, println};

pub fn run_shell() {
    print!(">> ");
    loop {
        let key = keyboard::read();

        if key == 13 {
            putc(b'\r');
            putc(b'\n');
            puts("Running... ");
            putc(b'\r');
            putc(b'\n');

            puts("> ");
        } else {
            putc(key);
        }
    }
}

