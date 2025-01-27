use core::arch::asm;

pub fn exit() {
    unsafe { asm!("mov ah, 4Ch", "mov al, 0", "int 0x21"); }
}
