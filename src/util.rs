use core::arch::asm;

/// Generates a pseudo-random 16-bit number using a linear feedback shift register.
///
/// # Returns
///
/// A pseudo-random u16 value
pub fn random() -> u16 {
    let value;
    unsafe {
        asm!(
            "mov bx, [11h]",
            "mov ax, bx",
            "shl bx, 7",
            "xor ax, bx",
            "mov bx, ax",
            "shr bx, 9",
            "xor ax, bx",
            "mov bx, ax",
            "shl bx, 8",
            "xor ax, bx",
            "mov [11h], ax",
            out("ax") value,
            out("bx") _,
            out("cx") _,
            out("dx") _,
        );
        value
    }
}

/// Seeds the random number generator using the system timer.
///
/// Uses BIOS interrupt 1Ah to get the current tick count for seeding.
pub fn seed_random() {
    unsafe {
        asm!(
            "int 1ah",
            "mov [11h], dx",
            "xor dx, dx",
            inout("ax") 0 => _,
            inout("cx") 0 => _,
            out("dx") _,
        );
    }
}
