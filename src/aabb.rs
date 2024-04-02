use super::vec3::Point3;
use super::ray::Ray;

#[derive(Debug,Copy,Clone)]
pub struct AABB{
    pub minimum: Point3,
    pub maximum: Point3,
}

impl AABB{
    pub fn zero()->AABB{
        AABB { minimum: Point3::zero(), maximum: Point3::zero() }
    }
    pub fn new(a: &Point3, b: &Point3)->AABB{
        AABB { minimum:*a, maximum:*b }
    }
    pub fn hit(&self, r:&Ray, t_min:&mut f32, t_max:&mut f32)->bool{
        for a in (0..3){
            let invD = 1.0/r.dir[a];
            let (mut t0,mut t1) = ( (self.minimum[a] - r.orig[a])*invD, (self.maximum[a] - r.orig[a])*invD);
            if invD < 0.0{(t0,t1) = (t1,t0);}
            *t_min = if t0 > *t_min {t0} else {*t_min};
            *t_max = if t1 < *t_max {t1} else {*t_max};
            if (t_max <= t_min){
                return false;
            }
        }

        return true;
    }
}  

pub fn surronding_box(box0:&mut AABB, box1:&mut AABB)->AABB{
    let small = Point3::new(box0.minimum.x().min(box1.minimum.x()),
                                  box0.minimum.y().min(box1.minimum.y()),
                                  box0.minimum.z().min(box1.minimum.z()));
    let big = Point3::new(box0.maximum.x().max(box1.maximum.x()),
                                box0.maximum.y().max(box1.maximum.y()),
                                box0.maximum.z().max(box1.maximum.z()));
    AABB { minimum:small, maximum: big }
}