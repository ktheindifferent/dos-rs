use core::arch::asm;
use crate::port;

/// Fills the entire screen with the specified color in VGA mode 13h.
///
/// # Arguments
///
/// * `color` - The palette index (0-255) to fill the screen with
pub fn fill_screen(color: u8) {
    unsafe {
        asm!(
            "mov   es, ax",
            "xor   di, di",
            "mov   cx, 320*200/2",
            "mov   al, dl",
            "mov   ah, al",
            "rep   stosw",
            inout("ax") 0xA000 => _,
            in("dl") color,
        )
    }
}

/// Plots a single pixel at the specified coordinates.
///
/// # Arguments
///
/// * `x` - The x-coordinate (0-319 in mode 13h)
/// * `y` - The y-coordinate (0-199 in mode 13h)
/// * `color` - The palette index (0-255) for the pixel color
pub fn plot_pixel(x: u16, y: u16, color: u8) {
    unsafe {
        asm!(
            "int 10h",
            in("ax") (0x0C00u16) | (color as u16),
            in("bh") 0u8,
            in("cx") x,
            in("dx") y,
        );
    }
}

/// Draws a rectangular box outline at the specified position.
///
/// # Arguments
///
/// * `x` - The x-coordinate of the top-left corner
/// * `y` - The y-coordinate of the top-left corner
/// * `w` - The width of the box
/// * `h` - The height of the box
/// * `color` - The palette index (0-255) for the box color
pub fn draw_box(x: u16, y: u16, w: u16, h: u16, color: u8) {
    // Box Left Wall
    for i in x..(x+h) {
        plot_pixel(x, i, color)
    }

    // Box Top Wall
    for i in x..(x+w) {
        plot_pixel(i, y, color)
    }

    // Box Bottom Wall
    for i in x..(x+w) {
        plot_pixel(i, y+h, color)
    }

    // Box Right Wall
    for i in x..(x+h) {
        plot_pixel(x + w, i, color)
    }
}

/// Resets the mouse driver to its default state.
#[allow(dead_code)]
pub fn reset_mouse(){
    unsafe {
        asm!(
            "mov   ax, 0",
            "int 33h",
        );
    }
}

/// Shows the mouse cursor on screen.
pub fn show_mouse(){
    unsafe {
        port::outb(0x3D4, 0x0A);
        port::outb(0x3D5, port::inb(0x3D5) & 0xC0);
    
        port::outb(0x3D4, 0x0B);
        port::outb(0x3D5, (port::inb(0x3D5) & 0xE0) | 15);
    }

    // Alternative implementation using BIOS interrupt 33h (mouse driver)
    // Kept for reference - may be needed for different DOS environments
    // reset_mouse();
    // unsafe {
    //     asm!(
    //         "mov   ax, 1",
    //         "int 33h",
    //     );
    // }
}

/// Hides the mouse cursor from the screen.
#[allow(dead_code)]
pub fn hide_mouse(){
    reset_mouse();
    unsafe {
        asm!(
            "mov   ax, 2",
            "int 33h",
        );
    }
}