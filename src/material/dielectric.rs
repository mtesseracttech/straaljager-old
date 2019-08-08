use crate::geometry::HitRecord;
use crate::material::{refract, schlick, Material};
use crate::math::Ray;
use rand::Rng;
use straal::{FloatType, Vec3};

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

        let outward_normal;
        let ni_over_nt;
        let cosine;
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
