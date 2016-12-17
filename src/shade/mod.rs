// shade.rs - things that can be shaded, and how to shade them
use image::{Rgb, Primitive};
use vector3d::Vec3;
use algebra::InnerProductSpace;
use shapes::Sphere;
use scene::{Scene, AmbientLight, Light};
use ray::{Ray, ShadeCell, Shadable};
use std::ops::{Add, Mul};

fn red<T: Primitive>(c: Rgb<T>) -> T {
    c.data[0]
}

fn green<T: Primitive>(c: Rgb<T>) -> T {
    c.data[1]
}

fn blue<T: Primitive>(c: Rgb<T>) -> T {
    c.data[2]
}

pub struct PhongShader {}

impl PhongShader {
    fn dot(x: Vec3, y: Vec3) -> f64 {
        x.dot(y).max(0.0)
    }

    fn ambient_light<'a>(s: &'a Scene) -> &'a AmbientLight {
        &s.ambient_light
    }

    fn diffuse_at_shade_cell(shade_cell: &ShadeCell, scene: &Scene) -> Vec<Rgb<u8>> {
        let &ShadeCell(p, n, _, _) = shade_cell;
        scene.lights
            .iter()
            .filter(|l| l.illuminates(p, &scene.objects))
            .map(|l| {
                PhongShader::adjust_intensity(l.colour,
                    PhongShader::dot((l.position - p).normalize(), n))
            })
            .collect()
    }

    fn specular_at_shade_cell(shade_cell: &ShadeCell, scene: &Scene) -> Vec<Rgb<u8>> {
        let &ShadeCell(p, n, v, m) = shade_cell;
        let shininess = m.shine();

        let from = |light: &Light| (p - light.position).normalize();
        let reflect = |x: Vec3, n: Vec3| -1.0 * (2.0 * n.dot(x) * n - x);
        let reflection_from = |light: &Light| reflect(from(light), n);

        scene.lights
            .iter()
            .filter(|l| l.illuminates(p, &scene.objects))
            .map(|l| {
                PhongShader::adjust_intensity(l.colour,
                    PhongShader::dot(reflection_from(l), -1.0 * v).powf(shininess))
            })
            .collect()
    }

    fn local_shade(shade_cell : &ShadeCell, scene: &Scene) -> Rgb<u8> {
        let &ShadeCell(_,_,_,m) = shade_cell;
        let ambience = m.ambient_refletivity();
        let diffusivity = m.diffusive_reflectivity();
        let specularity = m.specular_reflectiviy();

        let a : Rgb<u8> = PhongShader::adjust_intensity_piecewise(
            PhongShader::ambient_light(scene).colour, ambience);

        let b = PhongShader::diffuse_at_shade_cell(shade_cell, scene).into_iter()
            .map(|c| PhongShader::adjust_intensity_piecewise(c, diffusivity))
            .fold(a, PhongShader::add_illumination);

        PhongShader::specular_at_shade_cell(shade_cell, scene).into_iter()
            .map(|c| PhongShader::adjust_intensity_piecewise(c, specularity))
            .fold(b, PhongShader::add_illumination)
    }

    pub fn shade(shade_cell : &ShadeCell, scene: &Scene, influence : Vec<Rgb<u8>>)
                 -> Rgb<u8> {
        let i = PhongShader::local_shade(shade_cell, scene);
        influence.into_iter()
            .fold(i, PhongShader::add_illumination)
    }

    fn adjust_intensity_piecewise(c : Rgb<u8>, adjust_components : [f64 ; 3]) -> Rgb<u8> {
        let approx = |col: f64| col.floor() as u8;
        let adjust = |f: &Fn(Rgb<u8>) -> u8, a : f64| approx(a * (f(c) as f64));

        Rgb([adjust(&red, adjust_components[0]),
             adjust(&green, adjust_components[1]),
             adjust(&blue, adjust_components[2])])
    }

    fn adjust_intensity(c: Rgb<u8>, i: f64) -> Rgb<u8> {
        let adjust_components = [i ; 3];
        PhongShader::adjust_intensity_piecewise(c, adjust_components)
    }

    fn add_illumination(a: Rgb<u8>, b: Rgb<u8>) -> Rgb<u8> {
        Rgb([red(a).saturating_add(red(b)),
             green(a).saturating_add(green(b)),
             blue(a).saturating_add(blue(b))])
    }
}


// Tests for internal functions
#[test]
fn test_red_channel_getter() {
    let colour_red = Rgb([255 as u8, 0 as u8, 0 as u8]);

    assert!(255 == red(colour_red));
}

#[test]
fn test_green_channel_getter() {
    let colour_green = Rgb([0 as u8, 255 as u8, 0 as u8]);

    assert!(255 == green(colour_green));
}

#[test]
fn test_blue_channel_getter() {
    let colour_blue = Rgb([0 as u8, 0 as u8, 255 as u8]);

    assert!(255 == blue(colour_blue));
}

#[test]
fn test_phong_dot_returns_zero_on_negative() {
    let x = Vec3(1.0, 0.0, 0.0);
    let y = Vec3(-1.0, 0.0, 0.0);

    assert!(0.0 == PhongShader::dot(x, y));
}

#[test]
fn test_gets_diffuse_at_shade_cell() {
    let a_colour = Rgb([255 as u8; 3]);
    let light = Light {
        position: Vec3(2.0, 0.0, 0.0),
        colour: a_colour,
    };

    let sphere = Box::new(Sphere::simple(Vec3(0.0, 0.0, 0.0), 1.0)) as Box<Shadable>;
    let scene_objects = vec![sphere];

    let ambient = AmbientLight { colour: a_colour };

    let scene = Scene {
        ambient_light: ambient,
        objects: scene_objects,
        lights: vec![light],
    };

    let shade_cell = Ray::new(Vec3(2.0, 0.0, 0.0), Vec3(-1.0, 0.0, 0.0))
        .trace(&scene.objects)
        .unwrap();

    let diffuse = PhongShader::diffuse_at_shade_cell(&shade_cell, &scene);
    print!("{:?}", diffuse);
    let expected = vec![a_colour];

    assert!(expected == diffuse);
}

// Tests which run from the outside
#[cfg(test)]
mod tests {
    use super::*;
    use image::Rgb;
    use vector3d::Vec3;

}
