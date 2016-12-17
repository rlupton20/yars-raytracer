// raytrace.rs - generic raytracing functionality

use shade::{Shader, red, green, blue};
use ray::{Ray, ShadeCell};
use scene::Scene;
use image::Rgb;

pub struct Raytracer<S> {
    shader: S,
}

impl<S: Shader> Raytracer<S> {
    pub fn from_shader(s: S) -> Raytracer<S> {
        Raytracer { shader: s }
    }

    pub fn trace_to_depth(&self, depth: u8, ray: &Ray, scene: &Scene) -> Option<Rgb<u8>> {
        let next_step = |sc : &ShadeCell| {
            self.generate_next_rays_and_effect(sc).into_iter()
                .map(|(i,r)| (i, self.trace_to_depth(depth-1, &r, scene)))
                .filter(|&(i,op_col)| op_col.is_some())
                .map(|(i, is_col)| (i, is_col.unwrap()))
                .map(|(i, col)| Raytracer::<S>::adjust_intensity_piecewise(col, i))
                .collect()
        };

        if depth < 1 {
            None
        } else {
            ray.trace(&scene.objects)
                .map(|sc| self.shader.shade(&sc, scene, next_step(&sc)))
        }
    }

    fn generate_next_rays_and_effect(&self, shader_cell: &ShadeCell) -> Vec<([f64; 3], Ray)> {
        let &ShadeCell(p, n, v, m) = shader_cell;
        let reflection = Ray { origin : p,
                               direction : n.reflect(v) };
        vec![(m.reflectivity(), reflection)]
    }

    // This is the same function as in the shader, and probably should
    // be factored out at some point into a more useful module
    fn adjust_intensity_piecewise(c : Rgb<u8>, adjust_components : [f64 ; 3]) -> Rgb<u8> {
        let approx = |col: f64| col.floor() as u8;
        let adjust = |f: &Fn(Rgb<u8>) -> u8, a : f64| approx(a * (f(c) as f64));

        Rgb([adjust(&red, adjust_components[0]),
             adjust(&green, adjust_components[1]),
             adjust(&blue, adjust_components[2])])
    }
}
