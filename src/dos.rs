use core::arch::asm;

/// Prints a null-terminated string using DOS interrupt 21h.
///
/// # Arguments
///
/// * `s` - Pointer to a null-terminated string
pub fn print(s: *const u8) {
    unsafe {
        asm!(
            "int 21h",
            inout("ax") 0x0900 => _,
            in("dx") s,
        );
    }
}

/// Prints a single character to the screen using DOS interrupt 21h.
///
/// # Arguments
///
/// * `c` - The character to print
pub fn print_character(c: u8) {
    unsafe {
        asm!(
            "int 21h",
            inout("ax") 0x0200 => _,
            in("dl") c,
        );
    }
}

/// Gets keyboard input without blocking.
///
/// # Returns
///
/// The scan code of the pressed key, or 0 if no key is pressed
pub fn get_keyboard_input() -> u8 {
    let code;
    unsafe {
        asm!(
            "mov ah, 01h",
            "int 16h",
            "jz 2f",
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

/// Sets the video mode using BIOS interrupt 10h.
///
/// # Arguments
///
/// * `mode` - The video mode to set (e.g., 0x03 for text, 0x13 for VGA graphics)
pub fn set_video_mode(mode: u8) {
    unsafe {
        asm!(
            "int 10h",
            inout("ax") mode as u16 => _,
        );
    }
}

/// Exits the program and returns to DOS.
pub fn exit() {
    unsafe {
        asm!(
            "int 21h",
            inout("ax") 0x4C00 => _,
        );
    }
}

/// Immediately shuts down the computer using APM.
///
/// # Safety
///
/// This function performs a hard system shutdown.
/// Notes: Tested and working (2022)
#[allow(dead_code)]
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
