use crate::geometry::HitRecord;
use crate::material::Material;
use crate::math::{random_in_unit_sphere, Ray};
use straal::{FloatType, Vec3};

pub struct LambertianMaterial<T> {
    pub albedo: Vec3<T>,
}

impl<T> LambertianMaterial<T>
where
    T: FloatType<T>,
{
    pub fn create(albedo: &Vec3<T>) -> LambertianMaterial<T> {
        LambertianMaterial { albedo: *albedo }
    }
}

impl<T> Material<T> for LambertianMaterial<T>
where
    T: FloatType<T> + Send + Sync,
{
    fn scatter(
        &self,
        _r: &Ray<T>,
        record: &mut HitRecord<T>,
        attenuation: &mut Vec3<T>,
        scattered: &mut Ray<T>,
    ) -> bool {
        let target = record.position + record.normal + random_in_unit_sphere();
        scattered.origin = record.position;
        scattered.direction = target - record.position;
        attenuation.x = self.albedo.x;
        attenuation.y = self.albedo.y;
        attenuation.z = self.albedo.z;
        true
    }
}
