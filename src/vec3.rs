#![allow(unused)]
use std::collections::btree_map::Range;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::ops::{Index,IndexMut};
use std::process::Output;
use std::cmp::min;
use rand::{Rng, thread_rng};
use rand::distributions::uniform::{SampleRange, SampleUniform};
use rand::distributions::{Distribution, Standard};

#[derive(Debug,Copy,Clone)]
pub struct Vec3{
    pub e: [f32;3],
}
pub use Vec3 as Point3;
pub use Vec3 as Color;

//constructor
impl Vec3{
    pub fn new(e0: f32, e1: f32, e2:f32) -> Vec3{
        Vec3{  e: [e0,e1,e2]}
    }
    pub fn zero()->Vec3{
        Vec3 { e: [0.0,0.0,0.0] }
    }
}

//Getter, length, length_sq, dot, cross, norm
impl Vec3{
    pub fn x(&self)->f32{self.e[0]}
    pub fn y(&self)->f32{self.e[1]}
    pub fn z(&self)->f32{self.e[2]}

    pub fn length_sq(&self)->f32{
        self.e[0]*self.e[0]+self.e[1]*self.e[1]+self.e[2]*self.e[2]
    }
    pub fn length(&self)->f32{
        (self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2]).sqrt()
    }
    pub fn dot(&self,rhs:&Vec3)->f32{
        self.e[0]*rhs.e[0] + self.e[1]*rhs.e[1] + self.e[2]*rhs.e[2]
    }
    pub fn cross(&self,rhs:&Vec3)->Vec3{
        Vec3 { e: [self.e[1]*rhs.e[2] - self.e[2]*rhs.e[1],
                   self.e[2]*rhs.e[0] - self.e[0]*rhs.e[2],
                   self.e[0]*rhs.e[1] - self.e[1]*rhs.e[0]] }
    }
    pub fn norm(&mut self){
        let a = 1.0/self.length();
        self.e[0]*=a;
        self.e[1]*=a;
        self.e[2]*=a;

    }
    pub fn rotate(&mut self,phi: f32,theta:f32,psi:f32){
        let x = self.x();
        let y = self.y();
        let z = self.z();
        self[0] = x*theta.cos()*psi.cos() 
                 +y*(-phi.cos()*psi.sin() + phi.sin()*theta.sin()*psi.cos()) 
                 +z*(phi.sin()*psi.sin() + phi.cos()*theta.sin()*psi.cos());
        self[1] = x*theta.cos()*psi.sin()
                 +y*(phi.cos()*psi.cos() + phi.sin()*theta.sin()*psi.sin())
                 +z*(-phi.sin()*psi.cos() + phi.cos()*theta.sin()*psi.sin());
        self[2] = x*(-theta.sin())
                 +y*(phi.sin()*theta.sin())
                 +z*(phi.cos()*theta.cos());
    }
    
    pub fn rotate_x(&mut self, angle:f32){
        let y = self.y();
        let z = self.z();
        self[1] = y*angle.cos() - z*angle.sin();
        self[2] = y*angle.sin() + z*angle.cos();
    }
    pub fn rotate_y(&mut self, angle:f32){
        let x = self.x();
        let z = self.z();
        self[0] = x*angle.cos() + z*angle.sin();
        self[2] = -x*angle.sin() + z*angle.cos();
    }
    pub fn rotate_z(&mut self, angle:f32){
        let x = self.x();
        let y = self.y();
        self[0] = x*angle.cos() - y*angle.sin();
        self[1] = x*angle.sin() + y*angle.cos();
    }


    pub fn near_zero(&self)->bool{
        let s = 1e-8;
        return( (self.e[0].abs() < s) && (self.e[1].abs() < s) && (self.e[2].abs() < s) )
    }

}

//Display operator
impl std::fmt::Display for Vec3{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{},{},{}>",self.e[0],self.e[1],self.e[2])
    }
}

//Clone


//Negative of the Vector
impl Neg for Vec3{
    type Output = Vec3;
    fn neg(self) -> Self::Output { Vec3{e: [-self.e[0],-self.e[1],-self.e[2]]} }
}

//Inderx operator
impl Index<usize> for Vec3{
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output { &self.e[index] }
}

//IndexMut operator
impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f32 { &mut self.e[index] }
}

