// ray.rs - description of rays and related things

use vector3d::Vec3;
use algebra::InnerProductSpace;
use materials::{Material, HasMaterial};

#[derive(Clone,Copy,PartialEq)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<Vec3>;
}


pub trait Orientable {
    fn normal(&self, v: Vec3) -> Vec3;

    // If we have a normal, we can compute a reflection;
    // we provide a default implementation for reflect
    fn reflect(&self, p: Vec3, v: Vec3) -> Vec3 {
        let norm = self.normal(p);
        -1.0 * (2.0 * norm.dot(v) * norm - v)
    }
}

impl Ray {
    // intersection does dynamic dispatch over Intersectable objects
    pub fn intersection(&self, object: &Intersectable) -> Option<Vec3> {
        object.intersect(&self)
    }

    pub fn hits(&self, object: &Intersectable) -> bool {
        self.intersection(object).is_some()
    }

    pub fn trace(&self, objects: Vec<Box<Shadable>>) -> Option<(Vec3, Vec3, Material)> {
        let strikes = objects.into_iter()
            .map(|x| x.intersect(&self))
            .filter(|x| x.is_some());
        None
    }
}

pub trait Shadable: Intersectable + Orientable + HasMaterial {}

#[cfg(test)]
mod tests {
    use super::*;
    use shapes::Sphere;
    use vector3d::Vec3;

    #[test]
    fn test_ray_intersection_boolean() {
        let sphere = Sphere::simple(Vec3(0.0, 0.0, 0.0), 1.0);
        let ray = Ray {
            origin: Vec3(2.0, 0.0, 0.0),
            direction: Vec3(-1.0, 0.0, 0.0),
        };

        assert!(ray.hits(&sphere));
    }

}
