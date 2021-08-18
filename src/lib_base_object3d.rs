use crate::lib_3d::*;
use crate::lib_object3d::*;
use sdl2::pixels::Color;

#[allow(dead_code)]
pub fn dummy()-> Object3D{
    let p1 = new_point3d(10.0,20.0,600.0);
    let p2 = new_point3d(15.0,200.0,600.0);
    let p3 = new_point3d(100.0,220.0,600.0);
    
    let t1 = Triangle3D(p1, p2, p3);
    let t_list = vec![CTriangle3D{t:t1, c:Color::RGB(255,000,000)}];
    Object3D{t_list}
}

#[allow(dead_code)]
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

#[allow(dead_code)]
pub fn cylinder(radius: f64, lx: f64, nlong: usize)-> Object3D{
    let h = lx/2.0;
    let x0 = new_point3d(h,0.0,0.0);
    let x1 = new_point3d(-h,0.0,0.0);
    let mut t_list = Vec::new();
    
    let mut x0i = new_point3d(h,radius,0.0);
    let mut x1i = new_point3d(-h,radius,0.0);
    
    for i in 1..nlong{
        let x0ii = new_point3d(h,radius*((360.0*i as f64/nlong as f64).to_radians().cos()),radius*((360.0*i as f64/nlong as f64).to_radians().sin()));
        let x1ii = new_point3d(-h,radius*((360.0*i as f64/nlong as f64).to_radians().cos()),radius*((360.0*i as f64/nlong as f64).to_radians().sin()));
        
        let t0 = Triangle3D(x0, x0i, x0ii);
        let t1 = Triangle3D(x1, x1i, x1ii);
        t_list.push(CTriangle3D{t:t0, c:Color::RGB(100,000,000)});
        t_list.push(CTriangle3D{t:t1, c:Color::RGB(255,000,000)});
        
        let t0 = Triangle3D(x0i, x0ii, x1i);
        let t1 = Triangle3D(x1i, x1ii, x0ii);
        t_list.push(CTriangle3D{t:t0, c:Color::RGB(255,000,000)});
        t_list.push(CTriangle3D{t:t1, c:Color::RGB(100,000,000)});
        
        x0i = x0ii;
        x1i = x1ii;
    }
    let x0ii = new_point3d(h,radius,0.0);
    let x1ii = new_point3d(-h,radius,0.0);
    
    let t0 = Triangle3D(x0, x0i, x0ii);
    let t1 = Triangle3D(x1, x1i, x1ii);
    t_list.push(CTriangle3D{t:t0, c:Color::RGB(100,000,000)});
    t_list.push(CTriangle3D{t:t1, c:Color::RGB(255,000,000)});
    
    let t0 = Triangle3D(x0i, x0ii, x1i);
    let t1 = Triangle3D(x1i, x1ii, x0ii);
    t_list.push(CTriangle3D{t:t0, c:Color::RGB(255,000,000)});
    t_list.push(CTriangle3D{t:t1, c:Color::RGB(100,000,000)});
    
    Object3D{t_list}
}

#[allow(dead_code)]
pub fn spheroid(radius: f64, nlong: usize, nlat: usize)-> Object3D{
    //let p0 = new_point3d(radius,0.0,0.0);
    //let p1 = new_point3d(-radius,0.0,0.0);
    let mut t_list = Vec::new();

    fn get_height_radius(radius: f64, i: usize, nlat: usize)-> (f64, f64){
        let r = radius * (180.0*i as f64/nlat as f64).to_radians().sin();
        let h = radius * (180.0*i as f64/nlat as f64).to_radians().cos();
        (r,h)
    }
    
    fn get_x_y(radius: f64, i: usize, nlong: usize)-> (f64, f64){
        let x = radius * (360.0*i as f64/nlong as f64).to_radians().cos();
        let y = radius * (360.0*i as f64/nlong as f64).to_radians().sin();
        (x,y)
    }
    
    for i in 0..nlat{
        let (ri,hi) = get_height_radius(radius, i, nlat);
        let (rii,hii) = get_height_radius(radius, i+1, nlat);
        
        for j in 0..nlong{
            let (xij,yij) = get_x_y(ri, j, nlong);
            let (xijj,yijj) = get_x_y(ri, j+1, nlong);
            let (xiij,yiij) = get_x_y(rii, j, nlong);
            let (xiijj,yiijj) = get_x_y(rii, j+1, nlong);
            
            let pij = new_point3d(hi,xij,yij);
            let piij = new_point3d(hii,xiij,yiij);
            let pijj = new_point3d(hi,xijj,yijj);
            let piijj = new_point3d(hii,xiijj,yiijj);
            
            let t0 = Triangle3D(pij, piij, pijj);
            let t1 = Triangle3D(piijj, piij, pijj);
            t_list.push(CTriangle3D{t:t0, c:Color::RGB(100,000,000)});
            t_list.push(CTriangle3D{t:t1, c:Color::RGB(255,000,000)});               
        }
    }
    Object3D{t_list}
}





































