use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::Point;

pub struct Screen {
    canvas: Canvas<Window>,
    x: u32,
    y: u32,
    //z: f64,// tab de z index
    //maybe xmin, xmax array for triangle filling...
}

pub fn create_window(name: &str, x:u32, y:u32) -> Screen {
    //copy past from Rust-SDL2 demo code 
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window(name, x, y)
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
    //need remise au fond des Z index
}

pub fn draw_pixel(s: &mut Screen, x: u32, y: u32, _z: f64, c: Color){
    // TODO handle 0<=x<xmax & 0<=y<ymax
    s.canvas.set_draw_color(c);
    let result = s.canvas.draw_point(Point::new(x as i32, y as i32));
    match result {
        Ok(_o) => (),
        Err(e) => println!("draw_pixel error{:?}", e),
    }
    // TODO handle z buffer
}

