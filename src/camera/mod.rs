// camera - module for describing camera operations
use vector3d::Vec3;
use space_algebra::SO3;

use std::f64;

#[derive(Clone,Copy,PartialEq)]
pub struct Camera {
    canvas_x : u32,
    canvas_y : u32,
    field_of_view : f64,
    position : Vec3,
    orientation : SO3
}

impl Camera {
    pub fn new(x : u32, y : u32, fov : f64) -> Camera {
        if fov <= 0.0 || fov >= 180.0 {
            panic!("Camera: field of view too big or too small: {0}", fov);
        }
        else {
            Camera{ canvas_x : x,
                    canvas_y : y,
                    field_of_view : fov,
                    position : Vec3::zero(),
                    orientation : SO3::identity() }
        }
    }

    fn _get_world_height(&self) -> f64 {
        2.0 * (self.field_of_view / 2.0).tan()
    }

    fn _get_world_width(&self) -> f64 {
        (self.canvas_x as f64) / (self.canvas_y as f64) * self._get_world_height()
    }
        

    fn _get_zero_ray_direction(&self, x : u32, y : u32) -> Vec3 {
        assert!(x>=0);
        assert!(x < self.canvas_x);
        assert!(y>=0);
        assert!(y < self.canvas_y);

        let x_step = self._get_world_width() / (self.canvas_x as f64);
        let y_step = self._get_world_height() / (self.canvas_y as f64);

        Vec3( x_step * ((x as f64) - (self.canvas_x as f64) / 2.0),
              y_step * ((y as f64) - (self.canvas_y as f64) / 2.0),
              1.0 )
    }

    pub fn get_ray_through_pixel(&self, x : u32, y : u32) -> Vec3 {
        self.orientation * self._get_zero_ray_direction(x,y)
    }
        
}


#[test]
fn test_get_zero_ray_direction() {
    let camera = Camera::new(100,100,90.0);
    assert!(camera.get_ray_through_pixel(50, 50) == Vec3(0.0, 0.0, 1.0));
}
