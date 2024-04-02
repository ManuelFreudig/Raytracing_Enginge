use crate::sphere;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

use super::hittable_list::HittableList;
use super::vec3::{Vec3,Point3,Color};
use super::vec3::{rand_vec3};
use super::polygon_object::Object_3D;
use super::material::Material;
use super::sphere::Sphere;
use std::f32::consts::PI;
use rand::{thread_rng,Rng};
use std::rc::Rc;
use super::bvh::BvhNode;
use super::texture::Texture;
use super::texture::{SolidColor,CheckerTexture,NoiseTexture,ImageTexture};
use super::aarect::{XYRect,new_xyrect,new_xzrect,new_yzrect};
use super::constant_medium::ConstantMedium;

pub struct Scene{
    pub world: HittableList,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vfov: f32,
    pub aparture: f32,
    pub time0:f32,
    pub time1:f32,
    pub background:Rc<dyn Texture>,
}

impl Scene {
    pub fn new(lookfrom:Point3, lookat:Point3, vfov: f32, aparture:f32, time0:f32,time1:f32,source_path:&str,gen_function: fn(f32,f32,&str)->HittableList, background:Rc<dyn Texture>)->Scene{
        Scene { world: gen_function(time0,time1,source_path), lookfrom, lookat, vfov, aparture, time0, time1,background}
    }
}


pub fn tow_spheres(time0: f32,time1:f32,source_path:&str)->HittableList{
    let mut objects = HittableList{objects: Vec::new()};
    let checker = Rc::new(CheckerTexture::new(Color{e:[0.2,0.3,0.1]}, Color{e:[0.9,0.9,0.9]}));
    objects.add(Rc::new(Sphere::new_stationary(Point3::new(0.0, -10.0, 0.0), 10.0, Material::new_lambertian(checker.clone()))));
    objects.add(Rc::new(Sphere::new_stationary(Point3::new(0.0, 10.0, 0.0), 10.0, Material::new_lambertian(checker))));
    return  objects;
}

pub fn tow_perlin_shperes(time0: f32,time1:f32,source_path:&str)->HittableList{
    let mut objects = HittableList{objects: Vec::new()};

    let pertext = Rc::new(NoiseTexture::new_scale(4.0));
    objects.add(Rc::new(Sphere::new_stationary(Point3::new(0.0,-1000.0,0.0),1000.0, Material::new_lambertian(pertext.clone()))));
    objects.add(Rc::new(Sphere::new_stationary(Point3::new(0.0,2.0,0.0),2.0, Material::new_lambertian(pertext.clone()))));

    return objects
} 

pub fn simple_light(time0:f32, time1:f32,source_path:&str)->HittableList{
    let mut objects = HittableList{objects: Vec::new()};

    let pertext = Rc::new(NoiseTexture::new_scale(4.0));
    objects.add(Rc::new(Sphere::new_stationary(Point3::new(0.0,-1000.0,0.0),1000.0, Material::new_lambertian(pertext.clone()))));
    objects.add(Rc::new(Sphere::new_stationary(Point3::new(0.0,2.0,0.0),2.0, Material::new_lambertian(pertext.clone()))));
    
    let difflight = Material::new_diffuse_light(Rc::new(SolidColor::new(Color{e:[4.0,4.0,4.0]})));
    let rect_light = new_xyrect(3.0,5.0,1.0,3.0,-2.0,difflight.clone(), true);
    objects.merge(rect_light.get_hittablelist());
    objects.add(Rc::new(Sphere::new_stationary(Point3::new(0.0,8.0,0.0),2.0, difflight)));

    return objects;

}

