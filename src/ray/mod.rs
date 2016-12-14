// ray.rs - description of rays and related things

use vector3d::Vec3;
use algebra::{InnerProductSpace, Real};
use materials::{Material, HasMaterial};

#[derive(Clone,Copy,PartialEq)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

// A ShadeCell contains all the point relevant information
// we need to colour a point
pub struct ShadeCell<'a>(Vec3, Vec3, &'a Material);

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

pub trait Shadable: Intersectable + Orientable + HasMaterial {}

impl Ray {
    pub fn new(o: Vec3, d: Vec3) -> Ray {
        Ray {
            origin: o,
            direction: d,
        }
    }

    // intersection does dynamic dispatch over Intersectable objects
    pub fn intersection(&self, object: &Intersectable) -> Option<Vec3> {
        object.intersect(&self)
    }

    pub fn hits(&self, object: &Intersectable) -> bool {
        self.intersection(object).is_some()
    }

    pub fn trace<'a>(&self, objects: &'a Vec<Box<Shadable>>) -> Option<ShadeCell<'a>> {
        let strikes = objects.into_iter()
            .map(|x| x.intersect(&self))
            .zip(objects.into_iter())
            .min_by_key(|x| Ray::measure_strike_distance(self.origin, x.0))
            .unwrap();

        match strikes.0 {
            None => None,
            Some(p) => {
                let shade_cell = ShadeCell(p, strikes.1.normal(p), strikes.1.material());
                Some(shade_cell)
            }
        }
    }

    // Helper function to find the closest intersection
    fn measure_strike_distance(p: Vec3, x: Option<Vec3>) -> Real {
        match x {
            Some(v) => Real::from_float((p - v).norm()).unwrap(),
            None => Real::zero(),
        }
    }
}


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

    #[test]
    fn test_ray_trace_picks_correct_object() {
        let sphere_1 = Box::new(Sphere::simple(Vec3(0.0, 0.0, 0.0), 1.0)) as Box<Shadable>;
        let sphere_2 = Box::new(Sphere::simple(Vec3(3.0, 0.0, 0.0), 1.0)) as Box<Shadable>;
        let ray = Ray {
            origin: Vec3(5.0, 0.0, 0.0),
            direction: Vec3(-1.0, 0.0, 0.0),
        };

        let objects = vec![sphere_1, sphere_2];
        let strike = ray.trace(&objects);

        // The following will assert if there is no collision
        let ShadeCell(x, _, _) = strike.unwrap();
        // The following tests the collision is correct
        assert!(Vec3(4.0, 0.0, 0.0) == x);
    }

    #[test]
    fn test_ray_trace_copes_with_no_strike() {
        let sphere = Box::new(Sphere::simple(Vec3(0.0, 0.0, 0.0), 1.0)) as Box<Shadable>;
        let ray = Ray {
            origin: Vec3(5.0, 0.0, 0.0),
            direction: Vec3(0.0, 1.0, 0.0),
        };

        let objects = vec![sphere];
        let strike = ray.trace(&objects);

        assert!(!strike.is_some());
    }

}
