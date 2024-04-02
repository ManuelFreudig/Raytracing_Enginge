#![allow(unused)]
use std::rc::Rc;
use std::ops::Index;
use crate::triangle;

use super::vec3::{Vec3,Point3};
use super::ray::Ray;
use super::hittable::{HitRecord,Hittable};
use super::sphere::Sphere;
use super::hittable_list::HittableList;
use super::triangle::Triangle;
use super::material::Material;
use super::aarect::{new_xyrect,new_xzrect,new_yzrect};
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::time::{self, Instant};

#[allow(non_camel_case_types)]

pub struct Object_3D{
    pub pivo: Point3,
    pub verteces: Vec<Point3>,
    pub faces: Vec<Triangle>,
    pub moving:bool,
    pub velocity:Vec3,
}


impl Object_3D{
    pub fn new()->Object_3D{
        let piv = Vec3::zero();
        let verteces = Vec::new();
        let faces = Vec::new();
        let vel = Vec3::zero();
        return Object_3D{pivo:piv,verteces:verteces,faces:faces,moving:false,velocity:vel};
    }

    pub fn merge_object3d(&mut self,new_obj:&mut Object_3D)->bool{
        if self.moving || new_obj.moving{
            println!("Cant merge moving objects!");
            return false;
        }
        self.verteces.append(&mut new_obj.verteces);
        self.faces.append(&mut new_obj.faces);
        let mut p = Vec3::zero();
        for vertex in self.verteces.iter(){ p += *vertex; }
        self.pivo = p/self.verteces.len() as f32;
        return true;
    }

    pub fn create_from_file(path:&str,mat: Material) -> Object_3D{
        let now = Instant::now();

        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        let mut verteces:Vec<Point3> = Vec::new();
        let mut triangles:Vec<Triangle> = Vec::new();

        for line in reader.lines(){
            let file_line = line.unwrap(); 
            if file_line.starts_with("v "){
                let v_str: Vec<_> = file_line.split_whitespace().collect();
                verteces.push(Point3 { e: [v_str[1].parse::<f32>().unwrap(), v_str[2].parse::<f32>().unwrap(), v_str[3].parse::<f32>().unwrap()] })
            }
            else if  file_line.starts_with("f ") {
                let f_str: Vec<_> = file_line.split_whitespace().collect();
                if f_str.len() == 4{
                    let mut face_ids:[u32;3] = [0,0,0];
                    for i in [1,2,3]{
                        let st: Vec<_> =  f_str[i].split('/').collect();
                        face_ids[i-1] = st[0].parse::<u32>().unwrap()-1;
                    }
                    let v1 = verteces[face_ids[0] as usize];
                    let v2 = verteces[face_ids[1] as usize];
                    let v3 = verteces[face_ids[2] as usize];
                    
                    let mut nomral = (v2-v1).cross(&(v3-v1));
                    nomral.norm();
                    triangles.push(Triangle{vertecies:[v1,v2,v3], normal:nomral,material:mat.clone()});
                }
                else if f_str.len() == 5{
                    let mut face_ids:[u32;4] = [0,0,0,0];
                    for i in [1,2,3,4]{
                        let st: Vec<_> =  f_str[i].split('/').collect();
                        face_ids[i-1] = st[0].parse::<u32>().unwrap()-1;
                    }
                    let v1 = verteces[face_ids[0] as usize];
                    let v2 = verteces[face_ids[1] as usize];
                    let v3 = verteces[face_ids[2] as usize];
                    let v4 = verteces[face_ids[3] as usize];

                    let mut nomral = (v2-v1).cross(&(v3-v1));
                    nomral.norm();
                    triangles.push(Triangle{vertecies:[v1,v2,v3], normal:nomral,material:mat.clone()});
                    triangles.push(Triangle{vertecies:[v1,v3,v4], normal:nomral,material:mat.clone()});
                }
            }
        }
        let mut x_min = f32::MAX;
        let mut y_min = f32::MAX;
        let mut z_min = f32::MAX;
        let mut x_max = f32::MIN;
        let mut y_max = f32::MIN;
        let mut z_max = f32::MIN;
        
        for vertex in verteces.iter(){
            if vertex.x() < x_min{x_min=vertex.x();}
            if vertex.y() < y_min{y_min=vertex.y();}
            if vertex.z() < z_min{z_min=vertex.z();}
            if vertex.x() > x_max{x_max=vertex.x();}
            if vertex.y() > y_max{y_max=vertex.y();}
            if vertex.z() > z_max{z_max=vertex.z();}
        }

        let mut pivo = Vec3::new((x_max+x_min)/2.0,(y_max+y_min)/2.0,(z_max+z_min)/2.0);

        println!("X_range: {}-{}, Y_range: {}-{}, Z_range: {}-{}",x_min,x_max,y_min,y_max,z_min,z_max);
        
        //println!("{}",pivo);
        
        let elapsed_time = now.elapsed();
        let s = elapsed_time.as_millis() as f32/1000.0;
        print!("\nLoaded Object with {} faces in {} seconds\n",triangles.len(),s);   
        return (Object_3D{pivo,verteces,faces:triangles,moving:false,velocity:Vec3::zero()});
    }
    
