use crate::ray::Ray;
use straal::{FloatType, Vec3};

pub struct Camera<T> {
    pub origin: Vec3<T>,
    pub lower_left_corner: Vec3<T>,
    pub horizontal: Vec3<T>,
    pub vertical: Vec3<T>,
    pub aspect_ratio: T,
}

impl<T> Camera<T>
where
    T: FloatType<T>,
{
    //Negative Z points forward
    pub fn new(width: T, height: T, z_near: T, origin: Vec3<T>) -> Camera<T> {
        let width_axis = Vec3 {
            x: width,
            y: T::zero(),
            z: T::zero(),
        };

        let height_axis = Vec3 {
            x: T::zero(),
            y: height,
            z: T::zero(),
        };

        Camera {
            origin,
            lower_left_corner: (-width_axis / T::from(2.0).unwrap()
                - height_axis / T::from(2.0).unwrap())
                - Vec3 {
                    x: T::zero(),
                    y: T::zero(),
                    z: z_near,
                },
            horizontal: width_axis,
            vertical: height_axis,
            aspect_ratio: height / width,
        }
    }

    pub fn get_ray(&self, u: T, v: T) -> Ray<T>
    where
        T: FloatType<T>,
    {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + self.horizontal * u + self.vertical * v,
        }
    }
}
