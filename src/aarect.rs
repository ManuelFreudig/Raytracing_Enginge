use super::triangle::Triangle;
use super::hittable::Hittable;
use super::material::Material;
use super::ray::Ray;
use super::hittable::HitRecord;
use super::vec3::{Vec3,Point3};
use super::aabb::AABB;
use super::Object_3D;



pub fn new_xyrect(x0:f32,x1:f32,y0:f32,y1:f32,k:f32,material:Material,center_faced:bool)->Object_3D{
    let mut f = 1.0;
    if k < 0.0001{ f = if center_faced {-1.0} else {1.0}; }
    else{ f = if center_faced {-k/k.abs()} else {-k/k.abs()}; }
    let mut verteces = Vec::new();
    let mut faces = Vec::new();
    verteces.push(Point3{e:[x0,y0,k]});
    verteces.push(Point3{e:[x0,y1,k]});
    verteces.push(Point3{e:[x1,y0,k]});
    verteces.push(Point3{e:[x1,y1,k]});
    faces.push(Triangle::given_normal(verteces[0],verteces[1],verteces[2],Vec3{e:[0.0,0.0,f]},material.clone()));
    faces.push(Triangle::given_normal(verteces[1],verteces[2],verteces[3],Vec3{e:[0.0,0.0,f]},material));
    return Object_3D { pivo: Vec3 { e: [(x1-x0)/2.0,(y1-y0)/2.0,k] }, verteces, faces, moving: false, velocity: Vec3::zero() };
}

pub fn new_xzrect(x0:f32,x1:f32,z0:f32,z1:f32,k:f32,material:Material,center_faced:bool)->Object_3D{
    let mut f = 1.0;
    if k < 0.0001{ f = if center_faced {-1.0} else {1.0}; }
    else{ f = if center_faced {-k/k.abs()} else {-k/k.abs()}; }
    let mut verteces = Vec::new();
    let mut faces = Vec::new();
    verteces.push(Point3{e:[x0,k,z0]});
    verteces.push(Point3{e:[x0,k,z1]});
    verteces.push(Point3{e:[x1,k,z0]});
    verteces.push(Point3{e:[x1,k,z1]});
    faces.push(Triangle::given_normal(verteces[0],verteces[1],verteces[2],Vec3{e:[0.0,f,0.0]},material.clone()));
    faces.push(Triangle::given_normal(verteces[1],verteces[2],verteces[3],Vec3{e:[0.0,f,0.0]},material));
    return Object_3D { pivo: Vec3 { e: [(x1-x0)/2.0,k,(z1-z0)/2.0] }, verteces, faces, moving: false, velocity: Vec3::zero() };
}

pub fn new_yzrect(y0:f32,y1:f32,z0:f32,z1:f32,k:f32,material:Material,center_faced:bool)->Object_3D{
    let mut f = 1.0;
    if k < 0.0001{ f = if center_faced {-1.0} else {1.0}; }
    else{ f = if center_faced {-k/k.abs()} else {k/k.abs()}; }
    let mut verteces = Vec::new();
    let mut faces = Vec::new();
    verteces.push(Point3{e:[k,y0,z0]});
    verteces.push(Point3{e:[k,y0,z1]});
    verteces.push(Point3{e:[k,y1,z0]});
    verteces.push(Point3{e:[k,y1,z1]});
    faces.push(Triangle::given_normal(verteces[0],verteces[1],verteces[2],Vec3{e:[f,0.0,0.0]},material.clone()));
    faces.push(Triangle::given_normal(verteces[1],verteces[2],verteces[3],Vec3{e:[f,0.0,0.0]},material));
    return Object_3D { pivo: Vec3 { e: [k,(y1-y0)/2.0,(z1-z0)/2.0] }, verteces, faces, moving: false, velocity: Vec3::zero() };
}


#[derive(Clone)]
pub struct XYRect{
    pub material: Material,
    pub x0:f32,
    pub x1:f32,
    pub y0:f32,
    pub y1:f32,
    pub k:f32,
}

impl XYRect{
    pub fn new(x0:f32,x1:f32,y0:f32,y1:f32,k:f32,material:Material)->XYRect{
        XYRect { material, x0, x1, y0, y1, k }
    }
}

impl Hittable for XYRect {
    fn hit(&self,r: crate::ray::Ray, t_min: f32, t_max: f32, rec: &mut crate::hittable::HitRecord)-> bool {
        let t = (self.k-r.orig.z())/r.dir.z();
        if (t < t_min || t > t_max) { return false; }

        let x = r.orig.x() + t*r.dir.x();
        let y = r.orig.y() + t*r.dir.y();
        if (x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1){ return false; }

        rec.u = (x-self.x0)/(self.x1-self.x0);
        rec.v = (y-self.y0)/(self.y1-self.y0);
        rec.t = t;
        let outward_normal = Vec3::new(0.0,0.0,1.0);
        rec.set_face_normal(r, outward_normal);
        rec.material = self.material.clone();
        rec.p = r.at(t);
        return true;
    }

    fn bounging_box(&self,time0:f32,time1:f32, mut output_box: &mut AABB)->bool {
        output_box.minimum = Point3::new(self.x0,self.y0,self.k-0.0001);
        output_box.maximum = Point3::new(self.x1,self.y1,self.k+0.0001);
        return true;
    }
}