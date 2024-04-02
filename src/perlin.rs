use rand::{thread_rng,Rng};
use super::vec3::{Point3,Vec3};
const point_count:usize = 256;

#[derive(Clone)]
pub struct Perlin{ 
    pub ranfloat: [Vec3;point_count],
    pub perm_x: [i64;point_count],
    pub perm_y: [i64;point_count],
    pub perm_z: [i64;point_count],
}


impl Perlin {
    pub fn generate()->Perlin{
        let mut per = Perlin{ranfloat:[Vec3::zero();point_count],perm_x:[0;point_count],perm_y:[0;point_count],perm_z:[0;point_count]};
        let mut rn = thread_rng();
        for i in (0..point_count){
            per.ranfloat[i] = rn.gen();
        }
        per.perm_x = per.generate_perm();
        per.perm_y = per.generate_perm();
        per.perm_z = per.generate_perm();
        return(per);
    }

    pub fn noise(&self,p: Point3) -> f32{
        let mut u = p.x() - p.x().floor();
        let mut v = p.y() - p.y().floor();
        let mut w = p.z() - p.z().floor();
        

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;
        let mut c:[[[Vec3;2];2];2] = [[[Vec3::zero();2];2];2];
        for di in (0..2){
            for dj in (0..2){
                for dk in (0..2){
                    c[di][dj][dk] = self.ranfloat[
                        (self.perm_x[((i+di as i32) as i16 & 255) as usize] ^
                        self.perm_y[((j+dj as i32) as i16 & 255) as usize] ^
                        self.perm_z[((k+dk as i32) as i16 & 255) as usize]) as usize
                    ];
                }
            }
        }

        
        return self.perlin_interp(c,u,v,w);
    }

    pub fn turb(&self,p: Point3,depth:i16)->f32{
        let mut accum:f32 = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;

        for i in (0..depth){
            accum += weight*self.noise(temp_p);
            weight *= 0.5;
            temp_p*=2.0;
        }

        return accum.abs();
    }

    fn generate_perm(&self)->[i64;point_count]{
        let mut p:[i64;point_count] = [0;point_count];
        for i in (0..point_count){
            p[i] = i as i64;
        } 
        self.permutate(&mut p,point_count);
        return p;
    }
    fn permutate(&self,p: &mut [i64;point_count], n:usize){
        let mut rn = thread_rng();
        for i in (1..n).rev(){
            let target:usize = rn.gen_range(0..point_count);
            (p[i],p[target]) = (p[target],p[i]);
        }
    }
    fn trilinear_interp(&self,c:[[[f32;2];2];2],u:f32,v:f32,w:f32)->f32{
        let mut accum:f32 = 0.0;
        for i in (0..2){
            for j in (0..2){
                for k in (0..2){
                    accum += (i as f32 * u + (1-i) as f32 * (1.0-u))*
                             (j as f32 * v + (1-j) as f32 * (1.0-v))*
                             (k as f32 * w + (1-k) as f32 * (1.0-w))*c[i][j][k];
                }
            }
        }
        return accum;
    }
    fn perlin_interp(&self,c:[[[Vec3;2];2];2],u:f32,v:f32,w:f32)->f32{
        let uu = u*u*(3.0-2.0*u);
        let vv = v*v*(3.0-2.0*v);
        let ww = w*w*(3.0-2.0*w);
        let mut accum:f32 = 0.0;

        for i in (0..2){
            for j in (0..2){
                for k in (0..2){
                    let weiht_v = Vec3::new(u-i as f32,v-j as f32,w-k as f32);
                    accum += (i as f32 * uu + (1-i) as f32 * (1.0-uu))*
                             (j as f32 * vv + (1-j) as f32 * (1.0-vv))*
                             (k as f32 * ww + (1-k) as f32 * (1.0-ww))*c[i][j][k].dot(&weiht_v);
                }
            }
        }
        return accum;
    }

}