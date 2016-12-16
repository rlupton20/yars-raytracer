extern crate yars_raytracer;
extern crate image;

use std::fs::File;
use std::path::Path;

use yars_raytracer::vector3d::Vec3;
use yars_raytracer::algebra::InnerProductSpace;
use yars_raytracer::ray::Orientable;
use yars_raytracer::camera::CameraBuilder;
use yars_raytracer::scene::{Scene, Light, AmbientLight};
use yars_raytracer::shade::{PhongShader};
use image::{ImageBuffer, ImageRgb8, Rgb, PNG};

use yars_raytracer::shapes::Sphere;
use yars_raytracer::ray::{Ray, Shadable, ShadeCell};

fn main() {
    let WIDTH = 800;
    let HEIGHT = 600;
    let OUTPUT = "output.png";
    
    let camera = CameraBuilder::new(WIDTH,HEIGHT,45.0).build();

    let mut img = ImageBuffer::new(WIDTH,HEIGHT);

    // Some test paramters
    let a_colour = Rgb([255 as u8; 3]);
    let light = Light {
        position: Vec3(4.0, -4.0, 0.0),
        colour: a_colour,
    };

    let sphere = Box::new(Sphere::simple(Vec3(0.0, 0.0, 8.0), 1.0)) as Box<Shadable>;
    let obst = Box::new(Sphere::simple(Vec3(2.0, -2.0, 4.0), 0.05)) as Box<Shadable>;
    let scene_objects = vec![sphere, obst];

    let ambient = AmbientLight { colour: a_colour };

    let scene = Scene {
        ambient_light: ambient,
        objects: scene_objects,
        lights: vec![light],
    };

    // now do some tracing

    for (x,y,pixel) in img.enumerate_pixels_mut() {
        let dir = camera.get_direction_through_pixel(x,y);
        let ray = Ray { origin : Vec3::zero(),
                        direction : dir };
        
        match ray.trace(&scene.objects) {
            Some(sc) => {
                let i = PhongShader::diffuse_at_shade_cell(&sc, &scene).iter()
                    .map(|x| x.0)
                    .fold(0.1, |x, y| x + y);
                let j = PhongShader::specular_at_shade_cell(&sc, &scene).iter()
                    .map(|x| x.0)
                    .fold(0.1, |x, y| x + y)
                    .powf(5.0);
                let col = (40.0 + 0.3 * i * 255.0 + 0.3 * j * 255.0).floor().min(255.0) as u8;
                let ShadeCell(x, _, _, _) = sc;
                let Vec3(q,_,p) = x;

                *pixel = Rgb([col, 0 as u8, 0 as u8]);
            }
            None => *pixel = Rgb([0 as u8, 0 as u8, 0 as u8]),
        }
    }

    let ref mut fout = File::create(&Path::new(OUTPUT)).unwrap();
    let _ = ImageRgb8(img).save(fout, PNG);
}