//Operator AddAssign
impl AddAssign for Vec3{
    fn add_assign(&mut self, rhs: Self) {
        self.e[0]+=rhs.e[0];
        self.e[1]+=rhs.e[1];
        self.e[2]+=rhs.e[2];
    }   
}

//Operator SubAssign
impl SubAssign for Vec3{
    fn sub_assign(&mut self, rhs: Self) {
        self.e[0]-=rhs.e[0];
        self.e[1]-=rhs.e[1];
        self.e[2]-=rhs.e[2];
    }   
}

//Operator MulAssign
impl MulAssign<f32> for Vec3{
    fn mul_assign(&mut self, rhs: f32) {
        self.e[0]*=rhs;
        self.e[1]*=rhs;
        self.e[2]*=rhs;
    }
}

//Operator DivAssign
impl DivAssign<f32> for Vec3{
    fn div_assign(&mut self, rhs: f32) {
        self.e[0]/=rhs;
        self.e[1]/=rhs;
        self.e[2]/=rhs;
    }
}

//Operator Add
impl Add for Vec3{
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output{
        Vec3 {e: [self.e[0]+rhs.e[0],self.e[1]+rhs.e[1],self.e[2]+ rhs.e[2]]}
    }
}

//Operator Sub
impl Sub for Vec3{
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 {e: [self.e[0]-rhs.e[0],self.e[1]-rhs.e[1],self.e[2]-rhs.e[2]]} 
    }
}

//Operator Div vec
impl Div<Vec3> for Vec3{
    type Output = Vec3;
    fn div(self, rhs: Vec3) -> Self::Output {
        Vec3 {e: [self.e[0]/rhs.e[0],self.e[1]/rhs.e[1],self.e[2]/rhs.e[2]]}
    }
}

//Operator Div scalar
impl Div<f32> for Vec3{
    type Output = Vec3;
    fn div(self, rhs: f32) -> Self::Output {
        Vec3 {e: [self.e[0]/rhs,self.e[1]/rhs,self.e[2]/rhs]}
    }
}

//Operator Mul vec
impl Mul<Vec3> for Vec3{
    type Output = Vec3;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 {e: [self.e[0]*rhs.e[0],self.e[1]*rhs.e[1],self.e[2]*rhs.e[2]]}
    }
}

//Operator Mul scalar
impl Mul<f32> for Vec3{
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {e: [self.e[0]*rhs,self.e[1]*rhs,self.e[2]*rhs]}
    }
}

impl Distribution<Vec3> for Standard{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R)-> Vec3{
        let (rand_x,rand_y,rand_z) = rng.gen();
        Vec3 { e: [rand_x,rand_y,rand_z] }
    }
}

pub fn rand_vec3<R: Rng + ?Sized>(min:f32, max:f32,rng: &mut R)->Vec3{
    let v:Vec3 = rng.gen();
    v*(max-min)+Vec3 { e: [min,min,min]}
}

pub fn random_in_unit_sphere<R: Rng + ?Sized>(rn: &mut R)->Vec3{
    loop {
        let p = rand_vec3(-1.0, 1.0, rn);
        if p.length_sq()>=1.0{continue;}
        return(p);
    }
}

pub fn random_unit_vector<R: Rng + ?Sized>(rn: &mut R)->Vec3{
    let mut r =random_in_unit_sphere(rn);
    r.norm(); 
    return r;
}

pub fn reflected(v: Vec3, n: Vec3)->Vec3{
    return (v - n*v.dot(&n)*2.0);
}

pub fn refract(uv: Vec3, n:Vec3, etai_over_eta:f32)->Vec3{
    let mut cos_theta = -uv.dot(&n);
    if cos_theta > 1.0{cos_theta = 1.0;}
    let r_out_orth = (uv + n*cos_theta)*etai_over_eta;
    let r_out_para = -n*((1.0 - r_out_orth.length_sq()).abs()).sqrt();
    return(r_out_orth+r_out_para);
}

pub fn random_in_unit_disk()-> Vec3{
    let mut rn = thread_rng();
    loop{
        let p = Vec3::new(rn.gen_range(-1.0..1.0),rn.gen_range(-1.0..1.0),0.0);
        if (p.length_sq() >= 1.0){continue;}
        return p;
    }
}