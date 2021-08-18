use crate::lib_surface::*;
use crate::lib_3d::*;
use sdl2::pixels::Color;

pub struct CTriangle3D{//ColoredTriangle
    pub t: Triangle3D,
    pub c: Color,
}

pub struct Object3D{
    pub t_list: Vec<CTriangle3D>,
}

pub fn draw_object3d(s: &mut Screen, o: &Object3D, h: f64){
    for ct in &o.t_list {
        fill_triangle3d(s, &ct.t, ct.c, h);
    }
}
   
pub fn translation_object3d(o: &mut Object3D, v: &Point3D){
    for ct in &mut o.t_list {
        translation_triangle3d(&mut ct.t, v);
    }
}

pub fn rotation_object3d(o: &mut Object3D, c: &Point3D, x: f64, y: f64, z: f64){
    for ct in &mut o.t_list {
        rotation_triangle3d(&mut ct.t, c, x, y, z);
    }
}

