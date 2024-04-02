use std::str::FromStr;

use crate::vec3::refract;

use super::vec3::{random_unit_vector,reflected,random_in_unit_sphere};
use rand::{thread_rng,Rng};
use super::hittable::HitRecord;
use super::vec3::{Color,Vec3,Point3}; 
use super::ray::Ray;
use super::texture::Texture;
use super::texture::{SolidColor};
use std::rc::Rc;

#[derive(Clone)]
pub struct Material{
    id: u8,
    albedo: Rc<dyn Texture>,
    fuzz: f32,
    ref_idx: f32,


}


impl Material {
    pub fn empty()->Material{
        Material { id: 0, albedo: Rc::new(SolidColor::new(Color::zero())),fuzz:0.0,ref_idx:0.0}
    }

    pub fn scatter(&self,r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray)->bool{
        
        match self.id {
            1 => return self.scatter_lambertian(r_in, rec, attenuation, scattered),
            2 => return self.scatter_metal(r_in, rec, attenuation, scattered),
            3 => return self.scatter_dielectric(r_in, rec, attenuation, scattered),
            4 => return self.scatter_diffuse_light(r_in, rec, attenuation, scattered),
            5 => return self.scatter_istoropic(r_in,rec,attenuation,scattered),
            _ =>  {println!("Error: Id not in range"); return false}
        }
    }
}


//Lambertian
impl Material{
    pub fn new_lambertian(color: Rc<dyn Texture>)->Material{
        Material{id:1, albedo:color,fuzz:0.0,ref_idx:0.0}        
    }

    fn scatter_lambertian(&self,r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray)-> bool {
        let mut rn = rand::thread_rng();
        let mut scatter_direction = rec.normal + random_unit_vector(&mut rn);
        //Catch degenerate scatter direction
        if (scatter_direction.near_zero()){scatter_direction = rec.normal}
        
        scattered.orig = rec.p;
        scattered.dir = scatter_direction;
        attenuation.e = self.albedo.value(rec.u, rec.v, rec.p).e;
        return true;
    }
}


//Metal
impl Material{
    pub fn new_metal(color: Color,fuzz:f32)->Material{
        Material{id:2, albedo:Rc::new(SolidColor::new(Color{e:color.e})),fuzz:fuzz,ref_idx:0.0}        
    }

    fn scatter_metal(&self,r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray)-> bool{
        let mut v = r_in.dir;
        v.norm();
        let mut rn = rand::thread_rng();
        let reflect = reflected(v, rec.normal);
        scattered.orig = rec.p;
        scattered.dir = reflect + random_in_unit_sphere(&mut rn)*self.fuzz;
        attenuation.e = self.albedo.value(rec.u, rec.v, rec.p).e;
        return (scattered.dir.dot(&rec.normal) > 0.0);
    }
}


//Dielectric
impl Material{
    pub fn new_dielectric(refraction_index:f32)->Material{
        Material{id:3, albedo:Rc::new(SolidColor::new(Color::zero())),fuzz:0.0,ref_idx:refraction_index}
    }

    fn scatter_dielectric(&self,r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray)-> bool{
        let mut rn = rand::thread_rng();
        attenuation.e = [1.0,1.0,1.0];
        let refraction_ratio = if rec.front_face {(1.0/self.ref_idx)} else {self.ref_idx};

        let mut unit_dir = r_in.dir;
        unit_dir.norm();

        let mut cos_theta = -unit_dir.dot(&rec.normal);
        if cos_theta > 1.0 {cos_theta = 1.0;}
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        let cant_refract = refraction_ratio*sin_theta > 1.0;
        let mut direction = Vec3::zero();

        if (cant_refract || reflectance(cos_theta, refraction_ratio) > rn.gen()){direction = reflected(unit_dir, rec.normal);}
        else {direction = refract(unit_dir, rec.normal, refraction_ratio);}

        
        scattered.orig = rec.p;
        scattered.dir = direction;
        return(true);  
    }
    
}


//diffuse_light
impl Material{
    pub fn new_diffuse_light(color:Rc<dyn Texture>)->Material{
        Material{id:4, albedo:color,fuzz:0.0,ref_idx:0.0}
    }
    fn scatter_diffuse_light(&self,r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray)-> bool{false}

    pub fn emitted(&self,u:f32, v:f32, p:&Point3)->Color{
        if self.id == 4{ return self.albedo.value(u, v, *p); }
        return Color::zero();
    }
}


//isotropic
impl Material{
    pub fn new_isotropic(color:Rc<dyn Texture>)->Material{
        Material { id: 5, albedo: color, fuzz: 0.0, ref_idx: 0.0 }
    }
    fn scatter_istoropic(&self,r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray)-> bool{
        let mut rn = thread_rng();
        scattered.orig = rec.p;
        scattered.dir = random_in_unit_sphere(&mut rn);
        scattered.time = r_in.time();
        attenuation.e = self.albedo.value(rec.u, rec.v, rec.p).e;
        return true;
    }

}

fn reflectance(cos: f32, ref_idx: f32)->f32{
    //Schlicks approximation for reflectance.
    let mut r0 = (1.0-ref_idx)/(1.0+ref_idx);
    r0 = r0*r0;
    return r0 + (1.0 - r0)*(1.0 - cos).powf(5.0); 
}