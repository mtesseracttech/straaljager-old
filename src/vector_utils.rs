use straal::{FloatType, IVec3, Vec3, Vec3n};

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
