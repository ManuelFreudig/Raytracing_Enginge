use std::rc::Rc;
use super::vec3::{Color,Point3,Vec3};
use super::perlin::Perlin;
use image;
use image::GenericImageView;


pub trait Texture:CloneTexture{
    fn value(&self,u:f32,v:f32, p:Point3)->Color;
}
pub trait CloneTexture{
    fn clone_texture(&self) ->Rc<dyn Texture>;
}

impl<T> CloneTexture for T 
where 
    T: 'static + Texture + Clone,{
    fn clone_texture(&self)-> Rc<dyn Texture>{
        Rc::new(self.clone())
    }
}
#[derive(Clone)]
pub struct SolidColor{
    color_value:Color,
}

impl SolidColor{
    pub fn new(c:Color)->SolidColor{SolidColor { color_value: c }}
}

impl Texture for SolidColor{
    fn value(&self,u:f32,v:f32, p:Vec3)->Color {return self.color_value;}
}

#[derive(Clone)]
pub struct CheckerTexture{
    odd: Rc<dyn Texture>,
    even: Rc<dyn Texture>,
}
impl CheckerTexture{
    pub fn new(c1:Color,c2:Color)->CheckerTexture{
        let even = Rc::new(SolidColor::new(c1));
        let odd = Rc::new(SolidColor::new(c2));
        CheckerTexture {odd, even}
    }
}

impl Texture for CheckerTexture {
    fn value(&self,u:f32,v:f32, p:Vec3)->Color {
        let sins = (10.0*p.x()).sin()*(10.0*p.y()).sin()*(10.0*p.z()).sin();
        if sins < 0.0 {self.odd.value(u, v, p)}
        else{self.even.value(u, v, p)}
    }
}

#[derive(Clone)]
pub struct NoiseTexture{
    noise:Perlin,
    scale:f32,
}
impl NoiseTexture {
    pub fn new()->NoiseTexture{ NoiseTexture { noise: Perlin::generate(), scale:1.0 } }
    pub fn new_scale(sc:f32)->NoiseTexture{ NoiseTexture{noise: Perlin::generate(), scale:sc}}
}

impl Texture for NoiseTexture{
    fn value(&self,u:f32,v:f32, p:Vec3)->Vec3 {
        return Color::new(1.0,1.0,1.0) * 0.5 * (1.0 + (self.scale*p.z() + 10.0*self.noise.turb(p*self.scale, 7)).sin());
    }
}

#[derive(Clone)]
pub struct ImageTexture{
    pub width:u32,
    pub height: u32,
    pixels: Vec<Vec<[u8;3]>>,
}

impl ImageTexture{
    pub fn new(file:&str)->ImageTexture{
        let img = image::open(file).expect("File not found!");
        let (width,height) = img.dimensions();
        println!("{},{}",width,height);
        let mut img_array = vec![vec![[0 as u8,0,0]; height as usize];width as usize];

        for (w, h, p)  in img.pixels(){
            img_array[w as usize][h as usize] = [p[0],p[1],p[2]];
        }
        ImageTexture { width, height, pixels: img_array}
    }
}

impl Texture for ImageTexture{
    fn value(&self,u:f32,v:f32, p:Color)->Color {
        let mut _u = u;
        let mut _v = v;
        _u.clamp(0.0, 1.0);
        let mut k = 1.0 - _v.clamp(0.0, 1.0);
        //println!("{},{}",self.width,self.height);
        let w = _u * (self.width as f32 - 1.0);
        let h = k *(self.height as f32 - 1.0);
        //println!("{},{}",_u,k);
        //println!("{},{}",w,h);
        let col = self.pixels[w as usize][h as usize];
        //println!("{:?}",col);
        return(Color::new(col[0] as f32/255.0,col[1] as f32/255.0,col[2] as f32/255.0));
    }
}