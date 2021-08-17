mod lib_surface;
mod lib_2d;
mod lib_3d;

use lib_surface::*;
use lib_3d::*;
use sdl2::pixels::Color;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    
    let mut window = create_window("Funky Palmier", 800, 600);
    clear_window(&mut window, Color::RGB(0,0,0));
    
    let h = 500.0;
    
    let mut red_triangle= Triangle3D([new_point3d(10.0,20.0,600.0),new_point3d(15.0,200.0,600.0),new_point3d(100.0,220.0,600.0)]);
    let mut green_triangle = red_triangle.clone();
    translation_triangle3d(&mut green_triangle, &new_vector3d(-100.0,-50.0,0.0));
    let mut blue_triangle = red_triangle.clone();
    rotation_triangle3d(&mut blue_triangle, &new_point3d(0.0,-50.0,0.0),0.0,0.0,90.0);
    
    for i in 0..1000 {
        clear_window(&mut window, Color::RGB(0,0,0));
        rotation_triangle3d(&mut red_triangle, &new_point3d(10.0,20.0,00.0),1.0,0.0,0.0);
        rotation_triangle3d(&mut green_triangle, &new_point3d(10.0,20.0,600.0),0.0,1.0,0.0);
        rotation_triangle3d(&mut blue_triangle, &new_point3d(10.0,20.0,600.0),0.0,0.0,1.0);
        
        fill_triangle3d(&mut window, &red_triangle, Color::RGB(255,0,0), h);
        fill_triangle3d(&mut window, &green_triangle, Color::RGB(0,255,0), h);
        fill_triangle3d(&mut window, &blue_triangle, Color::RGB(0,0,255), h);
        
        update_window(&mut window);
        sleep(Duration::new(0,10_000_000));
    }
}

