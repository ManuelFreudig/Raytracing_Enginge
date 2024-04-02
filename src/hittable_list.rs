#![allow(unused)]
use std::rc::Rc;


use super::vec3::{Vec3,Point3};
use super::ray::Ray;
use super::hittable::{HitRecord,Hittable};
use super::sphere::Sphere;
use super::aabb::AABB;
use super::aabb::surronding_box;

#[derive(Clone)]
pub struct HittableList{
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new(&mut self, object: Rc<dyn Hittable>){
        self.add(object)
    }
    pub fn add(&mut self, object: Rc<dyn Hittable>){
        self.objects.push(object);
    }
    pub fn merge(&mut self, hit_list:HittableList){
        for obj in hit_list.objects{
            self.add(obj);
        }
    }
    pub fn len(&self)->usize{
        self.objects.len()
    }

}

impl Hittable for HittableList{
    fn hit(&self,r: Ray, t_min: f32, t_max: f32, rec: &mut HitRecord)-> bool{
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far:f32 = t_max;
        for object in &self.objects{
            if object.hit(r,t_min,closest_so_far,&mut temp_rec){
                hit_anything = true;
                closest_so_far = temp_rec.clone().t;
                *rec = temp_rec.clone();
            }
        }
        return hit_anything
    }

    fn bounging_box(&self,time0:f32,time1:f32, output_box: &mut crate::aabb::AABB)->bool {
        if self.objects.len() == 0{ return false;}

        let mut temp_box = AABB::new(&Vec3::zero(),&Vec3::zero());
        let mut first_box = true;
        for object in self.objects.iter(){
            if !object.bounging_box(time0, time1, &mut temp_box){ return  false;}
            if first_box{*output_box = temp_box; first_box = false;}
            else {*output_box = surronding_box(output_box,&mut temp_box);}
        }
        return true;
    }
}