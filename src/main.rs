mod lib_surface;
mod lib_2d;

use lib_surface::*;
use lib_2d::*;
use sdl2::pixels::Color;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    
    let mut window = create_window("Funky Palmier", 800, 600);
    clear_window(&mut window, Color::RGB(0,0,0));
    
    let red_triangle = Triangle2D(Point2D(10,20),Point2D(11,200),Point2D(100,100));
    fill_triangle2d(&mut window, red_triangle, Color::RGB(255,0,0));
    
    let green_triangle = Triangle2D(Point2D(203,103),Point2D(750,150),Point2D(174,129));
    fill_triangle2d(&mut window, green_triangle, Color::RGB(0,255,0));
    
    let blue_triangle = Triangle2D(Point2D(178,361),Point2D(142,417),Point2D(133,380));
    fill_triangle2d(&mut window, blue_triangle, Color::RGB(0,0,255));
    
    let white_triangle = Triangle2D(Point2D(500,300),Point2D(300,500),Point2D(700,500));
    fill_triangle2d(&mut window, white_triangle, Color::RGB(255,255,255));
    
    let black_triangle = Triangle2D(Point2D(500,302),Point2D(304,498),Point2D(696,498));
    fill_triangle2d(&mut window, black_triangle, Color::RGB(0,0,0));
    
    update_window(&mut window);
    sleep(Duration::new(10,0));
}

