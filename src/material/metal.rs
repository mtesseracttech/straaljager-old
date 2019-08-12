use straal::{FloatType, Vec3};

use crate::geometry::HitRecord;
use crate::material::Material;
use crate::math::{random_in_unit_sphere, Ray};

pub struct MetalMaterial<T> {
    pub albedo: Vec3<T>,
    pub roughness: T,
}

impl<T> MetalMaterial<T>
    where
        T: FloatType<T>,
{
    pub fn create(albedo: &Vec3<T>, roughness: T) -> MetalMaterial<T> {
        MetalMaterial {
            albedo: *albedo,
            roughness: if roughness > T::one() {
                T::one()
            } else {
                roughness
            },
        }
    }
}

impl<T> Material<T> for MetalMaterial<T>
    where
        T: FloatType<T> + Send + Sync,
{
    fn scatter(
        &self,
        r: &Ray<T>,
        record: &mut HitRecord<T>,
        attenuation: &mut Vec3<T>,
        scattered: &mut Ray<T>,
    ) -> bool {
        let reflected = Vec3::<T>::reflect(r.direction.normalized(), record.normal);
        scattered.origin = record.position;
        scattered.direction = reflected + random_in_unit_sphere() * self.roughness;
        attenuation.x = self.albedo.x;
        attenuation.y = self.albedo.y;
        attenuation.z = self.albedo.z;
        scattered.direction.dot(record.normal) > T::zero()
    }
}