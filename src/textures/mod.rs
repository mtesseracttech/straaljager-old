use straal::{FloatType, Vec3};

pub use constant_texture::*;

pub mod constant_texture;

pub trait Texture<T>: Send + Sync where T: FloatType<T> {
    fn sample_color(&self, u: T, v: T, p: &Vec3<T>) -> Vec3<T>;
}