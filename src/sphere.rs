use super::ray::Ray;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use std::fmt::Debug;
use std::sync::Arc;
use straal::{FloatType, Vec3};

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

    fn fill_hit_record(&self, solution: T, r: &Ray<T>, record: &mut HitRecord<T>) {
        record.t = solution;
        record.p = r.point_at_parameter(solution);
        record.n = (record.p - self.center) / self.radius;
        record.material = Arc::downgrade(&self.material);
    }
}

impl<T> Hittable<T> for Sphere<T>
where
    T: FloatType<T> + Debug + Send + Sync,
{
    fn hit(&self, r: &Ray<T>, t_min: T, t_max: T, record: &mut HitRecord<T>) -> bool {
        let oc = r.get_origin() - self.center;
        let a = r.get_direction().dot(r.get_direction());
        let b = T::from(2).unwrap() * oc.dot(r.get_direction());
        let c = oc.dot(oc) - self.radius * self.radius;

        let discriminant = b * b - T::from(4).unwrap() * a * c;

        if discriminant > T::zero() {
            let sqrt_d = T::sqrt(discriminant);

            let sol1 = (-b - sqrt_d) / (T::from(2).unwrap() * a);
            if sol1 < t_max && sol1 > t_min {
                self.fill_hit_record(sol1, r, record);
                return true;
            }

            let sol2 = (-b + sqrt_d) / (T::from(2).unwrap() * a);
            if sol2 < t_max && sol2 > t_min {
                self.fill_hit_record(sol2, r, record);
                return true;
            }
        }
        false
    }
}
