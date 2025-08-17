#![feature(proc_macro_hygiene)]
#![no_main]
#![no_std]

mod dos;
mod panic;
mod text;
mod io;
mod port;
mod opn;
mod rng;
mod util;
mod video;
mod test_boxes;

#[no_mangle]
pub unsafe extern "C" fn start() {
    util::seed_random();
    dos::set_video_mode(0x13);

    // Clear screen with a background color
    video::fill_screen(0);
    
    // Run comprehensive box drawing tests
    test_boxes::test_boxes();
    
    video::show_mouse();
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
