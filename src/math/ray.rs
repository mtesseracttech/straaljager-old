use straal::{FloatType, Vec3};

pub struct Ray<T> {
    pub origin: Vec3<T>,
    pub direction: Vec3<T>,
    pub time: T,
}

impl<T> Ray<T>
    where
        T: FloatType<T>,
{
    pub fn get_origin(&self) -> Vec3<T> {
        self.origin
    }

    pub fn get_direction(&self) -> Vec3<T> {
        self.direction
    }

    pub fn get_time(&self) -> T {
        self.time
    }

    pub fn point_at_parameter(&self, t: T) -> Vec3<T> {
        return self.origin + (self.direction * t);
    }

    pub fn default() -> Ray<T> {
        Ray {
            origin: Vec3::<T>::zero(),
            direction: Vec3::<T>::zero(),
            time: T::zero(),
        }
    }

    pub fn default_with_time(time: T) -> Ray<T> {
        Ray {
            origin: Vec3::<T>::zero(),
            direction: Vec3::<T>::zero(),
            time,
        }
    }
}

pub type RayN = Ray<f32>;
pub type RayH = Ray<f64>;
