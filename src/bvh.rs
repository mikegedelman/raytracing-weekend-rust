use crate::vec3::*;
use crate::ray::*;

#[derive(Clone, Copy)]
pub struct AABB {
    pub minimum: Point3,
    pub maximum: Point3,
}

impl AABB {
    pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
            let small = Point3::new(f32::min(box0.minimum.x(), box1.minimum.x()),
                         f32::min(box0.minimum.y(), box1.minimum.y()),
                         f32::min(box0.minimum.z(), box1.minimum.z()));
        
            let big = Point3::new(f32::max(box0.maximum.x(), box1.maximum.x()),
                       f32::max(box0.maximum.y(), box1.maximum.y()),
                       f32::max(box0.maximum.z(), box1.maximum.z()));
        
            return AABB {
                minimum: small,
                maximum: big
            };
    }

    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / ray.dir[a];
            let mut t0 = (self.minimum[a] - ray.orig[a]) * inv_d;
            let mut t1 = (self.maximum[a] - ray.orig[a]) * inv_d;
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            let t_min_p = if t0 > t_min { t0 } else { t_min };  // p -> prime
            let t_max_p = if t1 < t_max { t1 } else { t_max };

            if t_max_p <= t_min_p {
                return false;
            }
        }

        true
    }
}