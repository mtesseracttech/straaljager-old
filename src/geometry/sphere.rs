use std::fmt::Debug;
use std::sync::Arc;

use straal::{FloatType, Vec3};

use crate::geometry::{HitRecord, Hittable};
use crate::material::Material;
use crate::math::Ray;

pub struct Sphere<T> {
    pub center: Vec3<T>,
    pub radius: T,
    pub material: Arc<dyn Material<T>>,
}

impl<T> Sphere<T>
    where
        T: FloatType<T>,
{
    pub fn get_center(&self) -> Vec3<T> {
        self.center
    }

    pub fn get_radius(&self) -> T {
        self.radius
    }
}

impl<T> Hittable<T> for Sphere<T>
    where
        T: FloatType<T> + Debug + Send + Sync,
{
    fn hit(&self, r: &Ray<T>, t_min: T, t_max: T, record: &mut HitRecord<T>) -> bool {
        let oc = r.origin - self.center;
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
                record.normal = (record.position - self.center) / self.radius;
                record.material = Arc::downgrade(&self.material);
                return true;
            }
            let sol = (-b + sqrt_d) / a;
            if sol < t_max && sol > t_min {
                record.t = sol;
                record.position = r.point_at_parameter(sol);
                record.normal = (record.position - self.center) / self.radius;
                record.material = Arc::downgrade(&self.material);
                return true;
            }
        }
        return false;
    }
}