// shade.rs - things that can be shaded, and how to shade them
use image::{Rgb, Primitive};
use vector3d::Vec3;
use scene::{ Scene, AmbientLight, Light };


fn red<T : Primitive>(c : Rgb<T>) -> T {
    c.data[0]
}

fn green<T : Primitive>(c : Rgb<T>) -> T {
    c.data[1]
}

fn blue<T : Primitive>(c : Rgb<T>) -> T {
    c.data[2]
}

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

struct PhongShader {}

impl PhongShader {
    fn ambient_light(s : Scene) -> AmbientLight {
        s.ambient_light
    }

}
