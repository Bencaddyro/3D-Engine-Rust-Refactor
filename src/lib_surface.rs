use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::Point;

pub struct Screen {
    canvas: Canvas<Window>,
    pub x: usize,
    pub y: usize,
    //z: f64,// tab de z index
}

pub fn create_window(name: &str, x:usize, y:usize) -> Screen {
    //copy past from Rust-SDL2 demo code 
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window(name, x as u32, y as u32)
        .position_centered()
        .build()
        .unwrap();
    let canvas = window.into_canvas().build().unwrap();
    Screen{canvas, x, y}
}

pub fn update_window(s: &mut Screen){
    s.canvas.present();
}

pub fn clear_window(s: &mut Screen, c: Color){
    s.canvas.set_draw_color(c);
    s.canvas.clear();
    //need reset for (soon)TM z-index
}

pub fn draw_pixel(s: &mut Screen, x: usize, y: usize, _z: f64, c: Color){
    // (x,y) already in correct range with fill_triangle2d
    //println!("draw_pixel x={},y={}",x,y);
    s.canvas.set_draw_color(c);
    let result = s.canvas.draw_point(Point::new(x as i32, y as i32));
    match result {
        Ok(_o) => (),
        Err(e) => println!("draw_pixel error{:?}", e),
    }
    // TODO handle z buffer
}