pub fn conrell_box(time0:f32, time1:f32,source_path:&str)->HittableList{
    let mut objects = HittableList{objects: Vec::new()};
    
    let red = Material::new_lambertian(Rc::new(SolidColor::new(Color{e:[0.65,0.05,0.05]})));
    let white = Material::new_lambertian(Rc::new(SolidColor::new(Color{e:[0.73,0.73,0.73]})));
    let green = Material::new_lambertian(Rc::new(SolidColor::new(Color{e:[0.12,0.45,0.15]})));
    let light = Material::new_diffuse_light(Rc::new(SolidColor::new(Color{e:[15.0,15.0,15.0]})));

    objects.merge(new_yzrect(0.0, 555.0, 0.0, 555.0, 555.0, green, true).get_hittablelist());
    objects.merge(new_yzrect(0.0, 555.0, 0.0, 555.0, 0.0, red, false).get_hittablelist());
    objects.merge(new_xzrect(213.0, 343.0, 227.0, 332.0, 554.0, light, true).get_hittablelist());
    objects.merge(new_xzrect(0.0, 555.0, 0.0, 555.0, 0.0, white.clone(), false).get_hittablelist());
    objects.merge(new_xzrect(0.0, 555.0, 0.0, 555.0, 555.0, white.clone(), true).get_hittablelist());
    objects.merge(new_xyrect(0.0, 555.0, 0.0, 555.0, 555.0, white, true).get_hittablelist());

    return objects;
}

pub fn cornell_box_with_boxes(time0:f32, time1:f32,source_path:&str)->HittableList{
    let mut objects = conrell_box(time0, time1, source_path);
    let white = Material::new_lambertian(Rc::new(SolidColor::new(Color{e:[0.73,0.73,0.73]})));
    let mut box1 = Object_3D::create_box(Point3{e:[130.0,0.0,65.0]}, Point3{e:[295.0,165.0,230.0]}, white.clone());
    let mut box2 = Object_3D::create_box(Point3{e:[265.0,0.0,295.0]}, Point3{e:[430.0,330.0,460.0]}, white);
    box1.rotate_y(-15.0*PI/180.0);
    box2.rotate_y(18.0*PI/180.0);
    
    objects.merge(box1.get_hittablelist());
    objects.merge(box2.get_hittablelist());

    return objects;
}

pub fn conrell_smoke(time0:f32, time1:f32,source_path:&str)->HittableList{
    let mut objects = HittableList{objects: Vec::new()};
    
    let red = Material::new_lambertian(Rc::new(SolidColor::new(Color{e:[0.65,0.05,0.05]})));
    let white = Material::new_lambertian(Rc::new(SolidColor::new(Color{e:[0.73,0.73,0.73]})));
    let green = Material::new_lambertian(Rc::new(SolidColor::new(Color{e:[0.12,0.45,0.15]})));
    let light = Material::new_diffuse_light(Rc::new(SolidColor::new(Color{e:[7.0,7.0,7.0]})));

    objects.merge(new_yzrect(0.0, 555.0, 0.0, 555.0, 555.0, green, true).get_hittablelist());
    objects.merge(new_yzrect(0.0, 555.0, 0.0, 555.0, 0.0, red, false).get_hittablelist());
    objects.merge(new_xzrect(113.0, 443.0, 127.0, 432.0, 554.0, light, true).get_hittablelist());
    objects.merge(new_xzrect(0.0, 555.0, 0.0, 555.0, 0.0, white.clone(), false).get_hittablelist());
    objects.merge(new_xzrect(0.0, 555.0, 0.0, 555.0, 555.0, white.clone(), true).get_hittablelist());
    objects.merge(new_xyrect(0.0, 555.0, 0.0, 555.0, 555.0, white.clone(), true).get_hittablelist());

    let mut box1 = Object_3D::create_box(Point3{e:[130.0,0.0,65.0]}, Point3{e:[295.0,165.0,230.0]}, white.clone());
    let mut box2 = Object_3D::create_box(Point3{e:[265.0,0.0,295.0]}, Point3{e:[430.0,330.0,460.0]}, white);
    box1.rotate_y(-15.0*PI/180.0);
    box2.rotate_y(18.0*PI/180.0);

    objects.add(Rc::new(ConstantMedium::new(Rc::new(box1.get_hittablelist()), 0.01, Rc::new(SolidColor::new(Color{e:[0.0,0.0,0.0]})))));
    objects.add(Rc::new(ConstantMedium::new(Rc::new(box2.get_hittablelist()), 0.01, Rc::new(SolidColor::new(Color{e:[1.0,1.0,1.0]})))));

    return objects;
}


