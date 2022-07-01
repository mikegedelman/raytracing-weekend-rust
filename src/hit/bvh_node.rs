use crate::ray::Ray;
use crate::util::random_usize;
use crate::vec3::Point3;

use super::{HitRecord, Hittable, HittableBehavior};

/// AABB: Axis-Aligned Bounding Box
/// We'll use this concept to bound objects and groups of objects, to more quickly
/// determine whether a ray hits any of them.
#[derive(Clone, Copy)]
pub struct AABB {
    pub minimum: Point3,
    pub maximum: Point3,
}

impl AABB {
    /// Surround box0 and box1 with an AABB
    pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
        let small = Point3::new(
            f32::min(box0.minimum.x(), box1.minimum.x()),
            f32::min(box0.minimum.y(), box1.minimum.y()),
            f32::min(box0.minimum.z(), box1.minimum.z()),
        );

        let big = Point3::new(
            f32::max(box0.maximum.x(), box1.maximum.x()),
            f32::max(box0.maximum.y(), box1.maximum.y()),
            f32::max(box0.maximum.z(), box1.maximum.z()),
        );

        return AABB {
            minimum: small,
            maximum: big,
        };
    }

    /// Test whether the AABB is hit
    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / ray.dir[a];
            let mut t0 = (self.minimum[a] - ray.orig[a]) * inv_d;
            let mut t1 = (self.maximum[a] - ray.orig[a]) * inv_d;
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            let t_min_p = if t0 > t_min { t0 } else { t_min }; // p -> prime
            let t_max_p = if t1 < t_max { t1 } else { t_max };

            if t_max_p <= t_min_p {
                return false;
            }
        }

        true
    }
}

/// A BVHNode will be a node in a Bounding Volume Hierarchy
/// We represent this as a tree
#[derive(Clone)]
pub struct BVHNode {
    left: Box<Hittable>,
    right: Box<Hittable>,
    aabb_box: AABB,
}

impl HittableBehavior for BVHNode {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if !self.aabb_box.hit(r, t_min, t_max) {
            return None;
        }

        let hit_left = self.left.hit(r, t_min, t_max);
        let hit_right_tmax = match hit_left {
            Some(left_hit_rec) => left_hit_rec.t,
            None => t_max,
        };
        let hit_right = self.right.hit(r, t_min, hit_right_tmax);

        if hit_right.is_some() {
            return hit_right;
        } else if hit_left.is_some() {
            return hit_left;
        }

        None
    }

    fn bounding_box(&self, _: f32, _: f32) -> Option<AABB> {
        Some(self.aabb_box)
    }
}

impl BVHNode {
    /// Generate a BVH for a given list of Hittables
    pub fn new(src_objects: &Vec<Hittable>, time0: f32, time1: f32) -> BVHNode {
        Self::new_helper(src_objects, 0, src_objects.len(), time0, time1)
    }

    /// A helper method since we use recursion to generate the BVH
    fn new_helper(
        src_objects: &Vec<Hittable>,
        start: usize,
        end: usize,
        time0: f32,
        time1: f32,
    ) -> BVHNode {
        let axis = random_usize(0, 3);
        let comparator = |a: &Hittable, b: &Hittable| Self::box_compare(a, b, axis);

        let num_objects = end - start;
        let (left, right) = match num_objects {
            0 => {
                panic!("BVHNode::new() got a list of 0 objects.");
            }
            1 => (src_objects[start].clone(), src_objects[start].clone()),
            2 => {
                if comparator(&src_objects[start], &src_objects[start + 1])
                    == std::cmp::Ordering::Greater
                {
                    (src_objects[start].clone(), src_objects[start + 1].clone())
                } else {
                    (src_objects[start + 1].clone(), src_objects[start].clone())
                }
            }
            _ => {
                let mut objects = Vec::from_iter(src_objects[start..end].iter().cloned());
                objects.sort_by(comparator); // consider unstable sort for speed

                let mid = start + (num_objects / 2);
                (
                    BVHNode::new_helper(&src_objects, start, mid, time0, time1).into(),
                    BVHNode::new_helper(&src_objects, mid, end, time0, time1).into(),
                )
            }
        };

        let box_left = left.bounding_box(time0, time1).unwrap();
        let box_right = right.bounding_box(time0, time1).unwrap();
        let aabb_box = AABB::surrounding_box(&box_left, &box_right);

        BVHNode {
            left: Box::new(left),
            right: Box::new(right),
            aabb_box,
        }
    }

    /// Define a compare method, to be used for sorting lists of hittables along a given
    /// axis.
    fn box_compare(a: &Hittable, b: &Hittable, axis: usize) -> std::cmp::Ordering {
        let box_a = a.bounding_box(0.0, 0.0).unwrap();
        let box_b = b.bounding_box(0.0, 0.0).unwrap();

        let cmp = box_a.minimum[axis] - box_b.minimum[axis];
        if cmp < 0.0 {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    }
}
