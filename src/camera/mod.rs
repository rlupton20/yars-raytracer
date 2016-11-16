// camera - module for describing camera operations
use vector3d::Vec3;
use algebra::InnerProductSpace;
use space_algebra::SO3;
use ray::Ray;

use std::f64;
use std::f64::consts::PI;
use std::ops::{Add, Mul};

#[derive(Clone,Copy,PartialEq)]
pub struct CameraBuilder {
    canvas_x: u32,
    canvas_y: u32,
    field_of_view: f64,
    position: Vec3,
    orientation: SO3,
}

pub struct Camera {
    canvas_x: u32,
    canvas_y: u32,
    width: f64,
    height: f64,
    position: Vec3,
    orientation: SO3,
}

impl CameraBuilder {
    pub fn new(x: u32, y: u32, fov: f64) -> CameraBuilder {
        if fov <= 0.0 || fov >= 180.0 {
            panic!("CameraBuilder: field of view too big or too small: {0}",
                   fov);
        } else {
            CameraBuilder {
                canvas_x: x,
                canvas_y: y,
                field_of_view: fov,
                position: Vec3::zero(),
                orientation: SO3::identity(),
            }
        }
    }

    fn _get_world_height(&self) -> f64 {
        2.0 * (self.field_of_view / 2.0).tan()
    }

    fn _get_world_width(&self) -> f64 {
        (self.canvas_x as f64) / (self.canvas_y as f64) * self._get_world_height()
    }



    pub fn build(self) -> Camera {
        Camera {
            canvas_x: self.canvas_x,
            canvas_y: self.canvas_y,
            width: self._get_world_width(),
            height: self._get_world_height(),
            position: self.position,
            orientation: self.orientation,
        }
    }
}


impl Camera {

    fn _get_zero_ray_direction(&self, x: u32, y: u32) -> Vec3 {
        assert!(x >= 0);
        assert!(x < self.canvas_x);
        assert!(y >= 0);
        assert!(y < self.canvas_y);

        let x_step = self.width / (self.canvas_x as f64);
        let y_step = self.height / (self.canvas_y as f64);

        Vec3(x_step * ((x as f64) - (self.canvas_x as f64) / 2.0),
             y_step * ((y as f64) - (self.canvas_y as f64) / 2.0),
             1.0)
    }

    pub fn get_direction_through_pixel(&self, x: u32, y: u32) -> Vec3 {
        self.orientation * self._get_zero_ray_direction(x, y)
    }

    pub fn get_ray_through_pixel(&self, x: u32, y: u32) -> Ray {
        Ray {
            origin: self.position,
            direction: self.get_direction_through_pixel(x, y),
        }
    }
}

impl Mul<CameraBuilder> for SO3 {
    type Output = CameraBuilder;
    fn mul(self, camera: CameraBuilder) -> CameraBuilder {
        CameraBuilder {
            canvas_x: camera.canvas_x,
            canvas_y: camera.canvas_y,
            field_of_view: camera.field_of_view,
            position: camera.position,
            orientation: self * camera.orientation,
        }
    }
}

impl Add<Vec3> for CameraBuilder {
    type Output = CameraBuilder;
    fn add(self, v: Vec3) -> CameraBuilder {
        CameraBuilder {
            canvas_x: self.canvas_x,
            canvas_y: self.canvas_y,
            field_of_view: self.field_of_view,
            position: self.position + v,
            orientation: self.orientation,
        }
    }
}


#[test]
fn test_get_zero_ray_direction() {
    let camera = CameraBuilder::new(100, 100, 90.0).build();
    assert!(camera.get_direction_through_pixel(50, 50) == Vec3(0.0, 0.0, 1.0));
}

#[test]
fn test_get_rotated_camera_direction() {
    let precamera = SO3::rotation_y(PI / 2.0) * CameraBuilder::new(100, 100, 90.0);
    let camera = precamera.build();
    let tolerance = 0.00000001;
    let v = camera.get_direction_through_pixel(50, 50) - Vec3(-1.0, 0.0, 0.0);
    assert!(v.dot(v) < tolerance);
}

#[test]
fn test_get_ray_through_pixel() {
    let camera = (CameraBuilder::new(100, 100, 90.0) + Vec3(1.0, 0.0, 0.0)).build();
    let expected = Ray {
        origin: Vec3(1.0, 0.0, 0.0),
        direction: Vec3(0.0, 0.0, 1.0),
    };
    assert!(expected == camera.get_ray_through_pixel(50, 50));
}
