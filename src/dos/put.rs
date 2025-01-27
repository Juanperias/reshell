use core::arch::asm;
use core::fmt::Arguments;

pub fn putc(char: u8) {
    unsafe {
        asm!(
            "mov dl, {}",
            "mov ah, 02h",
            "int 0x21",
            in(reg_byte) char
        );
    }
}

pub fn puts(text: &str) {
    putc(b' ');
    for letter in text.bytes() {
        putc(letter);
    }
}

pub fn put_args(args: Arguments) {
    let s = args.as_str().unwrap();

    puts(s);
}


