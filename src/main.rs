mod lib_surface;

use lib_surface::*;
use sdl2::pixels::Color;
use std::thread::sleep;
use std::time::Duration;


fn main() {
        
    let mut window = create_window("Funky Palmier", 800, 600);    
    clear_window(&mut window, Color::RGB(0,0,0));

    for i in 0..8 {
    for j in 0..6 {
        draw_pixel(&mut window, 50+i*100, 50+j*100, 0.0, Color::RGB(200,200,200));
    }}
    update_window(&mut window);
    sleep(Duration::new(10,0));
}
