// space-algebra - algebraic structure used for manipulating
// things in space

use vector3d::{Vec3, Matrix3};
use std::ops::Mul;
use algebra::Group;

use std::vec::Vec;
use std::f64;


#[derive(Clone, Copy, PartialEq, Debug)]
enum SO3Gen {
    RotationX(f64),
    RotationY(f64),
    RotationZ(f64),
}

impl SO3Gen {
    fn to_matrix(self) -> Matrix3 {
        match self {
            SO3Gen::RotationX(theta) => 
                Matrix3::with_columns( Vec3(1.0, 0.0, 0.0),
                                       Vec3(0.0, theta.cos(), -theta.sin()),
                                       Vec3(0.0, theta.sin(), theta.cos()) ),
            SO3Gen::RotationY(theta) =>
                Matrix3::with_columns( Vec3(theta.cos(), 0.0, theta.sin()),
                                       Vec3(0.0, 1.0, 0.0),
                                       Vec3(-theta.sin(), 0.0, theta.cos()) ),
            SO3Gen::RotationZ(theta) =>
                Matrix3::with_columns( Vec3(theta.cos(), theta.sin(), 0.0),
                                      Vec3(-theta.sin(), theta.cos(), 0.0),
                                      Vec3(0.0, 0.0, 1.0) )
        }
    }
}


#[derive(Clone, Copy, PartialEq, Debug)]
pub struct SO3(Matrix3);


impl SO3 {
    pub fn identity() -> SO3 {
        SO3(Matrix3::identity())
    }

    pub fn rotation_x(theta: f64) -> SO3 {
        if theta == 0.0 {
            SO3::identity()
        } else {
            SO3(SO3Gen::RotationX(theta).to_matrix())
        }
    }

    pub fn rotation_y(theta: f64) -> SO3 {
        if theta == 0.0 {
            SO3::identity()
        } else {
            SO3(SO3Gen::RotationY(theta).to_matrix())
        }
    }

    pub fn rotation_z(theta: f64) -> SO3 {
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

impl Mul<Vec3> for SO3 {
    type Output = Vec3;
    fn mul(self, rhs : Vec3) -> Vec3 {
        let SO3(m) = self;
        m * rhs
    }
}


#[test]
fn test_SO3_multiplication() {
    let tolerance = 0.0000001;
    let SO3(m1) = SO3::rotation_x(3.0) * SO3::rotation_x(-3.0);
    assert!(Matrix3::dist(m1, Matrix3::identity()) < tolerance);
}

#[test]
fn test_SO3_multiply_vector() {
    let s = SO3::rotation_x(1.0);
    let v = Vec3(1.0, 0.0, 0.0);
    let expected = Vec3(1.0, 0.0, 0.0);

    assert!(s * v == expected);
}
