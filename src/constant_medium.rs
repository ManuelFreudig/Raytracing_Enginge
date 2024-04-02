use std::rc::Rc;
use rand::{thread_rng,Rng};
use super::vec3::{Vec3,Point3,Color};
use super::ray::Ray;
use super::hittable::Hittable;
use super::hittable::HitRecord;
use super::material::Material;
use super::texture::Texture;
use super::aabb::AABB;

#[derive(Clone)]
pub struct ConstantMedium{
    boudary: Rc<dyn Hittable>,
    phase_function: Material,
    neg_inv_density: f32,
}

impl ConstantMedium{
    pub fn new(b: Rc<dyn Hittable>, d:f32, a: Rc<dyn Texture>)->ConstantMedium{
        ConstantMedium { boudary: b, phase_function: Material::new_isotropic(a), neg_inv_density: -1.0/d }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self,r: Ray, t_min: f32, t_max: f32, rec: &mut HitRecord)-> bool {
        let mut rn = thread_rng();
        let enabledebug = false;
        let debugging = enabledebug && rn.gen_range(0.0..1.0) < 0.00001;

        let mut rec1 = HitRecord::new();
        let mut rec2 = HitRecord::new();

        if(!self.boudary.hit(r, f32::MIN, f32::MAX, &mut rec1)){ return false; }
        if(!self.boudary.hit(r, rec1.t+0.0001, f32::MAX, &mut rec2)){ return false; }

        if (debugging){
            println!("t_min = {}, t_max = {}",rec1.t,rec2.t);
        }

        if (rec1.t < t_min){ rec1.t = t_min; }
        if (rec2.t > t_max){ rec2.t = t_max; }

        if (rec1.t >= rec2.t){ return false; }

        if (rec1.t < 0.0){ rec1.t = 0.0; }
        
        let ray_length = r.dir.length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let d_range:f32 = rn.gen(); 
        let hit_distance = self.neg_inv_density * d_range.log2();

        if (hit_distance > distance_inside_boundary){ return false; }

        rec.t = rec1.t + hit_distance/ray_length;
        rec.p = r.at(rec.t);

        if (debugging){
            println!("hit_distance = {hit_distance}\n rec.t = {}\n rec.p = {}",rec.t,rec.p)
        }
        rec.normal = Vec3::new(1.0,0.0,0.0);
        rec.front_face = true;
        rec.material = self.phase_function.clone();

        return true;
    }
    
    fn bounging_box(&self,time0:f32,time1:f32, output_box: &mut AABB)->bool {
        return self.boudary.bounging_box(time0, time1, output_box);
    }
}

