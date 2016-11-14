// space-algebra - algebraic structure used for manipulating
// things in space

use vector3d::{Vec3, Matrix3};
use std::ops::Mul;
use algebra::Group;

use std::vec::Vec;


#[derive(Clone, Copy, PartialEq, Debug)]
enum SO3Gen {
    RotationX(f64),
    RotationY(f64),
    RotationZ(f64),
}

impl SO3Gen {
    fn to_matrix(self) -> Matrix3 {
        match self {
            _ => Matrix3::identity()
        }
    }
}


#[derive(Clone, Copy, PartialEq, Debug)]
pub struct SO3(Matrix3);


impl SO3 {
    fn identity() -> SO3 {
        SO3(Matrix3::identity())
    }

    fn rotation_x(theta: f64) -> SO3 {
        if theta == 0.0 {
            SO3::identity()
        } else {
            SO3(SO3Gen::RotationX(theta).to_matrix())
        }
    }

    fn rotation_y(theta: f64) -> SO3 {
        if theta == 0.0 {
            SO3::identity()
        } else {
            SO3(SO3Gen::RotationY(theta).to_matrix())
        }
    }

    fn rotation_z(theta: f64) -> SO3 {
        if theta == 0.0 {
            SO3::identity()
        } else {
            SO3(SO3Gen::RotationZ(theta).to_matrix())
        }
    }
}


impl Mul for SO3 {
    type Output = SO3;

    fn mul(self, rhs : SO3) -> SO3 {
        let SO3(m) = self;
        let SO3(n) = rhs;
        SO3(m*n)
    }
}


#[test]
fn test_SO3_multiplication() {
    assert!(SO3::rotation_x(3.0) * SO3::rotation_y(3.0) == SO3::identity())
}
