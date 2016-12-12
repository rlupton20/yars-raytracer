// materials.rs - materials for objects
use image::Rgb;

pub struct Material {
    colour : Rgb<u8>,
}

impl Material {
    pub fn plain(col : Rgb<u8>) -> Material {
        Material { colour : col }
    }
}

pub trait HasMaterial {
    fn material(&self) -> &Material;
}
