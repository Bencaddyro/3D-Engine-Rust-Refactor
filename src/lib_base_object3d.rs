use crate::lib_surface::*;
use crate::lib_3d::*;
use crate::lib_object3d::*;
use sdl2::pixels::Color;

pub fn dummy()-> Object3D{
    let p1 = new_point3d(10.0,20.0,600.0);
    let p2 = new_point3d(15.0,200.0,600.0);
    let p3 = new_point3d(100.0,220.0,600.0);
    
    let t1 = Triangle3D(p1, p2, p3);
    let t_list = vec![CTriangle3D{t:t1, c:Color::RGB(255,000,000)}];
    Object3D{t_list}
}

pub fn cuboid(lx: f64, ly: f64, lz: f64)-> Object3D{
    let (x,y,z) = (lx/2.0,ly/2.0,lz/2.0);
    //Create 8 vertex point
    let p1 = new_point3d(x,y,z);
    let p2 = new_point3d(-x,y,z);
    let p3 = new_point3d(x,-y,z);
    let p4 = new_point3d(-x,-y,z);
    let p5 = new_point3d(x,y,-z);
    let p6 = new_point3d(-x,y,-z);
    let p7 = new_point3d(x,-y,-z);
    let p8 = new_point3d(-x,-y,-z);
    
    let mut t_list = Vec::new();
    
    let t1 = Triangle3D(p1, p2, p3);
    let t2 = Triangle3D(p2, p3, p4);
    let t3 = Triangle3D(p1, p3, p5);
    let t4 = Triangle3D(p3, p5, p7);
    let t5 = Triangle3D(p1, p2, p5);
    let t6 = Triangle3D(p2, p5, p6);
    let t7 = Triangle3D(p4, p6, p8);
    let t8 = Triangle3D(p2, p4, p6);
    let t9 = Triangle3D(p6, p7, p8);
    let t10 = Triangle3D(p5, p6, p7);
    let t11 = Triangle3D(p4, p7, p8);
    let t12 = Triangle3D(p3, p4, p7);
    
    t_list.push(CTriangle3D{t:t1, c:Color::RGB(255,000,000)});
    t_list.push(CTriangle3D{t:t2, c:Color::RGB(100,000,000)});
    t_list.push(CTriangle3D{t:t3, c:Color::RGB(000,255,000)});
    t_list.push(CTriangle3D{t:t4, c:Color::RGB(000,100,000)});
    t_list.push(CTriangle3D{t:t5, c:Color::RGB(000,000,255)});
    t_list.push(CTriangle3D{t:t6, c:Color::RGB(000,000,100)});
    t_list.push(CTriangle3D{t:t7, c:Color::RGB(255,255,000)});
    t_list.push(CTriangle3D{t:t8, c:Color::RGB(100,100,000)});
    t_list.push(CTriangle3D{t:t9, c:Color::RGB(255,000,255)});
    t_list.push(CTriangle3D{t:t10, c:Color::RGB(100,000,100)});
    t_list.push(CTriangle3D{t:t11, c:Color::RGB(000,255,255)});
    t_list.push(CTriangle3D{t:t12, c:Color::RGB(000,100,100)});
    Object3D{t_list}
}