    pub fn add_movement(&mut self, velocity:Vec3){
        self.moving = true;
        self.velocity = velocity;
    }

    pub fn shift(&mut self, vec:Vec3){
        for vertex in self.verteces.iter_mut(){
            *vertex+=vec;
        }
        for triangle in self.faces.iter_mut(){
            triangle.shif(vec);
        }
        self.pivo = Vec3::new(0.0,0.0,0.0);
        
        for vertex in self.verteces.iter(){
            self.pivo+=*vertex;
        }
        self.pivo /= self.verteces.len() as f32;
    }
    
    pub fn rotate(&mut self,alpha: f32,betha:f32,gamma:f32){
        let p = self.pivo;
        self.shift(-p);

        for vertex in self.verteces.iter_mut(){
            
            vertex.rotate(alpha, betha, gamma);
        }
        for triangle in self.faces.iter_mut(){
            triangle.rotate(alpha, betha, gamma);
        }
        self.shift(p);
    }

    pub fn rotate_x(&mut self,angle:f32){
        let p = self.pivo;
        self.shift(-p);
        for vertex in self.verteces.iter_mut(){
            vertex.rotate_x(angle);
        }
        for face in self.faces.iter_mut(){
            face.rotate_x(angle);
        }
        self.shift(p);
    }
    
    pub fn rotate_y(&mut self,angle:f32){
        let p = self.pivo;
        self.shift(-p);
        for vertex in self.verteces.iter_mut(){
            vertex.rotate_y(angle);
        }
        for face in self.faces.iter_mut(){
            face.rotate_y(angle);
        }
        self.shift(p);
    }

    pub fn rotate_z(&mut self,angle:f32){
        let p = self.pivo;
        self.shift(-p);
        for vertex in self.verteces.iter_mut(){
            vertex.rotate_z(angle);
        }
        for face in self.faces.iter_mut(){
            face.rotate_z(angle);
        }
        self.shift(p);
    }

    pub fn scale(&mut self, scale_factor:f32){
        let p = self.pivo;
        self.shift(-p);

        for vertex in self.verteces.iter_mut(){
            *vertex*=scale_factor;
        }
        for triangle in self.faces.iter_mut(){
            for vertex in triangle.vertecies.iter_mut(){
                *vertex*=scale_factor;
            }
        }
        self.shift(p);
    }

    pub fn get_hittablelist(self)->HittableList{
        let mut hitlist =  HittableList{objects: Vec::new()};
        for face in self.faces.iter(){
            hitlist.add(Rc::new(face.clone()));
        }
        return hitlist;
    }
    
    
    pub fn creat_pyramide(mat:Material)->Object_3D{
        let v1 = Point3::new(-0.5,0.0,-0.5);
        let v2 = Point3::new(0.5,0.0,-0.5);
        let v3 = Point3::new(0.0,0.0,-1.5);
        let v4 = Point3::new(0.0,1.0,-1.0);
        let verteces:Vec<Point3> = vec![v1,v2,v3,v4];
        let mut pivo = Vec3::new(0.0,0.0,0.0);    
        for vertex in verteces.iter(){
            pivo+= *vertex;
        }
        pivo /= verteces.len() as f32;
        println!("Pivo: {}",pivo);
        
        let mut t1 = Triangle::new(v1,v2,v3,mat.clone());
        let mut t2 = Triangle::new(v1,v3,v4,mat.clone());
        let mut t3 = Triangle::new(v1,v2,v4,mat.clone());
        let mut t4 = Triangle::new(v2,v3,v4,mat.clone());
        let faces = vec![t1,t2,t3,t4];
        
        return(Object_3D{pivo,verteces,faces,moving:false,velocity:Vec3::zero()});
    }

    pub fn print_verteces(&self){
        println!("Verteces:");
        for vertex in self.verteces.iter(){
            println!("{}",vertex);
        }
    }

    pub fn create_box(p0:Point3,p1:Point3,mat: Material)->Object_3D{
        let mut box_new = new_xyrect(p0.x(), p1.x(), p0.y(), p1.y(), p1.z(), mat.clone(), !(p1.z()>0.0));
        box_new.merge_object3d(&mut new_xyrect(p0.x(), p1.x(), p0.y(), p1.y(), p0.z(), mat.clone(), (p0.z()>0.0)));

        box_new.merge_object3d(&mut new_xzrect(p0.x(), p1.x(), p0.z(), p1.z(), p1.y(), mat.clone(),!(p1.y()>0.0)));
        box_new.merge_object3d(&mut new_xzrect(p0.x(), p1.x(), p0.z(), p1.z(), p0.y(), mat.clone(),(p0.y()>0.0)));
        
        box_new.merge_object3d(&mut new_yzrect(p0.y(), p1.y(), p0.z(), p1.z(), p1.x(), mat.clone(), !(p1.x()>0.0)));
        box_new.merge_object3d(&mut new_yzrect(p0.y(), p1.y(), p0.z(), p1.z(), p0.x(), mat.clone(), (p0.x()>0.0)));

        return box_new;
    }
}