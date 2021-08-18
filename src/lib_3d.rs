use crate::lib_surface::*;
use crate::lib_2d::*;
use sdl2::pixels::Color;

#[derive(Copy, Clone)]
pub struct Matrix3D(pub [[f64;4];4]);
#[derive(Copy, Clone)]
pub struct Point3D([f64;4]);
#[derive(Copy, Clone)]
pub struct Triangle3D(pub Point3D, pub Point3D, pub Point3D);

pub struct Plan3D{//a*x+b*y+c*z=d
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
}

pub fn new_point3d(x: f64, y: f64, z: f64)->Point3D{
    Point3D([x, y, z, 1.0])
}

pub fn new_vector3d(x: f64, y: f64, z: f64)->Point3D{
    Point3D([x, y, z, 0.0])
}

fn get_plan(t: &Triangle3D)-> Plan3D{
    let acx = t.2.0[0]-t.0.0[0];
    let acy = t.2.0[1]-t.0.0[1];
    let acz = t.2.0[2]-t.0.0[2];
    let bcx = t.2.0[0]-t.1.0[0];
    let bcy = t.2.0[1]-t.1.0[1];
    let bcz = t.2.0[2]-t.1.0[2];
    
    let a = acy * bcz - acz * bcy;
    let b = acz * bcx - acx * bcz;
    let c = acx * bcy - acy * bcx;
    let d = a * t.2.0[0] + b * t.2.0[1] + c * t.2.0[2];
    
    Plan3D{a, b, c, d}
}

#[allow(dead_code)]
pub fn print_triangle3d(t: &Triangle3D){
    println!("Triangle ({},{},{})-({},{},{})-({},{},{})",t.0.0[0],t.0.0[1],t.0.0[2],t.1.0[0],t.1.0[1],t.1.0[2],t.2.0[0],t.2.0[1],t.2.0[2],);
}

fn convert_3d_2d(p3d: &Point3D, h: f64, xsize: usize, ysize: usize)-> Point2D{
    //assuming centre of projection is (0,0,0) and projection plane have (0,0,1) as normal vector, and z = h
    let m = Matrix3D([[h/p3d.0[2], 0.0, 0.0, 0.0],
                      [0.0, h/p3d.0[2], 0.0, 0.0],
                      [0.0, 0.0, h/p3d.0[2], 0.0],
                      [0.0, 0.0, 0.0, 1.0]]);
    let mut p = new_point3d(0.0,0.0,0.0);
    multiplication_vector3d(&mut p, &m, &p3d);
    let x = p.0[0] + xsize as f64/2.0;
    let y = p.0[1] + ysize as f64/2.0;
    Point2D(x as i32, y as i32)
}

pub fn fill_triangle3d(s: &mut Screen, t: &Triangle3D, color: Color, h: f64){
    //TODO check if t is in render space aka in front of camera, otherwise drop render -> z > h
    //print_triangle3d(t);
    if t.0.0[2] < 0.000_000_1 || t.1.0[2] < 0.000_000_1 || t.2.0[2] < 0.000_000_1 { return () } //if any point of t is behind abort rendering, 0.000_000_1 instead of 0 to bypass small float rounding problem
    
    // orthogonal projection on 2d plane
    let a = convert_3d_2d(&t.0, h, s.x, s.y);
    let b = convert_3d_2d(&t.1, h, s.x, s.y);
    let c = convert_3d_2d(&t.2, h, s.x, s.y);
    let t2d = Triangle2D(a,b,c);
    let plan = get_plan(t);
    //TODO move plan creation elsewhere for opti ?
    
    fill_triangle2d(s, t2d, plan, h, color);
}

fn transform_triangle3d(t: &mut Triangle3D, m: &Matrix3D){
    let tt = (*t).clone();
    multiplication_vector3d(&mut t.0, m, &tt.0);
    multiplication_vector3d(&mut t.1, m, &tt.1);
    multiplication_vector3d(&mut t.2, m, &tt.2);
}

pub fn translation_triangle3d(t: &mut Triangle3D, v: &Point3D){
    let m = Matrix3D([[1.0, 0.0, 0.0, v.0[0]],
                      [0.0, 1.0, 0.0, v.0[1]],
                      [0.0, 0.0, 1.0, v.0[2]],
                      [0.0, 0.0, 0.0, 1.0]]);
    transform_triangle3d(t, &m)
}

pub fn rotation_triangle3d(t: &mut Triangle3D, c: &Point3D, x: f64, y: f64, z: f64){
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
    let mut m = Matrix3D([[0.0;4];4]);
    multiplication_matrix3d(&mut m, &mcentre, &mx);
    let m_copy = m.clone();
    multiplication_matrix3d(&mut m, &m_copy, &mx);
    let m_copy = m.clone();
    multiplication_matrix3d(&mut m, &m_copy, &my);
    let m_copy = m.clone();
    multiplication_matrix3d(&mut m, &m_copy, &mz);
    let m_copy = m.clone();
    multiplication_matrix3d(&mut m, &m_copy, &mreturn);
    transform_triangle3d(t, &m);
}

pub fn multiplication_vector3d(v1: &mut Point3D, m: &Matrix3D, v2: &Point3D){//v1 = m * v2
    v1.0 = [0.0;4];
    for i in 0..4 {
    for j in 0..4 {
        v1.0[i] = v1.0[i] + m.0[i][j] * v2.0[j]
    }}
}

fn multiplication_matrix3d(m1: &mut Matrix3D, m2: &Matrix3D, m3: &Matrix3D){//m1 = m2 * m3
    m1.0 = [[0.0;4];4];
    for i in 0..4 {
    for j in 0..4 {
    for k in 0..4 {
        m1.0[i][j] = m1.0[i][j] + m2.0[i][k] * m3.0[k][j];
    }}}
}

