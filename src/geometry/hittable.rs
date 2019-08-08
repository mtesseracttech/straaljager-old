use straal::FloatType;

use crate::geometry::HitRecord;
use crate::math::Ray;

pub trait Hittable<T>: Send + Sync
where
    T: FloatType<T> + Send + Sync,
{
    fn hit(&self, r: &Ray<T>, t_min: T, t_max: T, record: &mut HitRecord<T>) -> bool;
}
