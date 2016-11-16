// ray.rs - description of rays and related things

use vector3d::Vec3;

#[derive(Clone,Copy,PartialEq)]
pub struct Ray {
    pub origin : Vec3,
    pub direction : Vec3
}

pub trait Intersectable {
    fn intersect(self, ray : Ray) -> Option<Vec3>;
}

pub trait Orientable {
    fn normal(self, v : Vec3) -> Vec3;
}
