use core::{fmt, ops::*};
use std::iter::Sum;

use crate::util::*;

///
/// Ideas for SIMD stuff eventually: https://docs.rs/glam/0.8.7/src/glam/f32/vec3.rs.html#24
///

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default)]
// pub struct Vec3(pub(crate) f32, pub(crate) f32, pub(crate) f32);
pub struct Vec3 {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    #[inline]
    pub fn dot(u: &Self, v: &Self) -> f32 {
        (u.x * v.x) + (u.y * v.y) + (u.z * v.z)
    }

    #[inline]
    pub fn cross(u: &Self, v: &Self) -> Self {
        Self {
            x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x,
        }
    }

    #[inline]
    pub fn unit_vector(v: &Self) -> Self {
        (*v) / v.length()
    }

    #[inline]
    pub fn random() -> Vec3 {
        Vec3 {
            x: random_f32(),
            y: random_f32(),
            z: random_f32(),
        }
    }

    #[inline]
    pub fn random_range(min: f32, max: f32) -> Vec3 {
        Vec3 {
            x: random_f32_range(min, max),
            y: random_f32_range(min, max),
            z: random_f32_range(min, max),
        }
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

    // Member functions
    #[inline]
    pub fn length(&self) -> f32 {
        f32::sqrt(self.length_squared())
    }

    #[inline]
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn near_zero(&self) -> bool {
        self.x.abs() < f32::MIN_POSITIVE
            && self.y.abs() < f32::MIN_POSITIVE
            && self.z.abs() < f32::MIN_POSITIVE
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

impl Sum for Vec3 {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Vec3>,
    {
        iter.fold(Vec3::zero(), |a, b| a + b)
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Self;
    #[inline]
    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl DivAssign<Vec3> for Vec3 {
    #[inline]
    fn div_assign(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;
    #[inline]
    fn div(self, other: f32) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl DivAssign<f32> for Vec3 {
    #[inline]
    fn div_assign(&mut self, other: f32) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;
    #[inline]
    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl MulAssign<Vec3> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl MulAssign<f32> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, other: f32) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

// impl AsRef<[f32; 3]> for Vec3 {
//     #[inline]
//     fn as_ref(&self) -> &[f32; 3] {
//         unsafe { &*(self as *const Vec3 as *const [f32; 3]) }
//     }
// }
//
// impl AsMut<[f32; 3]> for Vec3 {
//     #[inline]
//     fn as_mut(&mut self) -> &mut [f32; 3] {
//         unsafe { &mut *(self as *mut Vec3 as *mut [f32; 3]) }
//     }
// }

impl Index<usize> for Vec3 {
    type Output = f32;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds: {}", index),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds: {}", index),
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    #[inline]
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Sub for Vec3 {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl SubAssign for Vec3 {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl Neg for Vec3 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
