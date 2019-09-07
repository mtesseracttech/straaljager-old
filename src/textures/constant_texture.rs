use straal::{FloatType, Vec3};

use crate::textures::Texture;

pub struct ConstantTexture<T> {
    pub color: T
}

impl<T> ConstantTexture<T> where T: FloatType<T> {
    pub fn default() -> ConstantTexture<T> {
        ConstantTexture {
            color: Vec3::<T>::zero()
        }
    }

    pub fn new(c: &Vec3<T>) -> ConstantTexture<T> {
        ConstantTexture {
            color: c.clone()
        }
    }
}

impl<T> Texture for ConstantTexture<T> where T: FloatType<T> {
    fn sample_color(&self, u: T, v: T, p: &Vec3<T>) -> Vec3<T> {
        self.color
    }
}