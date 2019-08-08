use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vector_utils::random_in_unit_sphere;
use rand::Rng;
use straal::{FloatType, Vec3};

pub trait Material<T>: Send + Sync
where
    T: FloatType<T> + Send + Sync,
{
    fn scatter(
        &self,
        r: &Ray<T>,
        record: &mut HitRecord<T>,
        attenuation: &mut Vec3<T>,
        scattered: &mut Ray<T>,
    ) -> bool;
}

pub struct DummyMaterial;

impl<T> Material<T> for DummyMaterial
where
    T: FloatType<T> + Send + Sync,
{
    fn scatter(
        &self,
        _r: &Ray<T>,
        _record: &mut HitRecord<T>,
        _attenuation: &mut Vec3<T>,
        _scattered: &mut Ray<T>,
    ) -> bool {
        println!("Drawing using dummy material now, this should not happen!");
        return false;
    }
}

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
        let target = record.p + record.n + random_in_unit_sphere();
        scattered.origin = record.p;
        scattered.direction = target - record.p;
        attenuation.x = self.albedo.x;
        attenuation.y = self.albedo.y;
        attenuation.z = self.albedo.z;
        true
    }
}

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
        let reflected = Vec3::<T>::reflect(r.direction.normalized(), record.n);
        scattered.origin = record.p;
        scattered.direction = reflected + random_in_unit_sphere() * self.roughness;
        attenuation.x = self.albedo.x;
        attenuation.y = self.albedo.y;
        attenuation.z = self.albedo.z;
        scattered.direction.dot(record.n) > T::zero()
    }
}

pub struct DielectricMaterial<T> {
    pub refractive_index: T,
}

impl<T> DielectricMaterial<T>
where
    T: FloatType<T>,
{
    pub fn create(refractive_index: T) -> DielectricMaterial<T> {
        DielectricMaterial { refractive_index }
    }
}

impl<T> Material<T> for DielectricMaterial<T>
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
        let reflected = Vec3::<T>::reflect(r.direction, record.n);

        attenuation.x = T::one();
        attenuation.y = T::one();
        attenuation.z = T::one();

        let mut outward_normal;
        let mut ni_over_nt;
        let mut cosine;
        if r.direction.dot(record.n) > T::zero() {
            outward_normal = -record.n;
            ni_over_nt = self.refractive_index;
            cosine = self.refractive_index * r.direction.dot(record.n) / r.direction.length();
        } else {
            outward_normal = record.n;
            ni_over_nt = T::one() / self.refractive_index;
            cosine = -r.direction.dot(record.n) / r.direction.length();
        }

        let reflect_prob;
        let mut refracted = Vec3::<T>::zero();
        match refract(r.direction, outward_normal, ni_over_nt) {
            Some(refract) => {
                reflect_prob = schlick(cosine, self.refractive_index);
                refracted = refract;
            }
            None => {
                reflect_prob = T::one();
                scattered.origin = record.p;
                scattered.direction = reflected;
            }
        }

        let mut rng = rand::thread_rng();
        if T::from(rng.gen_range(0.0, 1.0)).unwrap() < reflect_prob {
            scattered.origin = record.p;
            scattered.direction = reflected;
        } else {
            scattered.origin = record.p;
            scattered.direction = refracted;
        }
        true
    }
}

pub fn refract<S>(i: Vec3<S>, n: Vec3<S>, ni_over_nt: S) -> Option<Vec3<S>>
where
    S: FloatType<S>,
{
    let unit_i = i.normalized();
    let dt = unit_i.dot(n);
    let discriminant = S::one() - ni_over_nt * ni_over_nt * (S::one() - dt * dt);
    if discriminant > S::zero() {
        Some((i - n * dt) * ni_over_nt - n * discriminant.sqrt())
    } else {
        None
    }
}

pub fn schlick<T>(cosine: T, ref_idx: T) -> T
where
    T: FloatType<T>,
{
    let r0_sqrt = (T::one() - ref_idx) / (T::one() + ref_idx);
    let r0 = r0_sqrt * r0_sqrt;
    r0 + (T::one() - r0) * (T::one() - cosine).powi(5)
}
