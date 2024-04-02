use crate::vec3::{random_in_unit_sphere,random_in_unit_disk};

use super::vec3::{Point3,Vec3,Color};
use super::ray::Ray;
use std::f32::consts::PI;
use rand::{thread_rng,Rng};
#[derive(Debug,Copy,Clone)]
pub struct Camera{
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f32,
    pub _time0: f32,
    pub _time1:f32,
}

impl Camera{
    pub fn new(lookfrom:Point3, lookat:Point3, v_up:Vec3 ,v_fov: f32, aspect_ratio:f32, aperture: f32, focus_dist:f32, _time0:f32,_time1:f32)->Camera{
        let theta = v_fov*PI/180.0;
        let h = (theta/2.0).tan();
        let viewport_height = 2.0*h;
        let viewport_width = aspect_ratio*viewport_height;
        
        let mut w = lookfrom-lookat; w.norm();
        let mut u = v_up.cross(&w); u.norm();
        let v = w.cross(&u);

        let origin = lookfrom;
        let horizontal = u*viewport_width*focus_dist;
        let vertical = v*viewport_height*focus_dist;
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - w*focus_dist;

        Camera{origin:origin,lower_left_corner:lower_left_corner,horizontal:horizontal,vertical:vertical,u:u,v:v,w:w,lens_radius:aperture/2.0,_time0,_time1}
    }
    pub fn get_ray(self,s: f32, t:f32)->Ray{
        let mut rn = thread_rng();
        let rd = random_in_unit_disk()*self.lens_radius;
        let offset = self.u*rd.x() + self.v*rd.y();
        let time = rn.gen_range(self._time0..self._time1);
        Ray::new(self.origin+offset, self.lower_left_corner + self.horizontal*s + self.vertical*t - self.origin - offset,time)
    }
}