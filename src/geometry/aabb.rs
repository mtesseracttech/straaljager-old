use std::mem;

use straal::{FloatType, Vec3};

use crate::math::Ray;

#[derive(Clone, Debug)]
pub struct AABB<T> {
    pub min: Vec3<T>,
    pub max: Vec3<T>,
}

impl<T> AABB<T> where T: FloatType<T> {
    pub fn new(min: &Vec3<T>, max: &Vec3<T>) -> AABB<T> {
        AABB { min: min.clone(), max: max.clone() }
    }

    pub fn get_min(&self) -> Vec3<T> {
        self.min
    }

    pub fn get_max(&self) -> Vec3<T> {
        self.max
    }

    pub fn hit(&self, r: &Ray<T>, t_min: T, t_max: T) -> bool {
        for i in 0..3 {
            let inv_d = T::one() / r.get_direction()[i];

            let mut t0 = (self.get_min()[i] - r.get_origin()[i]) * inv_d;
            let mut t1 = (self.get_max()[i] - r.get_origin()[i]) * inv_d;

            if inv_d < T::zero() {
                mem::swap(&mut t0, &mut t1);
            }

            let t_min = if t0 > t_min { t0 } else { t_min };
            let t_max = if t1 < t_max { t1 } else { t_max };

            if t_max <= t_min {
                return false;
            }
        }
        true
    }

    pub fn surrounding_box(box0: &AABB<T>, box1: &AABB<T>) -> AABB<T> {
        let min = Vec3::<T>::new(
            T::min(box0.get_min().x, box1.get_min().x),
            T::min(box0.get_min().y, box1.get_min().y),
            T::min(box0.get_min().z, box1.get_min().z));
        let max = Vec3::<T>::new(
            T::max(box0.get_max().x, box1.get_max().x),
            T::max(box0.get_max().y, box1.get_max().y),
            T::max(box0.get_max().z, box1.get_max().z));
        AABB { min, max }
    }
}

impl<T> Default for AABB<T> where T: FloatType<T> {
    fn default() -> Self {
        AABB {
            min: Vec3::<T>::zero(),
            max: Vec3::<T>::zero(),
        }
    }
}