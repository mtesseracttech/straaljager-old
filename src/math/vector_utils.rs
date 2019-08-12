use rand::Rng;
use straal::{FloatType, Vec3};

pub fn gamma_color<T>(v: &Vec3<T>) -> Vec3<T>
    where
        T: FloatType<T>,
{
    Vec3::<T> {
        x: v.x.sqrt(),
        y: v.y.sqrt(),
        z: v.z.sqrt(),
    }
}

pub fn random_in_unit_sphere<T>() -> Vec3<T> where T: FloatType<T> {
    let mut rng = rand::thread_rng();
    let mut p = Vec3::<T>::new(
        rng.gen_range(-1.0, 1.0),
        rng.gen_range(-1.0, 1.0),
        rng.gen_range(-1.0, 1.0));
    while p.length_squared() >= T::one() {
        p.x = T::from(rng.gen_range(-1.0, 1.0)).unwrap();
        p.y = T::from(rng.gen_range(-1.0, 1.0)).unwrap();
        p.z = T::from(rng.gen_range(-1.0, 1.0)).unwrap();
    }
    p
}

pub fn random_in_unit_disk<T>() -> Vec3<T> where T: FloatType<T> {
    let mut rng = rand::thread_rng();
    let mut p = Vec3::<T>::new(
        rng.gen_range(-1.0, 1.0),
        rng.gen_range(-1.0, 1.0),
        0.0);
    while p.length_squared() >= T::one() {
        p.x = T::from(rng.gen_range(-1.0, 1.0)).unwrap();
        p.y = T::from(rng.gen_range(-1.0, 1.0)).unwrap();
        p.z = T::zero();
    }
    p
}
