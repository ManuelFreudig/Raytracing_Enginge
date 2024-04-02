#![allow(unused)]
use std::any::Any;
use std::fmt::format;
use std::io;
use rand::{thread_rng,Rng};
use std::io::{Write,BufReader,BufRead,ErrorKind};
use std::fs::File;
use std::clone::Clone;
use std::boxed::Box;
use std::time::{self, Instant};
use std::f32::consts::PI;
use std::f32::{};
use std::path::Path;
use std::ffi::OsStr;
use image;
use image::GenericImageView;
use std::rc::Rc;

mod vec3;
mod image_writer;
mod ray;
mod hittable;
mod sphere;
mod triangle;
mod hittable_list;
mod camera;
mod polygon_object;
mod material;
mod create_scene;
mod aabb;
mod bvh;
mod texture;
mod perlin;
mod aarect;
mod constant_medium;

use vec3::{Vec3,Point3,Color,rand_vec3,random_in_unit_sphere, random_unit_vector};
use image_writer::Image;
use ray::Ray;
use hittable::{HitRecord,Hittable};
use sphere::Sphere;
use hittable_list::HittableList;
use camera::Camera;
use polygon_object::Object_3D;
use triangle::Triangle;
use material::Material;
use texture::Texture;
use texture::{SolidColor,ImageTexture};
use create_scene::Scene;
use create_scene::{pokal_on_surface,ramdom_scene,random_scene_with_polygon_objectes,random_with_bvh,tow_spheres,tow_perlin_shperes,earth,simple_light};

use crate::create_scene::{bvh_scene_with_polygon_objectes, conrell_box,cornell_box_with_boxes,conrell_smoke,final_scene};




fn ray_color(r: Ray, world: &mut Box<dyn Hittable>, background: Rc<dyn Texture>, depth:u16)->Color{
    
    if depth <= 0{ return Color::zero() }
    
    let mut rn = rand::thread_rng();
    let mut rec = &mut HitRecord::new();
    if (!world.hit(r,0.001,f32::MAX,&mut rec)){
        let mut unit_direction = -r.direction();
        unit_direction.norm();
        let theta = unit_direction.y().acos();
        let phi = unit_direction.z().atan2(unit_direction.x()) + PI;
        let u = phi/(2.0*PI);;
        let v = theta/PI;
        return background.value(u, v, unit_direction);
    }
    
    let mut scattered = Ray::new(Vec3::zero(),Vec3::zero(),r.time);
    let mut attenuation = Color::zero();
    let mut emitted = rec.material.emitted(rec.u, rec.v, &rec.p);
    if(!rec.material.scatter(&r,&rec,&mut attenuation,&mut scattered)){
        return emitted
    }
    
    return emitted + attenuation * ray_color(scattered, world, background, depth-1);
    

}



