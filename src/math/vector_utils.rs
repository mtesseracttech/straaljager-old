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

pub fn random_in_unit_sphere<T>() -> Vec3<T>
where
    T: FloatType<T>,
{
    let mut rng = rand::thread_rng();
    let mut p = Vec3::<T>::zero();
    let mut length_sqr = T::max_value();
    while length_sqr >= T::one() {
        p = Vec3::<T> {
            x: T::from(rng.gen_range(0.0, 1.0)).unwrap(),
            y: T::from(rng.gen_range(0.0, 1.0)).unwrap(),
            z: T::from(rng.gen_range(0.0, 1.0)).unwrap(),
        } * T::from(2).unwrap()
            - Vec3::<T>::one();
        length_sqr = p.length_squared();
    }
    p
}