pub fn earth(time0: f32,time1:f32,source_path:&str)->HittableList{
    let earth_texture = Rc::new(ImageTexture::new(&(source_path.clone().to_owned() + "\\skyboxes\\earthmap.jpg")));
    let earth_surface = Material::new_lambertian(earth_texture);
    let globe = Sphere::new_stationary(Point3{e:[0.0,0.0,0.0]}, 2.0, earth_surface);

    let mut objects = HittableList{objects: Vec::new()};
    objects.add(Rc::new(globe));
    return objects;
}

pub fn ramdom_scene(time0: f32,time1:f32,source_path:&str) -> HittableList{
    let mut rn = thread_rng();

    let mut world =  HittableList{objects: Vec::new()};
    
    //Add Ground
    let c1 = Color::new(0.2,0.3,0.1);
    let c2 = Color::new(0.9, 0.9, 0.9);
    let groud_material = Material::new_lambertian(Rc::new(CheckerTexture::new(c1, c2)));
    world.add(Rc::new(Sphere::new_stationary(Vec3 { e: [0.0,-1000.0,0.0] }, 1000.0, groud_material)));

    let material1 = Material::new_dielectric(1.5);
    world.add(Rc::new(Sphere::new_stationary(Point3{e:[0.0,1.0,0.0]}, 1.0, material1)));

    let material2 = Material::new_lambertian(Rc::new(SolidColor::new(Color{e:[0.4,0.2,0.1]})));
    world.add(Rc::new(Sphere::new_stationary(Point3{e:[-4.0,1.0,0.0]},1.0,material2)));

    let material3 = Material::new_metal(Color{e:[0.7,0.6,0.5]}, 0.0);
    world.add(Rc::new(Sphere::new_stationary(Point3{e:[4.0,1.0,0.0]},1.0,material3)));

    for a in (-11..11){
        for b in (-11..11){
            let choose_mat:f32 = rn.gen();
            let mut  r1:f32 = rn.gen(); r1*=0.9;
            let mut r2:f32 = rn.gen();r2*=0.9;
            let center = Point3::new(a as f32 + r1,0.2,b as f32 + r2);
            
            if((center - Point3{e:[4.0,0.2,0.0]}).length_sq() > 0.9){
                if choose_mat < 0.8{
                    //diffuse
                    let albedo1:Color = rn.gen(); let albedo2:Color = rn.gen();
                    let albedo = albedo1*albedo2;
                    let sphere_material = Material::new_lambertian(Rc::new(SolidColor::new(albedo)));
                    let velocity = Vec3::new(0.0, rn.gen_range(0.0..0.3) as f32, 0.0);
                    //world.add(Rc::new(Sphere::new_moving(center,0.2,sphere_material,velocity)));
                    world.add(Rc::new(Sphere::new_stationary(center,0.2,sphere_material)));
                }
                else if choose_mat < 0.95 {
                    //metal
                    let albedo = rand_vec3(0.0, 0.5, &mut rn);
                    let fuzz:f32 = rn.gen_range(0.0..0.5);
                    let sphere_material = Material::new_metal(albedo, fuzz);
                    world.add(Rc::new(Sphere::new_stationary(center,0.2,sphere_material)));
                }
                else{
                    //glass
                    let sphere_material = Material::new_dielectric(1.5);
                    world.add(Rc::new(Sphere::new_stationary(center,0.2,sphere_material)));
                }
            }
        }
    }

    return world;
}



pub fn pokal_on_surface(source_path:&str) -> HittableList{
    let mut world =  HittableList{objects: Vec::new()};

    //Define Materials
    let material_ground = Material::new_lambertian(Rc::new(SolidColor::new(Color::new(0.8,0.8,0.0))));
    let pokal_material = Material::new_metal(Color::new(0.8,0.8,0.0),0.1);

     //Load Pokal
     let file_name = "PokalFlex";
     let obj_path = format!("{}/Object_files/{}.obj",source_path,file_name);
     let mut pokal = Object_3D::create_from_file(&obj_path,pokal_material);
     pokal.shift(Vec3 { e: [0.0,0.0,-3.0] });
     pokal.rotate(0.0, PI/2.0, PI/2.0);

    
    //Create Scene
    world.add(Rc::new(Sphere::new_stationary(Vec3 { e: [0.0,-101.5,-1.0] }, 100.0, material_ground)));
    world.merge(pokal.get_hittablelist());

    return world;
}

