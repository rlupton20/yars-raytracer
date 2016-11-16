extern crate yars_raytracer;
extern crate image;

use std::fs::File;
use std::path::Path;

use yars_raytracer::vector3d::Vec3;
use yars_raytracer::camera::CameraBuilder;
use image::{ImageBuffer, ImageRgb8, Rgb, PNG};

fn main() {
    let WIDTH = 800;
    let HEIGHT = 600;
    let OUTPUT = "output.png";
    
    let camera = CameraBuilder::new(WIDTH,HEIGHT,90.0).build();

    let mut img = ImageBuffer::new(WIDTH,HEIGHT);

    for (x,y,pixel) in img.enumerate_pixels_mut() {
        let Vec3(x,_,_) = camera.get_direction_through_pixel(x,y);
        
        if x*x < 0.5 {
            *pixel = Rgb([255 as u8, 0 as u8, 0 as u8]);
        }
        else {
            *pixel = Rgb([0 as u8, 0 as u8, 0 as u8]);
        }
    }

    let ref mut fout = File::create(&Path::new(OUTPUT)).unwrap();
    let _ = ImageRgb8(img).save(fout, PNG);
}
