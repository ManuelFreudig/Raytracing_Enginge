use std::cmp::min;
use rand::{Rng, thread_rng};
use super::ray::Ray;
use super::vec3::{Vec3,Point3};
use super::hittable::{HitRecord,Hittable};
use super::material::Material;

#[derive(Clone)]
pub struct Triangle{
    pub vertecies : [Point3;3],
    pub normal : Vec3,
    pub material: Material,
}

impl Triangle {
    pub fn new(v1:Vec3,v2:Vec3,v3:Vec3,mat:Material)->Triangle{
        let mut norm = (v2-v1).cross(&(v3-v1));
        norm.norm();
        return(Triangle{vertecies:[v1,v2,v3],normal:norm,material:mat});
    }
    pub fn given_normal(v1:Vec3,v2:Vec3,v3:Vec3,n:Vec3,mat:Material)->Triangle{
        Triangle{vertecies:[v1,v2,v3],normal:n,material:mat}
    }
    pub fn shif(&mut self,vec:Vec3){
        for vertex in self.vertecies.iter_mut(){
            *vertex+=vec;
        }
    }
    pub fn rotate(&mut self,phi: f32,theta:f32,psi:f32){
        for vertex in self.vertecies.iter_mut(){
            vertex.rotate(phi, theta, psi);
        }
        self.normal.rotate(phi, theta, psi);
    }
    pub fn rotate_x(&mut self,angle:f32){
        for vertex in self.vertecies.iter_mut(){
            vertex.rotate_x(angle);
        }
        self.normal.rotate_x(angle);
    }
    pub fn rotate_y(&mut self,angle:f32){
        for vertex in self.vertecies.iter_mut(){
            vertex.rotate_y(angle);
        }
        self.normal.rotate_y(angle);
    }
    pub fn rotate_z(&mut self,angle:f32){
        for vertex in self.vertecies.iter_mut(){
            vertex.rotate_z(angle);
        }
        self.normal.rotate_z(angle);
    }
}

impl std::fmt::Display for Triangle{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "1:{} 2:{} 3:{} n:{}",self.vertecies[0],self.vertecies[1],self.vertecies[2],self.normal)
    }
}    

impl Hittable for Triangle{
    fn hit(&self,r: Ray, t_min: f32, t_max: f32, rec: &mut HitRecord)-> bool{
        let epsilon = 0.0000001;
        let v0 = self.vertecies[0];
        let v1 = self.vertecies[1];
        let v2 = self.vertecies[2];
        let mut edge1 = v1-v0;
        let mut edge2 = v2-v0;
        let h = r.dir.cross(&edge2);
        let a = edge1.dot(&h);
        if (a > -epsilon && a < epsilon){
            return false;
        }
        let f = 1.0/a;
        let s = r.orig-v0;
        let u = f*s.dot(&h);
        if(u < 0.0 || u > 1.0){
            return false;
        }
        let q = s.cross(&edge1);
        let v = f*r.dir.dot(&q);
        if (v < 0.0 || u+v > 1.0){
            return false;
        }
        let t_intersection = f*edge2.dot(&q);
        if (t_intersection < t_min || t_max < t_intersection){
            return false;
        }
        rec.u = u;
        rec.v = v; 
        rec.t = t_intersection;
        rec.p = r.at(rec.t);
        rec.normal = self.normal;
        rec.front_face = r.direction().dot(&self.normal)>0.0;
        rec.material = self.material.clone();

        return true;
    }

    fn bounging_box(&self,time0:f32,time1:f32, output_box: &mut crate::aabb::AABB)->bool {
        let mut rn = thread_rng();
        let mut minimum = Point3::new(self.vertecies[0].x().min(self.vertecies[1].x().min(self.vertecies[2].x())),
                                        self.vertecies[0].y().min(self.vertecies[1].y().min(self.vertecies[2].y())),
                                        self.vertecies[0].z().min(self.vertecies[1].z().min(self.vertecies[2].z())));
        let mut maximum = Point3::new(self.vertecies[0].x().max(self.vertecies[1].x().max(self.vertecies[2].x())),
                                        self.vertecies[0].y().max(self.vertecies[1].y().max(self.vertecies[2].y())),
                                        self.vertecies[0].z().max(self.vertecies[1].z().max(self.vertecies[2].z())));
        for i in (0..2){
            if maximum[i]-minimum[i] < 0.0002{
                maximum[i]+=(0.0001+rn.gen_range(-0.00005..0.00005));
                minimum[i]-=(0.0001-rn.gen_range(-0.00005..0.00005));
            }
        }
        output_box.minimum = minimum;
        output_box.maximum = maximum;
        return true;
    }
}