pub fn random_scene_with_polygon_objectes(source_path:&str)->HittableList{
    let mut rn = thread_rng();

    let mut world =  HittableList{objects: Vec::new()};

    



    //Add Ground
    let groud_material = Material::new_lambertian(Rc::new(SolidColor::new(Color::new(0.5,0.5,0.5))));
    world.add(Rc::new(Sphere::new_stationary(Vec3 { e: [0.0,-1000.0,0.0] }, 1000.0, groud_material)));

    for a in (-11..11){
        for b in (-11..11){
            let choose_mat:f32 = rn.gen();
            let mut  r1:f32 = rn.gen(); r1*=0.9;
            let mut r2:f32 = rn.gen();r2*=0.9;
            let center = Point3::new(a as f32 + r1,0.2,b as f32 + r2);
            
            if((center - Point3{e:[4.0,0.2,0.0]}).length_sq() > 0.9){
                if choose_mat < 0.8{
                    //diffuse
                    let albedo1:Color = rn.gen(); let albedo2:Color = rn.gen();
                    let albedo = albedo1*albedo2;
                    let sphere_material = Material::new_lambertian(Rc::new(SolidColor::new(albedo)));
                    world.add(Rc::new(Sphere::new_stationary(center,0.2,sphere_material)));
                }
                else if choose_mat < 0.95 {
                    //metal
                    let albedo = rand_vec3(0.0, 0.5, &mut rn);
                    let fuzz:f32 = rn.gen_range(0.0..0.5);
                    let sphere_material = Material::new_metal(albedo, fuzz);
                    world.add(Rc::new(Sphere::new_stationary(center,0.2,sphere_material)));
                }
                else{
                    //glass
                    let sphere_material = Material::new_dielectric(1.5);
                    world.add(Rc::new(Sphere::new_stationary(center,0.2,sphere_material)));
                }
            }
        }
    }
    let material1 = Material::new_dielectric(1.5);
    let file_name = "teapot";
    let obj_path = format!("{}\\Object_files\\{}.obj",source_path,file_name);
    let mut pokal = Object_3D::create_from_file(&obj_path,material1);
    pokal.shift(Vec3 { e: [0.0,-1.0,0.0] });
    pokal.rotate(0.0, 0.0, 0.0);
    pokal.scale(0.5);
    world.merge(pokal.get_hittablelist());

    let material2 = Material::new_lambertian(Rc::new(SolidColor::new(Color{e:[0.4,0.2,0.1]})));
    world.add(Rc::new(Sphere::new_stationary(Point3{e:[-4.0,1.0,0.0]},1.0,material2)));

    //Load Pokal
    let material3 = Material::new_metal(Color{e:[0.7,0.6,0.5]}, 0.0);
    let file_name = "PokalFlex";
    let obj_path = format!("{}\\Object_files\\{}.obj",source_path,file_name);
    let mut pokal = Object_3D::create_from_file(&obj_path,material3);
    pokal.shift(Vec3 { e: [4.0,1.0,-1.0] });
    pokal.rotate(0.0, PI/2.0, PI/2.0);
    world.merge(pokal.get_hittablelist());



    return world;   
}



pub fn random_with_bvh(time0:f32, time1:f32,source_path:&str)->HittableList{
    let objects = ramdom_scene(time0,time1,"");
    let mut world = BvhNode::empty();
    world.bvhnode(&objects.objects, 0, objects.len(), time0, time1);
    let mut h_list = HittableList{objects: Vec::new()};
    h_list.add(Rc::new(world));
    return(h_list);
}



