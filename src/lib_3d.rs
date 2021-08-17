use crate::lib_surface::*;
use crate::lib_2d::*;
use sdl2::pixels::Color;

pub struct Matrix3D([[f64;4];4]);
pub struct Point3D([f64;4]);
pub struct Triangle3D(pub [Point3D;3]);

pub fn new_point3d(x: f64, y: f64, z: f64)->Point3D{
    Point3D([x, y, z, 1.0])
}

pub fn new_vector3d(x: f64, y: f64, z: f64)->Point3D{
    Point3D([x, y, z, 0.0])
}

#[allow(dead_code)]
fn print_triangle3d(t: &Triangle3D){
    println!("Triangle ({},{},{})-({},{},{})-({},{},{})",t.0[0].0[0],t.0[0].0[1],t.0[0].0[2],t.0[1].0[0],t.0[1].0[1],t.0[1].0[2],t.0[2].0[0],t.0[2].0[1],t.0[2].0[2],);
}

fn convert_3d_2d(p3d: &Point3D, h: f64, xsize: usize, ysize: usize)-> Point2D{
    //assuming centre of projection is (0,0,0) and projection plane have (0,0,1) as normal vector, and z = h
    let m = Matrix3D([[h/p3d.0[2], 0.0, 0.0, 0.0],
                      [0.0, h/p3d.0[2], 0.0, 0.0],
                      [0.0, 0.0, h/p3d.0[2], 0.0],
                      [0.0, 0.0, 0.0, 1.0]]);
    let p = multiplication_vector3d(&m, &p3d);
    let x = p.0[0]+xsize as f64/2.0;
    let y = p.0[1]+ysize as f64/2.0;
    Point2D(x as i32, y as i32)
}

pub fn fill_triangle3d(s: &mut Screen, t: &Triangle3D, color: Color, h: f64){
    //TODO check if t is in render space aka in front of camera, otherwise drop render -> z > h
    //print_triangle3d(t);
    if t.0[0].0[2] < 0.000_000_1 || t.0[1].0[2] < 0.000_000_1 || t.0[2].0[2] < 0.000_000_1 { return () } //if any point of t is behind abort rendering, 0.000_000_1 instead of 0 to bypass small float rounding problem
    
    // orthogonal projection on 2d plane
    let a = convert_3d_2d(&t.0[0], h, s.x, s.y);
    let b = convert_3d_2d(&t.0[1], h, s.x, s.y);
    let c = convert_3d_2d(&t.0[2], h, s.x, s.y);
    let t2d = Triangle2D(a,b,c);
    
    //TODO find normal vector to triangle plane, allowing z-index computation
    fill_triangle2d(s, t2d, color);
}

fn transform_triangle3d(t: &Triangle3D, m: &Matrix3D)-> Triangle3D{
    let mut res = Triangle3D([new_point3d(0.0,0.0,0.0),new_point3d(0.0,0.0,0.0),new_point3d(0.0,0.0,0.0)]);
    for i in 0..3 {
        res.0[i] = multiplication_vector3d(&m, &t.0[i]);
    }
    res
}

pub fn translation_triangle3d(t: &Triangle3D, v: &Point3D)-> Triangle3D{
    let m = Matrix3D([[1.0, 0.0, 0.0, v.0[0]],
                      [0.0, 1.0, 0.0, v.0[1]],
                      [0.0, 0.0, 1.0, v.0[2]],
                      [0.0, 0.0, 0.0, 1.0]]);
    transform_triangle3d(t, &m)
}

pub fn rotation_triangle3d(t: &Triangle3D, c: Point3D, x: f64, y: f64, z: f64)-> Triangle3D{
    let x = x.to_radians();
    let y = y.to_radians();
    let z = z.to_radians();
    let mcentre = Matrix3D([[1.0, 0.0, 0.0, c.0[0]],
                            [0.0, 1.0, 0.0, c.0[1]],
                            [0.0, 0.0, 1.0, c.0[2]],
                            [0.0, 0.0, 0.0, 1.0]]);
    let mreturn = Matrix3D([[1.0, 0.0, 0.0, -c.0[0]],
                            [0.0, 1.0, 0.0, -c.0[1]],
                            [0.0, 0.0, 1.0, -c.0[2]],
                            [0.0, 0.0, 0.0, 1.0]]);
    let mx = Matrix3D([[1.0, 0.0, 0.0, 0.0],
                       [0.0, x.cos(), -(x.sin()), 0.0],
                       [0.0, x.sin(), x.cos(), 0.0],
                       [0.0, 0.0, 0.0, 1.0]]);
    let my = Matrix3D([[y.cos(), 0.0, y.sin(), 0.0],
                       [0.0, 1.0, 0.0, 0.0],
                       [-(y.sin()), 0.0, y.cos(), 0.0],
                       [0.0, 0.0, 0.0, 1.0]]);
    let mz = Matrix3D([[z.cos(), -(z.sin()), 0.0, 0.0],
                       [z.sin(), z.cos(), 0.0, 0.0],
                       [0.0, 0.0, 1.0, 0.0],
                       [0.0, 0.0, 0.0, 1.0]]);
    let m = multiplication_matrix3d(&mcentre,&mx);
    let m = multiplication_matrix3d(&m,&my);
    let m = multiplication_matrix3d(&m,&mz);
    let m = multiplication_matrix3d(&m,&mreturn);
    transform_triangle3d(t, &m)
}

fn multiplication_vector3d(m: &Matrix3D, v: &Point3D)-> Point3D{
    let mut res = Point3D([0.0;4]);
    for i in 0..4 {
    for j in 0..4 {
        res.0[i] = res.0[i] + m.0[i][j] * v.0[j]
    }}
    res
}

fn multiplication_matrix3d(m1: &Matrix3D, m2: &Matrix3D)-> Matrix3D{
    let mut res = Matrix3D([[0.0;4];4]);
    for i in 0..4 {
    for j in 0..4 {
    for k in 0..4 {
        res.0[i][j] = res.0[i][j] + m1.0[i][k] * m2.0[k][j];
    }}}
    res
}

