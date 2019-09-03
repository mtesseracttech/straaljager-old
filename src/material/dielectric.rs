use rand::{thread_rng, Rng};
use straal::{FloatType, Vec3};

use crate::geometry::HitRecord;
use crate::material::Material;
use crate::math::{schlick, Ray};

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
        let reflected = Vec3::<T>::reflect(r.direction, record.normal);

        attenuation.x = T::one();
        attenuation.y = T::one();
        attenuation.z = T::one();

        let outward_normal;
        let ni_over_nt;
        let cosine;
        if Vec3::<T>::dot(r.direction, record.normal) > T::zero() {
            outward_normal = -record.normal;
            ni_over_nt = self.refractive_index;
            let tmp_cos = Vec3::<T>::dot(r.direction, record.normal) / r.direction.length();
            cosine = (T::one()
                - self.refractive_index * self.refractive_index * (T::one() - tmp_cos * tmp_cos))
                .sqrt();
        } else {
            outward_normal = record.normal;
            ni_over_nt = T::one() / self.refractive_index;
            cosine = -Vec3::<T>::dot(r.direction, record.normal) / r.direction.length();
        }

        let refracted;
        let reflect_prob;
        match refract(r.direction, outward_normal, ni_over_nt) {
            Some(r) => {
                refracted = r;
                reflect_prob = schlick(cosine, self.refractive_index);
            }
            None => {
                scattered.origin = record.position;
                scattered.direction = reflected;
                refracted = Vec3::<T>::zero();
                reflect_prob = T::one();
            }
        }

        if T::from(thread_rng().gen_range(0.0, 1.0)).unwrap() < reflect_prob {
            scattered.origin = record.position;
            scattered.direction = reflected;
        } else {
            scattered.origin = record.position;
            scattered.direction = refracted;
        }
        true
    }
}

pub fn refract<T>(v: Vec3<T>, n: Vec3<T>, ni_over_nt: T) -> Option<Vec3<T>>
where
    T: FloatType<T>,
{
    let uv = v.normalized();
    let dt = Vec3::dot(uv, n);
    let discriminant = T::one() - ni_over_nt * ni_over_nt * (T::one() - dt * dt);
    if discriminant > T::zero() {
        Some((uv - (n * dt)) * ni_over_nt - n * discriminant.sqrt())
    } else {
        None
    }
}
