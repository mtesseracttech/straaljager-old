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

pub fn refract<T>(v: Vec3<T>, n: Vec3<T>, ni_over_nt: T) -> Option<Vec3<T>> where T: FloatType<T> {
    let n_dot_i = Vec3::dot(n, v);
    let k = T::one() - ni_over_nt * ni_over_nt * (T::one() - n_dot_i * n_dot_i);
    if k < T::zero() {
        None
    } else {
        Some(v * ni_over_nt - n * (ni_over_nt * n_dot_i + k.sqrt()))
    }
}