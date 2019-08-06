use straal::{FloatType, Vec3};

use super::ray::Ray;
use std::fmt::Debug;

#[derive(Copy, Clone, Debug, Hash)]
pub struct HitRecord<T> {
    pub t: T,
    pub p: Vec3<T>,
    pub n: Vec3<T>,
}

impl<T> HitRecord<T>
where
    T: FloatType<T>,
{
    pub fn default() -> HitRecord<T> {
        HitRecord {
            t: T::from(0).unwrap(),
            p: Vec3::zero(),
            n: Vec3::zero(),
        }
    }
}

pub trait Hittable<T>
where
    T: FloatType<T>,
{
    fn hit(&self, r: &Ray<T>, t_min: T, t_max: T, record: &mut HitRecord<T>) -> bool;
}

pub struct HittableScene<T> {
    pub hittable_list: Vec<Box<dyn Hittable<T>>>,
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

    pub fn add_hittable(&mut self, hittable: Box<dyn Hittable<T>>) {
        self.hittable_list.push(hittable);
    }
}

impl<T> Hittable<T> for HittableScene<T>
where
    T: FloatType<T> + Debug,
{
    fn hit(&self, r: &Ray<T>, t_min: T, t_max: T, record: &mut HitRecord<T>) -> bool {
        let mut temp_rec = HitRecord::<T>::default();
        let mut closest_so_far = t_max;
        let mut hit_anything = false;
        for hittable in &self.hittable_list {
            if hittable.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                record.p = temp_rec.p;
                record.n = temp_rec.n;
                record.t = temp_rec.t;
            }
        }
        hit_anything
    }
}
