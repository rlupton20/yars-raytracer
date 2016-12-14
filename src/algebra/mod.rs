// algebra - define algebraic structures
use std::ops::{Add, Mul};
use std::marker::Sized;
use std::cmp::Ordering;

pub trait VectorSpace : Add + Sized {
    type Field: Mul<Self>;
}


pub trait InnerProductSpace : VectorSpace {
    fn dot(self, rhs : Self) -> Self::Field;
}


pub trait Group : Mul + Sized {
    fn identity() -> Self;
    fn inverse(self) -> Self;
}

pub trait GroupAction<T> : Group + Mul<T> {}


#[derive(PartialEq, PartialOrd)]
pub struct Real(f64);

impl Real {
    pub fn from_float(x : f64) -> Option<Real> {
        if x.is_nan() { None } else { Some(Real(x)) }
    }

    pub fn zero() -> Real {
        Real(0.0)
    }
}

impl Eq for Real {}

impl Ord for Real {
    fn cmp(&self, other : &Real) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ordering_on_real() {
        let x = Real::from_float(3.4).unwrap();
        let y = Real::from_float(8.6).unwrap();
        assert!(x < y);
        assert!(!(y < x));
        assert!(x!=y);
        assert!(x==x);
    }
}
        
