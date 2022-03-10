use crate::vec3::*;

pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Ray {
        Ray { orig, dir }
    }

    // Calculate P for P(t) = A + (b * t) where A is the origin of the ray, and b is the direction.
    pub fn at(&self, t: f32) -> Point3 {
        self.orig + t * self.dir
    }
}
