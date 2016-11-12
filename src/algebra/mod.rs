// algebra - define algebraic structures
use std::ops::{Add, Mul};
use std::marker::Sized;

pub trait VectorSpace : Add + Sized {
    type Field: Mul<Self>;
}


pub trait InnerProductSpace : VectorSpace {
    fn dot(&self, rhs : &Self) -> Self::Field;
}


pub trait Group : Mul + Sized {
    fn identity() -> Self;
    fn inverse(self) -> Self;
}

pub trait GroupAction<T> : Group + Mul<T> {}
