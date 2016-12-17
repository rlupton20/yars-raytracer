// vector3d -- 3d vectors for working in space
use std::ops::{Add, Mul, Sub};
use algebra::{VectorSpace, InnerProductSpace, Group};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vec3(pub f64, pub f64, pub f64);

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Matrix3(Vec3, Vec3, Vec3);


impl Vec3 {
    pub fn i(&self) -> f64 {
        self.0
    }

    pub fn j(&self) -> f64 {
        self.1
    }

    pub fn k(&self) -> f64 {
        self.2
    }

    pub fn zero() -> Vec3 {
        Vec3(0.0, 0.0, 0.0)
    }
}

// We give Vec3 a vector space structure

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3(self.i() + rhs.i(), self.j() + rhs.j(), self.k() + rhs.k())
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3(self * rhs.i(), self * rhs.j(), self * rhs.k())
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3 {
        self + (-1.0) * rhs
    }
}

impl VectorSpace for Vec3 {
    type Field = f64;
}

// And we also give it an inner product

impl InnerProductSpace for Vec3 {
    fn dot(self, rhs: Vec3) -> f64 {
        self.i() * rhs.i() + self.j() * rhs.j() + self.k() * rhs.k()
    }
}

// From the inner product space structure we can define a norm
impl Vec3 {
    pub fn norm(self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn normalize(self) -> Vec3 {
        (1.0 / self.norm()) * self
    }

    pub fn reflect(self, v : Vec3) -> Vec3 {
        -1.0 * (2.0 * v.dot(self) * self - v)
    }
}

// Now we give Vec3 an (additive) group structure
impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        self + rhs
    }
}

impl Group for Vec3 {
    fn identity() -> Vec3 {
        Vec3::zero()
    }

    fn inverse(self) -> Vec3 {
        (-1.0) * self
    }
}


// Matrix operations

impl Matrix3 {
    fn row1(&self) -> Vec3 {
        let &Matrix3(x, y, z) = self;
        let Vec3(a, _, _) = x;
        let Vec3(b, _, _) = y;
        let Vec3(c, _, _) = z;

        Vec3(a, b, c)
    }

    fn row2(&self) -> Vec3 {
        let &Matrix3(x, y, z) = self;
        let Vec3(_, a, _) = x;
        let Vec3(_, b, _) = y;
        let Vec3(_, c, _) = z;

        Vec3(a, b, c)
    }

    fn row3(&self) -> Vec3 {
        let &Matrix3(x, y, z) = self;
        let Vec3(_, _, a) = x;
        let Vec3(_, _, b) = y;
        let Vec3(_, _, c) = z;

        Vec3(a, b, c)
    }

    pub fn identity() -> Matrix3 {
        Matrix3(Vec3(1.0, 0.0, 0.0),
                Vec3(0.0, 1.0, 0.0),
                Vec3(0.0, 0.0, 1.0))
    }

    pub fn with_columns(col1: Vec3, col2: Vec3, col3: Vec3) -> Matrix3 {
        Matrix3(col1, col2, col3)
    }

    pub fn dist(m1: Matrix3, m2: Matrix3) -> f64 {
        let Matrix3(x, y, z) = m1;
        let Matrix3(r, s, t) = m2;

        (x - r).dot(x - r) + (y - s).dot(y - s) + (z - t).dot(z - t)
    }
}


impl Mul<Vec3> for Matrix3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3(self.row1().dot(rhs),
             self.row2().dot(rhs),
             self.row3().dot(rhs))
    }
}


impl Mul for Matrix3 {
    type Output = Matrix3;

    fn mul(self, rhs: Matrix3) -> Matrix3 {
        let Matrix3(x, y, z) = rhs;

        Matrix3(self * x, self * y, self * z)
    }
}


// Unit tests

#[test]
fn test_components_of_vec3() {
    let v = Vec3(1.0, 2.0, 3.0);

    assert!(v.i() == 1.0);
    assert!(v.j() == 2.0);
    assert!(v.k() == 3.0);
}

#[test]
fn test_vec3_adds_to_vec3_correctly() {
    let v = Vec3(1.0, 2.0, 3.0);
    let w = Vec3(1.0, 2.0, 3.0);
    let expected = Vec3(2.0, 4.0, 6.0);
    assert!(v + w == expected);
}

