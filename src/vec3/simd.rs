use core::{fmt, ops::*};
use std::iter::Sum;
use std::simd::{f32x4,Simd};

#[derive(Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Vec3 {
   val: f32x4
}


impl Vec3 {
    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { val: Simd::from([x, y, z, 0.0]) }
    }

    #[inline]
    pub fn zero() -> Self {
        Self { val: Simd::from([0.0, 0.0, 0.0, 0.0]) }
    }

    #[inline]
    pub fn x(&self) -> f32 {
        self.val[0]
    }

    #[inline]
    pub fn y(&self) -> f32 {
        self.val[1]
    }

    #[inline]
    pub fn z(&self) -> f32 {
        self.val[2]
    }

    #[inline]
    pub fn dot(u: &Self, v: &Self) -> f32 {
        // TODO: maybe use SIMD primitives for this
        (u.x() * v.x()) + (u.y() * v.y()) + (u.z() * v.z())
    }

    #[inline]
    pub fn cross(u: &Self, v: &Self) -> Self {
        // TODO: maybe use SIMD primitives for this
        Self {
            val: Simd::from([
                u.y() * v.z() - u.z() * v.y(),
                u.z() * v.x() - u.x() * v.z(),
                u.x() * v.y() - u.y() * v.x(),
                0.0,
            ]),
        }
    }

}

// Custom impl for Debug instead of the derive here, otherwise
// we get ugly output due to the f32x4 val field.
impl fmt::Debug for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Vec3")
         .field("x", &self.val[0])
         .field("y", &self.val[1])
         .field("z", &self.val[2])
         .finish()
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x(), self.y(), self.z())
    }
}

impl Sum for Vec3 {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::zero(), |a, b| a + b)
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Self;
    #[inline]
    fn div(self, other: Self) -> Self {
        Self {
            val: self.val / other.val
        }
    }
}

impl DivAssign<Vec3> for Vec3 {
    #[inline]
    fn div_assign(&mut self, other: Self) {
        self.val /= other.val;
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;
    #[inline]
    fn div(self, other: f32) -> Self {
        Self {
            val: self.val / Simd::from([other, other, other, other])
        }
    }
}

impl DivAssign<f32> for Vec3 {
    #[inline]
    fn div_assign(&mut self, other: f32) {
        self.val /= Simd::from([other, other, other, other]);
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;
    #[inline]
    fn mul(self, other: Self) -> Self {
        Self {
            val: self.val * other.val
        }
    }
}

impl MulAssign<Vec3> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, other: Self) {
        self.val *= other.val;
    }
}


impl Mul<Vec3> for f32 {
    type Output = Vec3;
    #[inline]
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            val: other.val * Simd::from([self, self, self, self])
        }
    }
}

impl MulAssign<f32> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, other: f32) {
        self.val *= Simd::from([other, other, other, other]);
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
        &self.val[index]
    }
}

impl IndexMut<usize> for Vec3 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.val[index]
    }
}

impl Add for Vec3 {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        Self {
            val: self.val + other.val
        }
    }
}

impl AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.val += other.val;
    }
}

impl Sub for Vec3 {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        Self {
            val: self.val - other.val
        }
    }
}

impl SubAssign for Vec3 {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        self.val -= other.val;
    }
}

impl Neg for Vec3 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self {
            val: -self.val
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
