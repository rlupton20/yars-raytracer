// shade.rs - things that can be shaded, and how to shade them
use vector3d::Vec3;
use ray::{Intersectable, Orientable};
use materials::HasMaterial;
use scene::{ Scene, Light };

pub trait Shadable : Intersectable + Orientable + HasMaterial {}