pub fn bvh_scene_with_polygon_objectes(source_path:&str,time0:f32, time1:f32)->BvhNode{
    let mut rn = thread_rng();

    let mut random_spheres =  HittableList{objects: Vec::new()};

    //Add Ground
    let groud_material = Material::new_lambertian(Rc::new(SolidColor::new(Color::new(0.5,0.5,0.5))));
    random_spheres.add(Rc::new(Sphere::new_stationary(Vec3 { e: [0.0,-1000.0,0.0] }, 1000.0, groud_material)));

    for a in (-11..11){
        for b in (-11..11){
            let choose_mat:f32 = rn.gen();
            let mut  r1:f32 = rn.gen(); r1*=0.9;
            let mut r2:f32 = rn.gen();r2*=0.9;
            let center = Point3::new(a as f32 + r1,0.2,b as f32 + r2);
            
            if((center - Point3{e:[4.0,0.2,0.0]}).length_sq() > 0.9){
                if choose_mat < 0.8{
                    //diffuse
                    let albedo1:Color = rn.gen(); let albedo2:Color = rn.gen();
                    let albedo = albedo1*albedo2;
                    let sphere_material = Material::new_lambertian(Rc::new(SolidColor::new(albedo)));
                    random_spheres.add(Rc::new(Sphere::new_stationary(center,0.2,sphere_material)));
                }
                else if choose_mat < 0.95 {
                    //metal
                    let albedo = rand_vec3(0.0, 0.5, &mut rn);
                    let fuzz:f32 = rn.gen_range(0.0..0.5);
                    let sphere_material = Material::new_metal(albedo, fuzz);
                    random_spheres.add(Rc::new(Sphere::new_stationary(center,0.2,sphere_material)));
                }
                else{
                    //glass
                    let sphere_material = Material::new_dielectric(1.5);
                    random_spheres.add(Rc::new(Sphere::new_stationary(center,0.2,sphere_material)));
                }
            }
        }
    }
    
    let mut bvn_rnspheres = BvhNode::empty();
    bvn_rnspheres.bvhnode(&random_spheres.objects, 0, random_spheres.len(), time0, time1);

    let material1 = Material::new_dielectric(1.5);
    let file_name = "teapot";
    let obj_path = format!("{}\\Object_files\\{}.obj",source_path,file_name);
    let mut pokal = Object_3D::create_from_file(&obj_path,material1);
    pokal.shift(Vec3 { e: [0.0,-1.0,0.0] });
    pokal.rotate(0.0, 0.0, 0.0);
    pokal.scale(0.5);
    let mut bvn_teapot = BvhNode::empty();
    let teapot_hitlist = pokal.get_hittablelist();
    bvn_teapot.bvhnode(&teapot_hitlist.objects, 0, teapot_hitlist.len(), time0, time1);

    let material2 = Material::new_lambertian(Rc::new(SolidColor::new(Color{e:[0.4,0.2,0.1]})));
    let mut world = HittableList{objects: Vec::new()};
    world.add(Rc::new(Sphere::new_stationary(Point3{e:[-4.0,1.0,0.0]},1.0,material2)));

    //Load Pokal
    let material3 = Material::new_metal(Color{e:[0.7,0.6,0.5]}, 0.0);
    let file_name = "PokalFlex";
    let obj_path = format!("{}\\Object_files\\{}.obj",source_path,file_name);
    let mut pokal = Object_3D::create_from_file(&obj_path,material3);
    pokal.shift(Vec3 { e: [4.0,1.0,-1.0] });
    pokal.rotate(0.0, PI/2.0, PI/2.0);
    let mut bvn_pokal = BvhNode::empty();
    let pokal_hitlist = pokal.get_hittablelist();
    bvn_pokal.bvhnode(&pokal_hitlist.objects, 0, pokal_hitlist.len(), time0, time1);
    
    world.add(Rc::new(bvn_rnspheres));
    world.add(Rc::new(bvn_teapot));
    world.add(Rc::new(bvn_pokal));

    let mut bvh_world = BvhNode::empty();
    bvh_world.bvhnode(&world.objects, 0, world.len(), time0, time1);

    return bvh_world;
}


