use std::fmt::Debug;
use std::sync::Arc;

use rand::Rng;
use straal::FloatType;

use crate::geometry::{AABB, HitRecord, Hittable};
use crate::math::Ray;

pub struct BvhNode<T> where T: Sync + Send {
    pub left: Arc<dyn Hittable<T>>,
    pub right: Arc<dyn Hittable<T>>,
    pub aabb: AABB<T>,
}

impl<T> BvhNode<T> where T: FloatType<T> + Sync + Send + Debug + 'static {
    pub fn new(list: &mut [Arc<dyn Hittable<T> + Send + Sync>], time0: T, time1: T) -> BvhNode<T> {
        match rand::thread_rng().gen_range(0, 3) {
            0 => {
                list.sort_unstable_by(|a, b| {
                    let box_left = (**a).bounding_box(T::zero(), T::zero());
                    let box_right = (**b).bounding_box(T::zero(), T::zero());
                    if box_left.is_none() || box_right.is_none() { println!("No bounding box in the BVH node constructor"); }
                    let box_left = box_left.unwrap();
                    let box_right = box_right.unwrap();
                    box_left.get_min().x.partial_cmp(&box_right.get_min().x).unwrap()
                })
            }
            1 => {
                list.sort_unstable_by(|a, b| {
                    let box_left = (**a).bounding_box(T::zero(), T::zero());
                    let box_right = (**b).bounding_box(T::zero(), T::zero());
                    if box_left.is_none() || box_right.is_none() { println!("No bounding box in the BVH node constructor"); }
                    let box_left = box_left.unwrap();
                    let box_right = box_right.unwrap();
                    box_left.get_min().y.partial_cmp(&box_right.get_min().y).unwrap()
                })
            }
            _ => {
                list.sort_unstable_by(|a, b| {
                    let box_left = (**a).bounding_box(T::zero(), T::zero());
                    let box_right = (**b).bounding_box(T::zero(), T::zero());
                    if box_left.is_none() || box_right.is_none() { println!("No bounding box in the BVH node constructor"); }
                    let box_left = box_left.unwrap();
                    let box_right = box_right.unwrap();
                    box_left.get_min().z.partial_cmp(&box_right.get_min().z).unwrap()
                })
            }
        }

        let mut new_node = match list.len() {
            0 => {
                panic!("No entries in the BVH, what are you trying to do?");
            }
            1 => {
                BvhNode::<T> {
                    left: list[0].clone(),
                    right: list[0].clone(),
                    aabb: AABB::default(),
                }
            }
            2 => {
                BvhNode::<T> {
                    left: list[0].clone(),
                    right: list[1].clone(),
                    aabb: AABB::default(),
                }
            }
            _ => {
                let div = list.split_at_mut(list.len() / 2);
                BvhNode::<T> {
                    left: Arc::new(BvhNode::new(div.0, time0, time1)),
                    right: Arc::new(BvhNode::new(div.1, time0, time1)),
                    aabb: AABB::default(),
                }
            }
        };

        let box_left = (*new_node.left).bounding_box(time0, time1);
        let box_right = (*new_node.right).bounding_box(time0, time1);

        if box_left.is_none() || box_right.is_none() {
            println!("No bounding box in BVH Constructor");
        }
        new_node.aabb = AABB::surrounding_box(&box_left.unwrap(), &box_right.unwrap());
        new_node
    }
}

impl<T> Hittable<T> for BvhNode<T> where T: FloatType<T> + Sync + Send {
    fn hit(&self, r: &Ray<T>, t_min: T, t_max: T, record: &mut HitRecord<T>) -> bool {
        if self.aabb.hit(r, t_min, t_max) {
            let mut left_rec = HitRecord::<T>::default();
            let mut right_rec = HitRecord::<T>::default();

            let hit_left = (*self.left).hit(r, t_min, t_max, &mut left_rec);
            let hit_right = (*self.right).hit(r, t_min, t_max, &mut right_rec);

            if hit_left & &hit_right {
                if left_rec.t < right_rec.t {
                    record.update(left_rec);
                } else {
                    record.update(right_rec);
                }
                return true;
            } else if hit_left {
                record.update(left_rec);
                return true;
            } else if hit_right {
                record.update(right_rec);
                return true;
            }
            return false;
        }
        return false;
    }

    fn bounding_box(&self, _t0: T, _t1: T) -> Option<AABB<T>> {
        Some(self.aabb.clone())
    }
}