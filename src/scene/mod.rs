// scene.rs - definitions for scenes
use image::Rgb;
use vector3d::Vec3;
use ray::{Ray, Shadable, ShadeCell};

pub struct Light {
    pub position : Vec3,
    pub colour : Rgb<u8>
}

pub struct AmbientLight {
    pub colour : Rgb<u8>
}
    

pub struct Scene {
    pub ambient_light : AmbientLight,
    pub objects : Vec<Box<Shadable>>,
    pub lights : Vec<Light>
}

impl Light {
    pub fn illuminates(&self, p : Vec3, objects : &Vec<Box<Shadable>>) -> bool {
        let strike = Ray::new(p, self.position - p).trace(objects);
        match strike {
            Some(ShadeCell(x,_,_,_)) => if (x - p).norm() < (p - self.position).norm() {
                false
            }
            else {
                true
            },
            None => true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vector3d::Vec3;
    use image::Rgb;
    use shapes::Sphere;
    use ray::Shadable;

    #[test]
    fn test_can_detect_interfering_object() {
        let light = Light {
            position : Vec3(0.0, 0.0, 5.0),
            colour : Rgb([255 as u8 ; 3]) };

        let sphere = Box::new(Sphere::simple(Vec3(0.0, 0.0, 0.0), 1.0)) as Box<Shadable>;
        let objects = vec!(sphere);

        let point = Vec3(0.0, 0.0, -5.0);

        assert!(!light.illuminates(point, &objects));
    }

    #[test]
    fn test_can_detect_non_interfering_object() {
        let light = Light {
            position : Vec3(0.0, 0.0, 5.0),
            colour : Rgb([255 as u8 ; 3]) };

        let sphere = Box::new(Sphere::simple(Vec3(0.0, 0.0, 0.0), 1.0)) as Box<Shadable>;
        let objects = vec!(sphere);

        let point = Vec3(3.0, 0.0, -5.0);

        assert!(light.illuminates(point, &objects));
    }

    #[test]
    fn test_light_detects_interference_in_correct_portion_of_ray() {
        let light = Light {
            position : Vec3(2.0, 0.0, 0.0),
            colour : Rgb([255 as u8 ; 3]) };

        let sphere = Box::new(Sphere::simple(Vec3(0.0, 0.0, 0.0), 1.0)) as Box<Shadable>;
        let objects = vec!(sphere);

        let point = Vec3(3.0, 0.0, 0.0);

        assert!(light.illuminates(point, &objects));
    }

    #[test]
    fn test_light_illumination_is_not_confused_by_two_objects() {
        let light = Light {
            position : Vec3(2.0, 0.0, 0.0),
            colour : Rgb([255 as u8 ; 3]) };

        let sphere = Box::new(Sphere::simple(Vec3(0.0, 0.0, 0.0), 1.0)) as Box<Shadable>;
        let hidden = Box::new(Sphere::simple(Vec3(-5.0, 0.0, 0.0), 1.0)) as Box<Shadable>;
        let objects = vec!(sphere, hidden);

        let point = Vec3(1.0, 0.0, 0.0);

        assert!(light.illuminates(point, &objects));
    }
        
}
