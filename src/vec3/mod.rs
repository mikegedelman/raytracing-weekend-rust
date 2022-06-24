
#[cfg(feature = "simd")]
mod simd;
#[cfg(feature = "simd")]
pub use self::simd::Vec3;

#[cfg(not(feature = "simd"))]
mod not_simd;
#[cfg(not(feature = "simd"))]
pub use self::not_simd::Vec3;

use crate::util::*;

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {

    #[inline]
    pub fn unit_vector(v: &Self) -> Self {
        (*v) / v.length()
    }

    #[inline]
    pub fn random() -> Vec3 {
        Vec3::new(random_f32(), random_f32(), random_f32())
    }

    #[inline]
    pub fn random_range(min: f32, max: f32) -> Vec3 {
        Vec3::new(
                random_f32_range(min, max),
                random_f32_range(min, max),
                random_f32_range(min, max),
            )
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }


    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3::new(random_f32_range(-1.0, 1.0), random_f32_range(-1.0, 1.0), 0.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3::unit_vector(&Vec3::random_in_unit_sphere())
    }

    #[inline]
    pub fn length(&self) -> f32 {
        f32::sqrt(self.length_squared())
    }

    #[inline]
    pub fn length_squared(&self) -> f32 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub fn near_zero(&self) -> bool {
        self.x().abs() < f32::MIN_POSITIVE
            && self.y().abs() < f32::MIN_POSITIVE
            && self.z().abs() < f32::MIN_POSITIVE
    }

    pub fn reflect(v: &Vec3, nr: &Vec3) -> Vec3 {
        let n = *nr;
        (*v) - (2.0 * Vec3::dot(v, nr) * n)
    }

    pub fn refract(uvr: &Vec3, nr: &Vec3, etai_over_etat: f32) -> Vec3 {
        let uv = *uvr;
        let n = *nr;

        let cos_theta = f32::min(Vec3::dot(&(-uv), &n), 1.0);
        let r_out_perp = etai_over_etat * (uv + cos_theta*n);
        let r_out_parallel = -f32::sqrt((1.0 - r_out_perp.length_squared()).abs()) * n;

        r_out_perp + r_out_parallel
    }
}
