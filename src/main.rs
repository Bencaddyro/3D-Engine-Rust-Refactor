mod lib_surface;
mod lib_2d;
mod lib_3d;
mod lib_object3d;
mod lib_base_object3d;

use lib_surface::*;
use lib_3d::*;
use lib_object3d::*;
use lib_base_object3d::*;

use sdl2::pixels::Color;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    
    let mut window = create_window("Funky Palmier", 800, 600);
    clear_window(&mut window, Color::RGB(0,0,0));
    
    let h = 700.0;

    let mut o = spheroid(100.0,20,10);
    let p0 = new_point3d(0.0,0.0,800.0);
    
    translation_object3d(&mut o, &p0);
    
    //let c = new_point3d(0.0,0.0,0.0);
    
    for _ in 0..400 {
        clear_window(&mut window, Color::RGB(0,0,0));
        
        //translation_object3d(&mut o, &p);
        rotation_object3d(&mut o, &p0, 1.0,1.0,1.0);

        draw_object3d(&mut window, &mut o, h);

        update_window(&mut window);
        sleep(Duration::new(0,10_000_000));
    }
}

