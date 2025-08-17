use crate::video;
use crate::util;

pub fn test_boxes() {
    // Test 1: Small 1x1 box (single pixel)
    video::draw_box(10, 10, 1, 1, 15);
    
    // Test 2: 0x0 box (degenerate case - should draw a single point)
    video::draw_box(20, 20, 0, 0, 14);
    
    // Test 3: Horizontal line (height = 0)
    video::draw_box(30, 30, 50, 0, 13);
    
    // Test 4: Vertical line (width = 0)
    video::draw_box(100, 30, 0, 50, 12);
    
    // Test 5: Regular box
    video::draw_box(150, 50, 30, 40, 11);
    
    // Test 6: Large box
    video::draw_box(50, 100, 100, 80, 10);
    
    // Test 7: Box at screen edge
    video::draw_box(280, 150, 39, 49, 9);
    
    // Test 8: Multiple nested boxes
    for i in 0..5 {
        let offset = i * 10;
        let color = 5 + i as u8;
        video::draw_box(200 + offset, 10 + offset, 80 - (offset * 2), 60 - (offset * 2), color);
    }
    
    // Test 9: Random boxes to stress test
    for _ in 0..10 {
        let x = (util::random() as u16) % 300;
        let y = (util::random() as u16) % 180;
        let w = (util::random() as u16) % 20;
        let h = (util::random() as u16) % 20;
        let color = (util::random() as u8) % 255;
        video::draw_box(x, y, w, h, color);
    }
    
    // Test 10: Full screen border (edge case for overflow)
    video::draw_box(0, 0, 319, 199, 15);
}