// scene.rs - definitions for scenes
use image::Rgb;
use vector3d::Vec3;
use shade::Shadable;

pub struct Light {
    pub position : Vec3,
    pub colour : Rgb<u8>
}

pub struct AmbientLight {
    colour : Rgb<u8>
}
    

pub struct Scene {
    objects : Vec<Box<Shadable>>,
    lights : Vec<Light>
}
