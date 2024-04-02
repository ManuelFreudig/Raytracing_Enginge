#![allow(unused)]
use super::material::Material;
use super::ray::Ray;
use super::vec3::{Vec3,Point3,Color};
use super::aabb::AABB;
use std::rc::Rc;

#[derive(Clone)]
pub struct HitRecord{
    pub p: Point3,
    pub normal: Vec3,
    pub material: Material,
    pub t: f32, 
    pub front_face: bool,
    pub u:f32,
    pub v:f32, 
}

impl HitRecord{
    pub fn new()->HitRecord{
        HitRecord{p: Point3{e:[0.0,0.0,0.0]},normal: Vec3 { e:[0.0,0.0,0.0] }, material: Material::empty(),t:0.0, front_face:true ,u: 0.0, v: 0.0}
    }
    pub fn set_face_normal(&mut self,r: Ray, outward_normal: Vec3){
        self.front_face = r.direction().dot(&outward_normal)<0.0;
        self.normal = if self.front_face {outward_normal } else {-outward_normal};
    }
}


pub trait Hittable:CloneHittable{
    fn hit(&self,r: Ray, t_min: f32, t_max: f32, rec: &mut HitRecord)-> bool;
    fn bounging_box(&self,time0:f32,time1:f32, output_box: &mut AABB)->bool;
}

trait CloneHittable{
    fn clone_box(&self) -> Box<dyn Hittable>;
}

impl<T> CloneHittable for T 
where 
    T: 'static + Hittable + Clone,{
    fn clone_box(&self)-> Box<dyn Hittable>{
        Box::new(self.clone())
    }
}