#[test]
fn test_vec3_multiplies_with_scalar_correctly() {
    let v = Vec3(1.0, 2.0, 3.0);
    let scalar = 3.0;
    let expected = Vec3(3.0, 6.0, 9.0);
    assert!(scalar * v == expected);
}


#[test]
fn test_inner_product_space_for_vec3() {
    let v = Vec3(1.0, 1.0, 1.0);
    let w = Vec3(-0.5, 1.0, -0.5);
    let expected = 0.0;
    assert!(v.dot(w) == expected);
    assert!(w.dot(v) == expected);
}

#[test]
fn test_norm_of_vec3() {
    let v = Vec3(1.0, 4.0, 8.0);
    let expected = 9.0;

    assert!(v.norm() == expected);
}

#[test]
fn test_reflection_in_vector() {
    let v = Vec3(1.0, 0.0, 0.0);
    let w = Vec3(-1.0, -1.0, 0.0);
    let expected = Vec3(1.0, -1.0, 0.0);
    
    assert!(expected == v.reflect(w));
}

#[test]
fn test_group_structure_for_vec3() {
    let v = Vec3(1.0, 2.0, 3.0);
    assert!(v * v.inverse() == Vec3::identity());
}

#[test]
fn test_get_first_row_of_matrix() {
    let col1 = Vec3(1.0, 0.0, 0.0);
    let col2 = Vec3(2.0, 0.0, 0.0);
    let col3 = Vec3(3.0, 0.0, 0.0);
    let m = Matrix3(col1, col2, col3);
    let expected = Vec3(1.0, 2.0, 3.0);

    assert!(m.row1() == expected);
}

#[test]
fn test_get_second_row_of_matrix() {
    let col1 = Vec3(0.0, 1.0, 0.0);
    let col2 = Vec3(0.0, 2.0, 0.0);
    let col3 = Vec3(0.0, 3.0, 0.0);
    let m = Matrix3(col1, col2, col3);
    let expected = Vec3(1.0, 2.0, 3.0);

    assert!(m.row2() == expected);
}

#[test]
fn test_get_third_row_of_matrix() {
    let col1 = Vec3(0.0, 0.0, 1.0);
    let col2 = Vec3(0.0, 0.0, 2.0);
    let col3 = Vec3(0.0, 0.0, 3.0);
    let m = Matrix3(col1, col2, col3);
    let expected = Vec3(1.0, 2.0, 3.0);

    assert!(m.row3() == expected);
}

#[test]
fn test_multiply_vector_by_matrix() {
    let col1 = Vec3(1.0, 0.0, 0.0);
    let col2 = Vec3(1.0, -1.0, 1.0);
    let col3 = Vec3(1.0, 1.0, 1.0);
    let m = Matrix3(col1, col2, col3);
    let v = Vec3(1.0, 1.0, 1.0);
    let expected = Vec3(3.0, 0.0, 2.0);

    assert!(m * v == expected);
}

#[test]
fn test_matrix_multiplication() {
    let col1_1 = Vec3(3.0, 2.0, 0.0);
    let col1_2 = Vec3(0.0, 0.0, 1.0);
    let col1_3 = Vec3(2.0, -2.0, 1.0);
    let m_1 = Matrix3(col1_1, col1_2, col1_3);

    let col2_1 = Vec3(0.2, -0.2, 0.2);
    let col2_2 = Vec3(0.2, 0.3, -0.3);
    let col2_3 = Vec3(0.0, 1.0, 0.0);
    let m_2 = Matrix3(col2_1, col2_2, col2_3);
    let tolerance = 0.00000001;

    assert!(Matrix3::dist(m_1 * m_2, Matrix3::identity()) < tolerance);
    assert!(Matrix3::dist(m_2 * m_1, Matrix3::identity()) < tolerance);
}

#[test]
fn test_matrix_distance() {
    let zero = Vec3::zero();
    let m1 = Matrix3::with_columns(zero, zero, zero);
    let m2 = Matrix3::identity();

    assert!(Matrix3::dist(m1, m2) == 3.0);
}
