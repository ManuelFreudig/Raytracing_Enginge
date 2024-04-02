use super::ray::Ray;
use super::vec3::{Vec3,Point3};
use super::hittable::{HitRecord,Hittable};
use super::material::Material;
use super::aabb::AABB;
use super::aabb::surronding_box;
use std::f32::consts::PI;

#[derive(Clone)]
pub struct Sphere{
    pub center: Point3,
    pub radius: f32,
    pub material: Material,
    pub moving:bool,
    pub velocity:Vec3,
}

impl Hittable for Sphere{
    fn hit(&self,r: Ray, t_min: f32, t_max: f32, rec: &mut HitRecord)-> bool{
        let oc = r.origin() - self.center + self.velocity*r.time; 
        let a = r.direction().length_sq();
        let h = oc.dot(&r.direction());
        let c = oc.length_sq() - self.radius*self.radius;

        let discriminant = h*h - a*c;
        if (discriminant < 0.0){
            return false
        }
        let sq_d = discriminant.sqrt();
        let mut root = (-h-sq_d)/a;
        if (root < t_min || t_max < root){
            root = (-h + sq_d)/a;
            if (root < t_min || t_max < root){
                return false
            } 
        }
        
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center)/self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.material = self.material.clone();
        //calculate uv-coords
        let theta = (-outward_normal.y()).acos();
        let phi = (-outward_normal.z()).atan2(outward_normal.x()) + PI;
        rec.u = phi/(2.0*PI);
        rec.v = theta/PI;
        true
    }
    
    fn bounging_box(&self,time0:f32,time1:f32, output_box: &mut AABB)->bool {
        if self.moving{
            let c0 = self.center + self.velocity*time0;
            let c1 = self.center + self.velocity*time1;
            let mut box0 = AABB::new(&(c0 - Vec3 { e: [self.radius,self.radius,self.radius]}), &(c0 + Vec3 { e: [self.radius,self.radius,self.radius]}));
            let mut box1 = AABB::new(&(c1 - Vec3 { e: [self.radius,self.radius,self.radius]}), &(c1 + Vec3 { e: [self.radius,self.radius,self.radius]}));
            let s_box = surronding_box(&mut box0,&mut box1);
            output_box.minimum = s_box.minimum;
            output_box.maximum = s_box.maximum; 
        }
        else{
        output_box.minimum = self.center - Vec3 { e: [self.radius,self.radius,self.radius]};
        output_box.maximum = self.center + Vec3 { e: [self.radius,self.radius,self.radius]};
        }
        return true;
    }
}

impl Sphere{
    pub fn new_stationary(center:Point3,radius:f32,material:Material)->Sphere{
        Sphere { center, radius, material:material, moving: false, velocity: Vec3::zero() }
    }
    pub fn new_moving(center:Point3,radius:f32,material:Material,velocity:Vec3)->Sphere{
        Sphere { center, radius, material:material, moving: true, velocity }
    }

    pub fn shift(&mut self,del_r:Vec3){
        self.center +=del_r
    }
}
