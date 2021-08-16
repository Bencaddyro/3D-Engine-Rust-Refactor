use crate::lib_surface::*;
use std::cmp::min;
use std::cmp::max;
use std::convert::TryFrom;
use sdl2::pixels::Color;

pub struct Point2D(pub i32, pub i32);
pub struct Triangle2D(pub Point2D, pub Point2D, pub Point2D);

fn update_min_max(s: &Screen, x: i32, y: i32, xmin: &mut [usize], xmax: &mut [usize]){
    //println!("update_min_max x={},y={}",x,y);
    if (y >= 0) & (y < s.y as i32) {
        let x = if x < 0 {0} else {x};
        let x = if x >= s.x as i32 {s.x as i32 -1} else {x};
        let y = usize::try_from(y).unwrap();
        let x = usize::try_from(x).unwrap();
        xmin[y] = min(xmin[y],x);
        xmax[y] = max(xmax[y],x);
    }
}

fn bresenham(s: &Screen, m: &Point2D, n: &Point2D, xmin: &mut [usize], xmax: &mut [usize]){
    //Bresenham line implementation cf wikipedia for details
    let dx = n.0 - m.0;
    let dy = n.1 - m.1;
    let Point2D(mut x, mut y) = m;
    let xinc = if dx < 0 {-1} else {1};
    let yinc = if dy < 0 {-1} else {1};
    let dx = dx.abs();
    let dy = dy.abs();
    //println!("dx={}, dy={}, ({},{})-({},{}), xinc={}, yinc={}",dx,dy,m.0,m.1,n.0,n.1,xinc,yinc);
    let mut err;
    update_min_max(s, x, y, xmin, xmax);
    if dx > dy {
        err = dx/2;
        for _ in 0..dx{
            x = x + xinc;
            err = err + dy;
            if err > dx {
                err = err - dx;
                y = y + yinc;
            }
            update_min_max(s, x, y, xmin, xmax);
        }
    } else {
        err = dy/2;
        for _ in 0..dy{
            y = y + yinc;
            err = err + dx;
            if err > dy {
                err = err - dy;
                x = x + xinc;
            }
            update_min_max(s, x, y, xmin, xmax);
        }
    }
}

pub fn fill_triangle2d(s: &mut Screen, triangle: Triangle2D, c: Color){
    // Temporary array to store triangle' shell, maybe to move in futur for memomry optimization
    let mut xmin = vec![s.x-1; s.y];
    let mut xmax = vec![0; s.y];

    let ymin = max(0,min(triangle.0.1,min(triangle.1.1,triangle.2.1))) as usize;
    let ymax = min(s.y as i32,max(triangle.0.1,max(triangle.1.1,triangle.2.1))) as usize;
    //println!("ymin {}, ymax {}", ymin, ymax);

    // First step Bresenham algorithm on triangle's edge to find triangle's shell
    bresenham(s, &triangle.0, &triangle.1, &mut xmin, &mut xmax);
    bresenham(s, &triangle.1, &triangle.2, &mut xmin, &mut xmax);
    bresenham(s, &triangle.2, &triangle.0, &mut xmin, &mut xmax);
    
    //fill shell line by line (y) between min and max x
    for i in ymin..ymax{
        for j in xmin[i]..xmax[i]+1{
            draw_pixel(s, j, i, 0.0, c);
        }
    }
}

