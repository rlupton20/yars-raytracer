extern crate yars_raytracer;
extern crate image;

use std::fs::File;
use std::path::Path;

use yars_raytracer::vector3d::Vec3;
use yars_raytracer::algebra::InnerProductSpace;
use yars_raytracer::space_algebra::SO3;
use yars_raytracer::ray::Orientable;
use yars_raytracer::camera::CameraBuilder;
use yars_raytracer::scene::{Scene, Light, AmbientLight};
use yars_raytracer::shade::{Shader, PhongShader};
use yars_raytracer::materials::Material;
use yars_raytracer::raytrace::Raytracer;
use image::{ImageBuffer, ImageRgb8, Rgb, PNG};

use yars_raytracer::shapes::{Sphere, Plane};
use yars_raytracer::ray::{Ray, Shadable, ShadeCell};

fn main() {
    let WIDTH = 800;
    let HEIGHT = 600;
    let OUTPUT = "output.png";

    let camera = (SO3::rotation_x(0.47) * CameraBuilder::new(WIDTH, HEIGHT, 45.0) +
                  Vec3(0.0, -2.0, 0.0))
        .build();

    let mut img = ImageBuffer::new(WIDTH, HEIGHT);

    // Some test paramters
    let a_colour = Rgb([255 as u8; 3]);
    let light = Light {
        position: Vec3(4.0, -4.0, 0.0),
        colour: Rgb([255 as u8, 255 as u8, 200 as u8]),
    };

    let slate = Material::new([0.01; 3], [0.03; 3], [0.1; 3], [0.0; 3], 2.0);

    let mat1 = Material::new([0.5, 0.5, 0.5],
                             [0.5, 0.3, 0.01],
                             [0.5, 0.3, 0.01],
                             [0.1, 0.1, 0.05],
                             7.0);

    let mat2 = Material::new([0.3, 0.2, 0.5],
                             [0.3, 0.1, 0.5],
                             [0.3, 0.1, 0.5],
                             [0.1, 0.1, 0.1],
                             4.0);

    let plane =
        Box::new(Plane::new(Vec3(1.0, 0.0, 0.0), Vec3(0.0, 0.0, 1.0), slate)) as Box<Shadable>;
    let sphere = Box::new(Sphere::new(Vec3(0.0, -1.0, 5.0), 1.0, mat1)) as Box<Shadable>;
    let obst = Box::new(Sphere::new(Vec3(-2.0, -0.5, 4.0), 0.5, mat2)) as Box<Shadable>;
    let scene_objects = vec![plane, sphere, obst];

    let ambient = AmbientLight { colour: Rgb([70; 3]) };

    let scene = Scene {
        ambient_light: ambient,
        objects: scene_objects,
        lights: vec![light],
    };

    let tracer = Raytracer::<PhongShader>::from_shader(PhongShader::instance());

    // now do some tracing

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let ray = camera.get_ray_through_pixel(x, y);


        match tracer.trace_to_depth(2, &ray, &scene) {
            Some(col) => *pixel = col,
            None => *pixel = Rgb([0 as u8, 0 as u8, 0 as u8]),
        }
    }

    let ref mut fout = File::create(&Path::new(OUTPUT)).unwrap();
    let _ = ImageRgb8(img).save(fout, PNG);
}