fn main() {
    let source_path = "C:\\Users\\Manuel Freudig\\source\\Rust\\Willi\\Willi_git";

    let now = Instant::now();
    
    //Load Pokal
    let file_name = "PokalFlex";
    let obj_path = format!("{}\\Object_files\\{}.obj",source_path,file_name);
    let mut pokal = Object_3D::create_from_file(&obj_path,Material::new_metal(Color::new(0.8,0.8,0.0),0.1));
    pokal.shift(Vec3 { e: [0.0,0.0,-3.0] });
    pokal.rotate(0.0, PI/2.0, PI/2.0);

    //Image
    let aspect_ratio = 16.0/16.0;
    let image_width:u16 = 800;
    let image_height = (image_width as f32/aspect_ratio) as u16;
    let samples_per_pixel: u16 = 200;
    let max_depth: u16 = 50;

    //Load Background image
    
    
    let bg_name = "starmap_2020_4k_gal_print.jpg";
    let bg_path = format!("{}/skyboxes/{}",source_path,bg_name);
    let bg_texture = ImageTexture::new(&bg_path);


    //Scene
    let scene1 = Scene::new(Point3::new(13.0,2.0,3.0),
                                     Point3::new(0.0,0.0,0.0),
                                    20.0,0.1,
                                    0.0,1.0,source_path,
                                    ramdom_scene,Rc::new(SolidColor::new(Color{e:[0.7,0.8,1.0]})));

    let scene2 = Scene::new(Point3::new(13.0,2.0,3.0),
                                     Point3::new(0.0,0.0,0.0),
                                    20.0,0.0,
                                    0.0,1.0,source_path,
                                    tow_spheres,Rc::new(SolidColor::new(Color{e:[0.7,0.8,1.0]})));
    
    let scene3 = Scene::new(Point3::new(13.0,2.0,3.0),
                                    Point3::new(0.0,0.0,0.0),
                                   20.0,0.0,
                                   0.0,1.0,source_path,
                                   tow_perlin_shperes,Rc::new(SolidColor::new(Color{e:[0.7,0.8,1.0]})));
    let scene4 = Scene::new(Point3::new(13.0,2.0,3.0),
                                   Point3::new(0.0,0.0,0.0),
                                   50.0,0.0,
                                   0.0,1.0,source_path,
                                   earth,Rc::new(bg_texture));
    let scene5 = Scene::new(Point3::new(26.0,3.0,6.0),
                                   Point3::new(0.0,2.0,0.0),
                                   20.0,0.0,
                                   0.0,1.0,source_path,
                                   simple_light,Rc::new(SolidColor::new(Color{e:[0.0,0.0,0.0]})));
    let scene6 = Scene::new(Point3::new(278.0,278.0,-800.0),
                                   Point3::new(278.0,278.0,0.0),
                                   40.0,0.0,
                                   0.0,1.0,source_path,
                                   cornell_box_with_boxes,Rc::new(SolidColor::new(Color{e:[0.0,0.0,0.0]})));
    let scene7 = Scene::new(Point3::new(278.0,278.0,-800.0),
                                   Point3::new(278.0,278.0,0.0),
                                   40.0,0.0,
                                   0.0,1.0,source_path,
                                   conrell_smoke,Rc::new(SolidColor::new(Color{e:[0.0,0.0,0.0]})));
    let scene8 = Scene::new(Point3::new(478.0,278.0,-600.0),
                                   Point3::new(278.0,278.0,0.0),
                                   40.0,0.0,
                                   0.0,1.0,source_path,
                                   final_scene,Rc::new(SolidColor::new(Color{e:[0.0,0.0,0.0]})));

    let scene = scene8;

    
    //Camera
    
    let v_up = Vec3::new(0.0,1.0,0.0);
    let dist_to_focus = 10.0;
    
    let cam = Camera::new(scene.lookfrom,scene.lookat,v_up,scene.vfov,aspect_ratio, scene.aparture,dist_to_focus,scene.time0,scene.time1);

    //Render
    let mut img = Image::new(image_width,image_height,samples_per_pixel,"final_scene");
    let mut b_world:Box<dyn Hittable> = Box::new(scene.world);
    let mut rng = thread_rng();
    for j in (0..img.height).rev(){
        let new_now = Instant::now();
        let t_till_now = new_now.saturating_duration_since(now).as_secs() as f32;
        let time_per_line = t_till_now/( img.height - j) as f32;
        let predicted_time = time_per_line * j as f32;
        println!("\rRendered: {}/{} lines with {:.2} seconds per line",img.height - j,img.height, time_per_line);
        println!("Predicted time {}    time left: {}",format_time(time_per_line*image_height as f32),format_time(predicted_time));
        for i in 0..img.width{
            let mut pixel_color = Color::new(0.0,0.0,0.0);
            for s in 0..samples_per_pixel{
                let ur:f32 = rng.gen();
                let vr:f32 = rng.gen();
                let u = (i as f32 + ur as f32)/(image_width-1) as f32;
                let v = (j as f32 + vr as f32)/(image_height-1) as f32;
                let r = cam.get_ray(u,v);
                pixel_color += ray_color(r, &mut b_world, scene.background.clone(), max_depth);
            }
            img.write_color(pixel_color);
        }
    }
    let elapsed_time = now.elapsed();
    let mut s = elapsed_time.as_millis() as f32/1000.0;
    let time = seconds_to_hours(s);
    print!("\nDone in {}\n",time);    
}


fn seconds_to_hours(mut s:f32)->String{
    let mut m = (s/60.0) as u64;
    let mut h = m/60;
    s -= (60*m) as f32;
    m -= 60*h;
    return format!("{}h {}m {}s",h,m,s as i16)

}


fn format_time(mut s:f32)->String{
    let mut m = (s/60.0) as u64;
    let mut h = m/60;
    s -= (60*m) as f32;
    m -= 60*h;
    return format!("{}h {}m {:.2}s",h,m,s)
}