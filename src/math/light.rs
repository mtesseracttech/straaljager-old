use straal::FloatType;

pub fn schlick<T>(cosine: T, ref_idx: T) -> T
    where
        T: FloatType<T>,
{
    let r0_sqrt = (T::one() - ref_idx) / (T::one() + ref_idx);
    let r0 = r0_sqrt * r0_sqrt;
    r0 + (T::one() - r0) * (T::one() - cosine).powi(5)
}


/*
bool refract(const vec3& v, const vec3& n, float ni_over_nt, vec3& refracted) {
    vec3 uv = unit_vector(v);
    float dt = dot(uv, n);
    float discriminant = 1.0 - ni_over_nt*ni_over_nt*(1-dt*dt);
    if (discriminant > 0) {
        refracted = ni_over_nt*(uv - n*dt) - n*sqrt(discriminant);
        return true;
    }
    else
        return false;
}
*/