pub fn final_scene(time0: f32,time1:f32,source_path:&str)->HittableList{
    let mut rn = thread_rng();
    let file = File::open(&(source_path.clone().to_owned() +"\\y_heights\\y-heigts.scn")).unwrap();
    let reader = BufReader::new(file);
    let mut y_heights:Vec<f32> = Vec::new();
    for line in reader.lines(){
        y_heights.push(line.unwrap().parse::<f32>().expect("Coudn't read y-value"));
    }

    let mut objects = HittableList{objects: Vec::new()};
    let mut boxes1 = Object_3D::new();
    let ground = Material::new_lambertian(Rc::new(SolidColor::new(Color{e:[0.48,0.83,0.53]})));


    let boxes_per_side:usize = 20;
    for i in (0..boxes_per_side){
        for j in (0..boxes_per_side){
            let w:f32 = 100.0;
            let x0:f32 = -1000.0 + i as f32*w;
            let z0:f32 = -1000.0 + j as f32*w;
            let y0:f32 = 0.0;
            let x1 = x0 + w;
            let y1:f32 = y_heights[i*20 + j];
            let z1 = z0 + w;
            let mut box_temp = Object_3D::create_box(Point3{e:[x0,y0,z0]}, Point3{e:[x1,y1,z1]}, ground.clone());
            boxes1.merge_object3d(&mut box_temp);
        }
    }
    boxes1.rotate_y(1.0*PI/180.0);
    let mut boxes1_hitlist = boxes1.get_hittablelist();
    
    let mut boxes1_bvh = BvhNode::empty();
    boxes1_bvh.bvhnode(&boxes1_hitlist.objects, 0, boxes1_hitlist.len(), time0, time1);
    objects.add(Rc::new(boxes1_bvh));
    
    //Light
    let light = Material::new_diffuse_light(Rc::new(SolidColor::new(Color::new(7.0,7.0,7.0))));
    objects.merge(new_xzrect(123.0, 423.0, 147.0, 412.0, 554.0, light, true).get_hittablelist());

    //moving Sphere
    let center1 = Point3{e:[400.0,400.0,200.0]};
    let velocity = Vec3 { e: [30.0,0.0,0.0] };
    let mooving_shpere_material = Material::new_lambertian(Rc::new(SolidColor::new(Color{e:[0.7,0.3,0.1]})));
    objects.add(Rc::new(Sphere::new_moving(center1, 50.0, mooving_shpere_material, velocity)));

    //Glas and Metal Spheres
    objects.add(Rc::new(Sphere::new_stationary(Point3{e:[260.0,150.0,45.0]}, 50.0, Material::new_dielectric(1.5))));
    objects.add(Rc::new(Sphere::new_stationary(Point3{e:[0.0,150.0,145.0]}, 50.0, Material::new_metal(Color{e:[0.8,0.8,0.8]}, 0.1))));

    //Glas Sphere with Smoke
    let mut boundary = Sphere::new_stationary(Point3{e:[360.0,150.0,145.0]}, 70.0, Material::new_dielectric(1.5));
    objects.add(Rc::new(boundary.clone()));
    objects.add(Rc::new(ConstantMedium::new(Rc::new(boundary.clone()), 0.2, Rc::new(SolidColor::new(Color{e:[0.2,0.4,0.9]})))));
    //boundary = Sphere::new_stationary(Point3::zero(), 5000.0, Material::new_dielectric(1.5));
    //objects.add(Rc::new(ConstantMedium::new(boundary,0.0001,Rc::new(SolidColor::new(Color{e:[1.0,1.0,1.0]})))));

    //Earth and Perlin
    let emat = Rc::new(ImageTexture::new(&(source_path.clone().to_owned() + "\\skyboxes\\earthmap.jpg")));
    objects.add(Rc::new(Sphere::new_stationary(Point3{e:[400.0,200.0,400.0]}, 100.0, Material::new_lambertian(emat))));
    let pertext = Rc::new(NoiseTexture::new_scale(0.1));
    objects.add(Rc::new(Sphere::new_stationary(Point3{e:[220.0,280.0,300.0]}, 80.0, Material::new_lambertian(pertext))));
    
    let mut boxes2 = HittableList{objects: Vec::new()};
    let white = Material::new_lambertian(Rc::new(SolidColor::new(Color{e:[0.73,0.73,0.73]})));
    let ns:usize = 1000;
    for j in (0..ns){
        let center = rand_vec3(0.0, 165.0, &mut rn) + Vec3 { e: [-100.0,270.0,395.0] };
        boxes2.add(Rc::new(Sphere::new_stationary(center, 10.0, white.clone())));
    }
    let mut boxes2_bvh = BvhNode::empty();
    boxes2_bvh.bvhnode(&boxes2.objects, 0, boxes2.len(), time0, time1);

    objects.add(Rc::new(boxes2_bvh));

    return objects
}


