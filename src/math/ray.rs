use straal::{FloatType, Vec3};

pub struct Ray<T> {
    pub origin: Vec3<T>,
    pub direction: Vec3<T>,
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

    pub fn point_at_parameter(&self, t: T) -> Vec3<T> {
        return self.origin + (self.direction * t);
    }
}

pub type RayN = Ray<f32>;
pub type RayH = Ray<f64>;
