use crate::geometry::{HitRecord, Hittable};
use crate::math::Ray;
use std::sync::Arc;
use straal::FloatType;

pub struct HittableScene<T> {
    pub hittable_list: Vec<Arc<dyn Hittable<T>>>,
}

impl<T> HittableScene<T>
where
    T: FloatType<T>,
{
    pub fn new() -> HittableScene<T> {
        return HittableScene {
            hittable_list: Vec::new(),
        };
    }

    pub fn add_hittable(&mut self, hittable: Arc<dyn Hittable<T> + Send + Sync>) {
        self.hittable_list.push(hittable);
    }
}

impl<T> Hittable<T> for HittableScene<T>
where
    T: FloatType<T> + Send + Sync,
{
    fn hit(&self, r: &Ray<T>, t_min: T, t_max: T, record: &mut HitRecord<T>) -> bool {
        let mut temp_rec = HitRecord::<T>::default();
        let mut closest_so_far = t_max;
        let mut hit_anything = false;
        for hittable in &self.hittable_list {
            if hittable.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                record.position = temp_rec.position;
                record.normal = temp_rec.normal;
                record.t = temp_rec.t;
                record.material = temp_rec.material.clone();
            }
        }
        hit_anything
    }
}
