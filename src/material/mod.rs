use straal::{FloatType, Vec3};

pub mod dielectric;
pub mod lambertian;
pub mod metal;

use crate::geometry::HitRecord;
use crate::math::Ray;
pub use dielectric::*;
pub use lambertian::*;
pub use metal::*;

pub trait Material<T>: Send + Sync
where
    T: FloatType<T> + Send + Sync,
{
    fn scatter(
        &self,
        r: &Ray<T>,
        record: &mut HitRecord<T>,
        attenuation: &mut Vec3<T>,
        scattered: &mut Ray<T>,
    ) -> bool;
}

pub struct DummyMaterial;

impl<T> Material<T> for DummyMaterial
where
    T: FloatType<T> + Send + Sync,
{
    fn scatter(
        &self,
        _r: &Ray<T>,
        _record: &mut HitRecord<T>,
        _attenuation: &mut Vec3<T>,
        _scattered: &mut Ray<T>,
    ) -> bool {
        println!("Drawing using dummy material now, this should not happen!");
        return false;
    }
}
