use straal::{FloatType, Vec3};

use crate::math::{random_in_unit_disk, Ray};

pub struct Camera<T> {
    pub origin: Vec3<T>,
    pub lower_left_corner: Vec3<T>,
    pub horizontal: Vec3<T>,
    pub vertical: Vec3<T>,
    pub aspect_ratio: T,
    pub lens_radius: T,
    pub forward: Vec3<T>,
    pub up: Vec3<T>,
    pub right: Vec3<T>,
}

impl<T> Camera<T>
    where
        T: FloatType<T>,
{
    pub fn new(look_from: Vec3<T>, look_at: Vec3<T>, v_up: Vec3<T>, vertical_fov: T, aspect_ratio: T, aperture: T, focus_distance: T) -> Camera<T> {
        let theta = vertical_fov.to_radians();
        let half_height = (theta / T::from(2).unwrap()).tan();
        let half_width = aspect_ratio * half_height;

        let w = (look_from - look_at).normalized();
        let u = (v_up.cross(w)).normalized();
        let v = w.cross(u);
        Camera {
            origin: look_from,
            lower_left_corner: look_from - u * focus_distance * half_width - v * focus_distance * half_height - w * focus_distance,
            horizontal: u * T::from(2).unwrap() * half_width * focus_distance,
            vertical: v * T::from(2).unwrap() * half_height * focus_distance,
            aspect_ratio,
            lens_radius: aperture / T::from(2).unwrap(),
            forward: w,
            up: v,
            right: u,
        }
    }

    pub fn get_ray(&self, x: T, y: T) -> Ray<T>
        where
            T: FloatType<T>,
    {
        let random_dist = random_in_unit_disk() * self.lens_radius.clone();
        let offset = self.right * random_dist.x + self.up * random_dist.y;
        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + self.horizontal * x + self.vertical * y - self.origin,
        }
    }
}
