use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::Point;

pub struct Screen {
    canvas: Canvas<Window>,
    pub x: usize,
    pub y: usize,
    z_index: Vec<Vec<f64>>,
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
    let z_index = vec![vec![f64::MAX;x];y];
    Screen{canvas, x, y, z_index}
}

pub fn update_window(s: &mut Screen){
    s.canvas.present();
}

pub fn clear_window(s: &mut Screen, c: Color){
    s.canvas.set_draw_color(c);
    s.canvas.clear();
    s.z_index = vec![vec![f64::MAX;s.y];s.x];
}

pub fn draw_pixel(s: &mut Screen, x: usize, y: usize, z: f64, c: Color){
    // (x,y,z) already in correct range with fill_triangle2d
    //println!("draw_pixel x={},y={}",x,y);
    if s.z_index[x][y] < z { return (); } //abort rendering if something ahead
    s.canvas.set_draw_color(c);
    s.z_index[x][y] = z;
    let result = s.canvas.draw_point(Point::new(x as i32, y as i32));
    match result {
        Ok(_o) => (),
        Err(e) => println!("draw_pixel error{:?}", e),
    }
    // TODO handle z buffer
}

