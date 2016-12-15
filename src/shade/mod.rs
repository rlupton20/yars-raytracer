// shade.rs - things that can be shaded, and how to shade them
use image::{Rgb, Primitive};
use vector3d::Vec3;
use algebra::InnerProductSpace;
use shapes::Sphere;
use scene::{Scene, AmbientLight, Light};
use ray::{Ray, ShadeCell, Shadable};
use std::ops::Mul;

fn red<T: Primitive>(c: Rgb<T>) -> T {
    c.data[0]
}

fn green<T: Primitive>(c: Rgb<T>) -> T {
    c.data[1]
}

fn blue<T: Primitive>(c: Rgb<T>) -> T {
    c.data[2]
}

struct PhongShader {}

impl PhongShader {
    fn dot(x: Vec3, y: Vec3) -> f64 {
        x.dot(y).max(0.0)
    }

    fn ambient_light<'a>(s: &'a Scene) -> &'a AmbientLight {
        &s.ambient_light
    }

    fn diffuse_at_shade_cell(shade_cell: &ShadeCell, scene: &Scene) -> Vec<(f64, Rgb<u8>)> {
        let &ShadeCell(p, n, _) = shade_cell;
        scene.lights
            .iter()
            .filter(|l| l.illuminates(p, &scene.objects))
            .map(|l| (PhongShader::dot((l.position - p).normalize(), n), l.colour))
            .collect()
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
    let expected = vec![(1.0, a_colour)];

    assert!(expected == diffuse);
}

// Tests which run from the outside
#[cfg(test)]
mod tests {
    use super::*;
    use image::Rgb;
    use vector3d::Vec3;

}
