use std::fs::File;
use std::io::Write;
use super::vec3::Color;


fn clamp(x:f32,min:f32,max:f32) -> f32{
    if(x<min){return min;}
    else if (x>max) {return max;}
    x
}
pub struct Image{
    pub width: u16,
    pub height: u16,
    pub path: String,
    pub output: File,
    pub samples_per_pixel: u16,
}

impl Image {
    pub fn new(width: u16, height: u16,samples_per_pixel:u16, name:&str)->Image{
        let mut path = String::from("C:\\Users\\Manuel Freudig\\source\\Rust\\Willi\\Willi_git\\images\\");
        path.push_str(name);
        path.push_str(".ppm");
        let output = File::create(&path);
        let mut output = match output {
            Ok(file) => file,
            Err(error) => panic!("Problem creating file: {:?}",error),
        };
        write!(output,"P3\n#samples: {}\n{} {}\n255\n",samples_per_pixel,width,height).expect("Failed to write to file");
        Image{width:width,height:height,path:path,output:output,samples_per_pixel:samples_per_pixel}
        }
        
    pub fn write_color(&mut self,pixel_color: Color){
        let mut r = pixel_color.x();
        let mut g = pixel_color.y();
        let mut b = pixel_color.z();

        // Divide the olor by the number of samples and gamma-correct for gamma = 2.0
        let scale = 1.0/self.samples_per_pixel as f32;
        r = (scale * r).sqrt();
        g = (scale * g).sqrt();
        b = (scale * b).sqrt();

        write!(&mut self.output,"{} {} {}\n",(255.999*clamp(r, 0.0, 0.999)) as u8,
                                   (255.999*clamp(g, 0.0, 0.999)) as u8,
                                   (255.999*clamp(b, 0.0, 0.999)) as u8)
        .expect("Failed to write Color to file");
    }

}


