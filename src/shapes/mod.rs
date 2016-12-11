// shapes.rs - descriptions of basic shapes
use vector3d::Vec3;
use algebra::InnerProductSpace;
use ray::{Ray, Intersectable, Orientable};

pub struct Sphere {
    pub centre : Vec3,
    pub radius : f64
}

impl Intersectable for Sphere {
    fn intersect(&self, ray : &Ray) -> Option<Vec3> {
        let dir = ray.direction;
        let dist_origin = ray.origin - self.centre;

        let a = dir.dot(dir);
        let b = dir.dot(dist_origin);
        let c = dist_origin.dot(dist_origin) - self.radius;

        let discriminant = b*b - a*c;

        if discriminant >= 0.0 {
            let t = if -b > discriminant.sqrt() {
                (-b - discriminant.sqrt()) / a
            }
            else {
                (-b + discriminant.sqrt()) / a
            };

            Some(ray.origin + t*ray.direction)
        }
        else {
            None
        }
    }
}


impl Orientable for Sphere {
    fn normal(self, v : Vec3) -> Vec3 {
        let w = v - self.centre;
        1.0 / w.norm() * w
    }
}

#[test]
fn test_sphere_orientability() {
    let s = Sphere { centre : Vec3::zero(),
                     radius : 1.0 };
    let p = Vec3(1.0, 0.0, 0.0);
    assert!(s.normal(p) == p);
}

#[test]
fn test_intersection_of_ray_and_sphere() {
    let sphere = Sphere { centre : Vec3::zero(),
                          radius : 1.0 };
    let ray = Ray { origin : Vec3::zero(),
                    direction : Vec3(1.0, 0.0, 0.0)};
    let expected = Some(Vec3(1.0, 0.0, 0.0));
    assert!(expected == sphere.intersect(&ray));
}
