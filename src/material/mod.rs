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

pub fn refract<S>(i: Vec3<S>, n: Vec3<S>, ni_over_nt: S) -> Option<Vec3<S>>
where
    S: FloatType<S>,
{
    let unit_i = i.normalized();
    let dt = unit_i.dot(n);
    let discriminant = S::one() - ni_over_nt * ni_over_nt * (S::one() - dt * dt);
    if discriminant > S::zero() {
        Some((i - n * dt) * ni_over_nt - n * discriminant.sqrt())
    } else {
        None
    }
}

pub fn schlick<T>(cosine: T, ref_idx: T) -> T
where
    T: FloatType<T>,
{
    let r0_sqrt = (T::one() - ref_idx) / (T::one() + ref_idx);
    let r0 = r0_sqrt * r0_sqrt;
    r0 + (T::one() - r0) * (T::one() - cosine).powi(5)
}
