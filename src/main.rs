extern crate yars_raytracer;
extern crate image;

use std::fs::File;
use std::path::Path;

use yars_raytracer::vector3d::Vec3;
use yars_raytracer::algebra::InnerProductSpace;
use yars_raytracer::ray::Orientable;
use yars_raytracer::camera::CameraBuilder;
use yars_raytracer::scene::Light;
use image::{ImageBuffer, ImageRgb8, Rgb, PNG};

use yars_raytracer::shapes::Sphere;
use yars_raytracer::ray::{Ray, Intersectable};

fn main() {
    let WIDTH = 800;
    let HEIGHT = 600;
    let OUTPUT = "output.png";
    
    let camera = CameraBuilder::new(WIDTH,HEIGHT,90.0).build();

    let mut img = ImageBuffer::new(WIDTH,HEIGHT);
    let s = Sphere::simple(Vec3(0.0, 0.0, 3.0), 1.0);
    // Lights
    let m = Vec3(3.0, -5.0, 2.0);

    for (x,y,pixel) in img.enumerate_pixels_mut() {
        let dir = camera.get_direction_through_pixel(x,y);
        let ray = Ray { origin : Vec3::zero(),
                        direction : dir };
        
        match s.intersect(&ray) {
            Some(p) => {
                let i = 0.4 * (m-p).normalize().dot(s.normal(p));
                let j = 0.4 * s.reflect(p, p-m).normalize().dot(-1.0 * p.normalize()).max(0.0).powf(7.0);
                let col : u8 = (40.0 + 255.0 * i + 255.0 * j).floor().min(255.0).max(0.0) as u8;
                *pixel = Rgb([col, 0 as u8, 0 as u8]);
            }
            None => *pixel = Rgb([0 as u8, 0 as u8, 0 as u8]),
        }
    }

    let ref mut fout = File::create(&Path::new(OUTPUT)).unwrap();
    let _ = ImageRgb8(img).save(fout, PNG);
}
