//Bounding volume hierachy

use crate::aabb::surronding_box;
use crate::material::Material;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

use super::hittable::{Hittable,HitRecord};
use super::hittable_list::HittableList;
use super::aabb::AABB;
use super::ray::Ray;
use rand::{Rng, thread_rng};
use std::borrow::BorrowMut;
use std::cmp::Ordering;
use std::rc::Rc;

#[derive(Clone)]
pub struct BvhNode{
    pub left: Rc<dyn Hittable>,
    pub right: Rc<dyn Hittable>,
    pub h_box: AABB,
}

impl Hittable for BvhNode{
    fn bounging_box(&self,time0:f32,time1:f32, output_box: &mut AABB)->bool {
        output_box.minimum = self.h_box.minimum;
        output_box.maximum = self.h_box.maximum;
        return true;
    }
    fn hit(&self,r: Ray, t_min: f32, t_max: f32, rec: &mut HitRecord)-> bool {
        let (mut tmin, mut tmax) = (t_min,t_max);
        if (!self.h_box.hit(&r, &mut tmin, &mut tmax)){return false;}
        
        let hit_left = self.left.hit(r, tmin, tmax, rec);
        let hit_right = self.right.hit(r, tmin, if hit_left {rec.t} else {tmax}, rec);
        
        return (hit_left || hit_right);
    }
}

impl BvhNode {
    pub fn empty()->BvhNode{
        BvhNode { left: Rc::new(Sphere::new_stationary(Vec3::zero(), 0.0, Material::empty())), 
                  right: Rc::new(Sphere::new_stationary(Vec3::zero(), 0.0, Material::empty())), 
                  h_box: AABB::zero() }
    }
    pub fn bvhnode(&mut self,src_objects: &Vec<Rc<dyn Hittable>>, start: usize, end:usize, time0:f32,time1:f32){
        let mut rn = thread_rng();
        let mut objects = src_objects.clone();
        let axis: usize = rn.gen_range(0..=2);
        let comparator = if axis == 0 {box_x_compare} else if axis==1 {box_y_compare} else {box_z_compare};

        let object_span = end-start;
        
        if object_span == 1{
            self.left = objects[start].clone();
            self.right = objects[start].clone()}
        else if object_span == 2 {
            if comparator(&objects[start],&objects[start+1]) == Ordering::Less{
                self.left = objects[start].clone();
                self.right = objects[start+1].clone();
            }
            else{
                self.left = objects[start+1].clone();
                self.right = objects[start].clone();
            }
        }
        else{
            objects[start..end].sort_by(|a,b| comparator(a,b));
            
            let mid = start + object_span/2;
            
            let mut left_node = BvhNode::empty();
            left_node.bvhnode(&objects, start, mid, time0, time1);
            let mut right_node = BvhNode::empty();
            right_node.bvhnode(&objects, mid, end, time0, time1);
            
            self.left = Rc::new(left_node);
            self.right = Rc::new(right_node);
        }
        let mut box_left = AABB::zero();
        let mut box_right = AABB::zero();
        if(!self.left.bounging_box(time0, time1, &mut box_left) || !self.right.bounging_box(time0, time1, &mut box_right)){
            println!("No bounding box in BvhNode constructor.")
        }
        self.h_box = surronding_box(&mut box_left, &mut box_right);      
    }
}


pub fn box_compare(a: &Rc<dyn Hittable>, b:&Rc<dyn Hittable>, axis:usize)->Ordering{
    let mut box_a = AABB::zero();
    let mut box_b = AABB::zero();

    if (!a.bounging_box(0.0, 0.0, &mut box_a) || !b.bounging_box(0.0, 0.0, &mut box_b)){
        println!("No bounding box in BvhNode constructor");
    }
    //Test: add a little bit to box_b
    return box_a.minimum.e[axis].partial_cmp(&(box_b.minimum.e[axis])).unwrap();
}

pub fn box_x_compare(a: &Rc<dyn Hittable>, b:&Rc<dyn Hittable>)->Ordering{box_compare(a, b, 0)}
pub fn box_y_compare(a: &Rc<dyn Hittable>, b:&Rc<dyn Hittable>)->Ordering{box_compare(a, b, 1)}
pub fn box_z_compare(a: &Rc<dyn Hittable>, b:&Rc<dyn Hittable>)->Ordering{box_compare(a, b, 2)}