use crate::geometry::{HitRecord, Hittable};
use crate::material::Material;
use crate::math::Ray;
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
        record.position = r.point_at_parameter(record.t);
        record.normal = (record.position - self.center) / self.radius;
        record.material = Arc::downgrade(&self.material);
    }
}

impl<T> Hittable<T> for Sphere<T>
where
    T: FloatType<T> + Debug + Send + Sync,
{
    fn hit(&self, r: &Ray<T>, t_min: T, t_max: T, record: &mut HitRecord<T>) -> bool {
        let oc = r.origin - self.center;
        let a = r.direction.dot(r.direction);
        let b = oc.dot(r.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > T::zero() {
            let sqrt_d = discriminant.sqrt();

            let sol1 = (-b - sqrt_d) / a;
            if sol1 < t_max && sol1 > t_min {
                self.fill_hit_record(sol1, r, record);
                return true;
            }

            let sol2 = (-b + sqrt_d) / a;
            if sol2 < t_max && sol2 > t_min {
                self.fill_hit_record(sol2, r, record);
                return true;
            }
        }
        false
    }
}
