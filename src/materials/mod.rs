// materials.rs - materials for objects
use image::Rgb;

pub struct Material {
    specular: [f64; 3],
    diffuse: [f64; 3],
    ambient: [f64; 3],
    reflectivity: [f64; 3],
    shine: f64,
}

impl Material {
    pub fn plain(col: Rgb<u8>) -> Material {
        Material::new([0.1, 0.3, 0.2],
                      [0.1, 0.8, 0.4],
                      [0.2, 0.6, 0.6],
                      [0.03; 3],
                      3.0)
    }

    pub fn new(specularity: [f64; 3],
               diffusivity: [f64; 3],
               ambience: [f64; 3],
               reflect: [f64; 3],
               shininess: f64)
               -> Material {
        Material {
            specular: specularity,
            diffuse: diffusivity,
            ambient: ambience,
            reflectivity: reflect,
            shine: shininess,
        }
    }

    pub fn shine(&self) -> f64 {
        self.shine
    }

    pub fn ambient_refletivity(&self) -> [f64; 3] {
        self.ambient
    }

    pub fn diffusive_reflectivity(&self) -> [f64; 3] {
        self.diffuse
    }

    pub fn specular_reflectiviy(&self) -> [f64; 3] {
        self.specular
    }

    pub fn reflectivity(&self) -> [f64; 3] {
        self.reflectivity
    }
}

pub trait HasMaterial {
    fn material(&self) -> &Material;
}
