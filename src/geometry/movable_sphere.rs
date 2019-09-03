use std::fmt::Debug;
use std::sync::Arc;

use straal::{FloatType, Vec3};

use crate::geometry::{HitRecord, Hittable};
use crate::material::Material;
use crate::math::Ray;

pub struct MovableSphere<T> {
    pub center0: Vec3<T>,
    pub center1: Vec3<T>,
    pub time0: T,
    pub time1: T,
    pub radius: T,
    pub material: Arc<dyn Material<T>>,
}

impl<T> MovableSphere<T>
    where
        T: FloatType<T>,
{
    pub fn get_center(&self, time: T) -> Vec3<T> {
        self.center0 + (self.center1 - self.center0) * ((time - self.time0) / (self.time1 - self.time0))
    }

    pub fn get_radius(&self) -> T {
        self.radius
    }
}

impl<T> Hittable<T> for MovableSphere<T>
    where
        T: FloatType<T> + Debug + Send + Sync,
{
    fn hit(&self, r: &Ray<T>, t_min: T, t_max: T, record: &mut HitRecord<T>) -> bool {
        let oc = r.origin - self.get_center(r.get_time());
        let a = Vec3::dot(r.direction, r.direction);
        let b = Vec3::dot(oc, r.direction);
        let c = Vec3::dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > T::zero() {
            let sqrt_d = discriminant.sqrt();
            let sol = (-b - sqrt_d) / a;
            if sol < t_max && sol > t_min {
                record.t = sol;
                record.position = r.point_at_parameter(sol);
                record.normal = (record.position - self.get_center(r.get_time())) / self.radius;
                record.material = Arc::downgrade(&self.material);
                return true;
            }
            let sol = (-b + sqrt_d) / a;
            if sol < t_max && sol > t_min {
                record.t = sol;
                record.position = r.point_at_parameter(sol);
                record.normal = (record.position - self.get_center(r.get_time())) / self.radius;
                record.material = Arc::downgrade(&self.material);
                return true;
            }
        }
        return false;
    }
}