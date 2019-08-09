pub mod hittable;
pub mod scene;
pub mod sphere;

use crate::material::{DummyMaterial, Material};
pub use hittable::*;
pub use scene::*;
pub use sphere::*;
use std::sync::Weak;
use straal::{FloatType, Vec3};

#[derive(Clone)]
pub struct HitRecord<T> {
    pub t: T,
    pub position: Vec3<T>,
    pub normal: Vec3<T>,
    pub material: Weak<dyn Material<T>>,
}

impl<T> HitRecord<T>
where
    T: FloatType<T> + Send + Sync,
{
    pub fn default() -> HitRecord<T> {
        HitRecord {
            t: T::from(0).unwrap(),
            position: Vec3::zero(),
            normal: Vec3::zero(),
            material: Weak::<DummyMaterial>::new(),
        }
    }
}
