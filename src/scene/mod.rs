// scene.rs - definitions for scenes
use image::Rgb;
use vector3d::Vec3;
use ray::Shadable;

pub struct Light {
    pub position : Vec3,
    pub colour : Rgb<u8>
}

pub struct AmbientLight {
    colour : Rgb<u8>
}
    

pub struct Scene {
    pub ambient_light : AmbientLight,
    pub objects : Vec<Box<Shadable>>,
    pub lights : Vec<Light>
}
