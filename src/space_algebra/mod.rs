// space-algebra - algebraic structure used for manipulating
// things in space

use vector3d::{Vec3, Matrix3};
use std::ops::{Mul};
use algebra::{Group};

use std::vec::Vec;


#[derive(Clone, Copy, PartialEq, Debug)]
enum SO3Gen {
    RotationX(f64),
    RotationY(f64),
    RotationZ(f64)
}

#[derive(Clone, PartialEq, Debug)]
pub struct SO3(Vec<SO3Gen>);


impl SO3 {
    fn rotation_x(theta : f64) -> SO3 {
        if theta == 0.0 {
            SO3(Vec::new())
        }
        else {
            SO3(vec![SO3Gen::RotationX(theta)])
        }
    }

    fn rotation_y(theta : f64) -> SO3 {
        if theta == 0.0 {
            SO3(Vec::new())
        }
        else {
            SO3(vec![SO3Gen::RotationY(theta)])
        }
    }

    fn rotation_z(theta : f64) -> SO3 {
        if theta == 0.0 {
            SO3(Vec::new())
        }
        else {
            SO3(vec![SO3Gen::RotationZ(theta)])
        }
    }
}
