use core::arch::asm;
use crate::port;

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

pub fn plot_pixel(x: u16, y: u16, color: u8) {
    unsafe {
        asm!(
            "xor bx, bx",  // Clear BX register (BH = 0 for page 0)
            "int 10h",
            in("ax") (0x0C00u16) | (color as u16),
            in("cx") x,
            in("dx") y,
            options(nostack),
        );
    }
}

pub fn draw_box(x: u16, y: u16, w: u16, h: u16, color: u8) {
    // Parameter validation to prevent overflow
    let max_x = x.saturating_add(w);
    let max_y = y.saturating_add(h);
    
    // Left wall: x constant, y varies
    for i in y..=max_y {
        plot_pixel(x, i, color);
    }
    
    // Top wall: y constant, x varies
    for i in x..=max_x {
        plot_pixel(i, y, color);
    }
    
    // Right wall: x+w constant, y varies
    for i in y..=max_y {
        plot_pixel(max_x, i, color);
    }
    
    // Bottom wall: y+h constant, x varies
    for i in x..=max_x {
        plot_pixel(i, max_y, color);
    }
}

pub fn reset_mouse(){
    unsafe {
        asm!(
            "mov   ax, 0",
            "int 33h",
        );
    }
}

pub fn show_mouse(){
    unsafe {
        port::outb(0x3D4, 0x0A);
        port::outb(0x3D5, (port::inb(0x3D5) & 0xC0) | 0);
    
        port::outb(0x3D4, 0x0B);
        port::outb(0x3D5, (port::inb(0x3D5) & 0xE0) | 15);
    }


    // reset_mouse();
    // unsafe {
    //     asm!(
    //         "mov   ax, 1",
    //         "int 33h",
    //     );
    // }
}

pub fn hide_mouse(){
    reset_mouse();
    unsafe {
        asm!(
            "mov   ax, 2",
            "int 33h",
        );
    }
}