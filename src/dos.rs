use core::arch::asm;

pub fn print(s: *const u8) {
    unsafe {
        asm!(
            "int 21h",
            inout("ax") 0x0900 => _,
            in("dx") s,
        );
    }
}

pub fn print_character(c: u8) {
    unsafe {
        asm!(
            "int 21h",
            inout("ax") 0x0200 => _,
            in("dl") c,
        );
    }
}

pub fn get_keyboard_input() -> u8 {
    let code;
    unsafe {
        asm!(
            "mov ah, 01h",
            "int 16h",
            "jz 1f",
            "mov ah, 00h",
            "int 16h",
            "mov al, ah",
            "xor ah, ah",
            "jmp 3f",
            "2:",
            "xor ax, ax",
            "3:",
            out("al") code,
        );
    }
    code
}

pub fn set_video_mode(mode: u8) {
    unsafe {
        asm!(
            "int 10h",
            inout("ax") mode as u16 => _,
        );
    }
}

pub fn exit() {
    unsafe {
        asm!(
            "int 21h",
            inout("ax") 0x4C00 => _,
        );
    }
}

// Immediately shuts down the computer
// Notes: Tested and working (2022)
pub fn shutdown() {
    unsafe {
        asm!("mov ax, 0x1000"); 
        asm!("mov ax, ss"); 
        asm!("mov sp, 0xf000"); 
        asm!("mov ax, 0x5307"); 
        asm!("mov bx, 0x0001"); 
        asm!("mov cx, 0x0003"); 
        asm!("int 0x15");
    }
}
