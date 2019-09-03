use straal::FloatType;

pub fn schlick<T>(cosine: T, ref_idx: T) -> T
where
    T: FloatType<T>,
{
    let r0_sqrt = (T::one() - ref_idx) / (T::one() + ref_idx);
    let r0 = r0_sqrt * r0_sqrt;
    r0 + (T::one() - r0) * (T::one() - cosine).powi(5)
}
