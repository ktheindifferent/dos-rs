#![feature(proc_macro_hygiene)]
#![no_main]
#![no_std]

mod dos;
mod panic;
mod text;
mod io;
mod port;
mod opn;
mod util;
mod video;

/// Entry point for the DOS executable.
///
/// # Safety
///
/// This function is unsafe because it:
/// - Is called directly by the DOS loader with undefined initial state
/// - Performs direct hardware manipulation through inline assembly
/// - Assumes a DOS environment with appropriate interrupt handlers
#[no_mangle]
pub unsafe extern "C" fn start() {
    util::seed_random();
    dos::set_video_mode(0x13);

    let bg_color: u8 = util::random() as u8 % 255;
    let color: u8 = util::random() as u8 % 255;

    video::fill_screen(bg_color);
    video::draw_box(0, 0, 319, 10, color);
    video::show_mouse();
    
    // Test code for random pixel plotting - kept for debugging graphics routines
    // Uncomment to test pixel plotting performance and random number generation
    // for i in 0..10000 {
    //     let x: u16 = util::random() as u16 % 320;
    //     let y: u16 = util::random() as u16 % 200;
    //     let color: u8 = util::random() as u8 % 255;

    //     video::plot_pixel(x, y, color);
    // }

    loop {
        let code = dos::get_keyboard_input();
        if code != 0 { break; }
    }

    dos::set_video_mode(0x03);

    print!("Thanks for trying Rusty DOS! Nöw with CP437 support for languagés!");
}
