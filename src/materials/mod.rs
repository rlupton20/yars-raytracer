// materials.rs - materials for objects
use image::Rgb;

pub struct Material {
    specular : [f64 ; 3],
    diffuse : [f64 ; 3],
    ambient : [f64 ; 3],
    shine : f64
}

impl Material {
    pub fn plain(col : Rgb<u8>) -> Material {
        Material { specular : [1.0 ; 3],
                   diffuse : [1.0 ; 3],
                   ambient : [1.0 ; 3],
                   shine : 3.0
        }
    }

    pub fn shine(&self) -> f64 {
        self.shine
    }

    pub fn ambient_refletivity(&self) -> [f64 ; 3] {
        self.ambient
    }

    pub fn diffusive_reflectivity(&self) -> [f64 ; 3] {
        self.diffuse
    }

    pub fn specular_reflectiviy(&self) -> [f64 ; 3] {
        self.specular
    }
}

pub trait HasMaterial {
    fn material(&self) -> &Material;
}
