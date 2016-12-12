// ray.rs - description of rays and related things

use vector3d::Vec3;
use algebra::InnerProductSpace;

#[derive(Clone,Copy,PartialEq)]
pub struct Ray {
    pub origin : Vec3,
    pub direction : Vec3
}

pub trait Intersectable {
    fn intersect(&self, ray : &Ray) -> Option<Vec3>;
}

pub trait Orientable {
    fn normal(&self, v : Vec3) -> Vec3;

    // If we have a normal, we can compute a reflection;
    // we provide a default implementation for reflect
    fn reflect(&self, p : Vec3, v : Vec3) -> Vec3 {
        let norm = self.normal(p);
        -1.0 * (2.0 * norm.dot(v) * norm - v)
    }
}
