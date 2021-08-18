use crate::lib_surface::*;
use crate::lib_3d::*;
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
    //println!("({},{})-({},{})",m.0,m.1,n.0,n.1);
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

fn circumvent<T: Ord>(min: T, max: T, val: T)-> T{
    if val < min {return min};
    if val > max {return max};
    val
}

pub fn fill_triangle2d(s: &mut Screen, triangle: Triangle2D, plan: Plan3D, h: f64, color: Color){
    // Temporary array to store triangle' shell, maybe to move in futur for memory optimization
    let mut xmin = vec![s.x-1; s.y];
    let mut xmax = vec![0; s.y];

    let ymin = min(triangle.0.1,min(triangle.1.1,triangle.2.1));
    let ymax = max(triangle.0.1,max(triangle.1.1,triangle.2.1));
    //print!("ymin {}, ymax {} => ", ymin, ymax);
    if ymin > s.y as i32 || ymax < 0 { return () } //abort rendering as triangle is not on screen
    let ymin = circumvent(0, (s.y-1) as i32, ymin) as usize;
    let ymax = circumvent(0, (s.y-1) as i32, ymax) as usize;
    //print!("ymin {}, ymax {}\n", ymin, ymax);
    // First step Bresenham algorithm on triangle's edge to find triangle's shell
    bresenham(s, &triangle.0, &triangle.1, &mut xmin, &mut xmax);
    bresenham(s, &triangle.1, &triangle.2, &mut xmin, &mut xmax);
    bresenham(s, &triangle.2, &triangle.0, &mut xmin, &mut xmax);
    
    let Plan3D{a, b, c, d} = plan;
    let xsize = s.x as f64 / 2.0;
    let ysize = s.y as f64 / 2.0;    
    
    
    //fill shell line by line (y) between min and max x
    for i in ymin..ymax{
        for j in xmin[i]..xmax[i]{
        
            let z = d / ( c + a * (j as f64-xsize) / h + b * (i as f64-ysize) / h );
            draw_pixel(s, j, i, z, color);
        }
    }
}




