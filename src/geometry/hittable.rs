use straal::FloatType;

use crate::geometry::{AABB, HitRecord};
use crate::math::Ray;

pub trait Hittable<T>: Send + Sync
    where
        T: FloatType<T> + Send + Sync,
{
    fn hit(&self, r: &Ray<T>, t_min: T, t_max: T, record: &mut HitRecord<T>) -> bool;
    fn bounding_box(&self, t0: T, t1: T) -> Option<AABB<T>>;
}
