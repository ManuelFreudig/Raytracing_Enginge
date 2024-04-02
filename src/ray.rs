use super::vec3::{Vec3,Point3};

#[derive(Clone,Copy)]
pub struct Ray{
    pub orig: Point3,
    pub dir: Vec3,
    pub time: f32,
}

impl Ray{
    pub fn new(origin: Point3, direction: Vec3,time:f32)->Ray{
        Ray{orig:origin,dir:direction,time}
    }
    pub fn origin(self)->Point3{self.orig}
    pub fn direction(self)->Vec3{self.dir}
    pub fn time(self)->f32{self.time}

    pub fn at(self,t:f32)->Point3{
        self.orig + self.dir*t
    }
}