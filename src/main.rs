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
    
    let mut c0 = cuboid(100.0,100.0,100.0);
    let mut c1 = cuboid(100.0,100.0,100.0);
    let mut c2 = cuboid(100.0,100.0,100.0);
    let mut c3 = cuboid(100.0,100.0,100.0);

    //let mut o = cylinder(100.0,200.0,5);
    let p0 = new_point3d(200.0,200.0,800.0);
    let p1 = new_point3d(-200.0,200.0,800.0);
    let p2 = new_point3d(200.0,-200.0,800.0);
    let p3 = new_point3d(-200.0,-200.0,800.0);
    
    translation_object3d(&mut c0, &p0);
    translation_object3d(&mut c1, &p1);
    translation_object3d(&mut c2, &p2);
    translation_object3d(&mut c3, &p3);
    
    let c = new_point3d(0.0,0.0,0.0);
    
    for i in 0..4000 {
        clear_window(&mut window, Color::RGB(0,0,0));
        
        //translation_object3d(&mut o, &p);
        rotation_object3d(&mut c0, &p0, 1.0,0.0,0.0);
        rotation_object3d(&mut c1, &p1, 0.0,1.0,0.0);
        rotation_object3d(&mut c2, &p2, 0.0,0.0,1.0);
        rotation_object3d(&mut c3, &p3, 1.0,1.0,1.0);
        
        draw_object3d(&mut window, &mut c0, h);
        draw_object3d(&mut window, &mut c1, h);
        draw_object3d(&mut window, &mut c2, h);
        draw_object3d(&mut window, &mut c3, h);
        
        update_window(&mut window);
        sleep(Duration::new(0,10_000_000));
    }
}

