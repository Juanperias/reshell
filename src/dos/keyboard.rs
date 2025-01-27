use core::arch::asm;

pub fn read() -> u8 {
    let mut al = 0;

    unsafe {
        asm!("mov ah, 0x07", "int 0x21", out("al") al)
    }

    al
}

pub fn read_print() -> u8 {
    let mut al = 0;

    unsafe {
        asm!("mov ah, 01H", "int 0x21", out("al") al)
    }

    al

}
