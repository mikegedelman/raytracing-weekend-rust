use enum_dispatch::enum_dispatch;

use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

mod bvh_node;
mod moving_sphere;
mod sphere;

use bvh_node::AABB;

pub use bvh_node::BVHNode;
pub use moving_sphere::MovingSphere;
pub use sphere::Sphere;

// #[derive(Debug, PartialEq)]
#[derive(Clone, Copy)]
pub struct HitRecord {
    pub p: Point3,    // Point3 where the ray hit the hittable
    pub normal: Vec3, // Normal pointing outwards from the object at p
    pub t: f32,
    pub front_face: bool,
    pub material: Material,
}

#[enum_dispatch]
pub trait HittableBehavior {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB>;
}

#[enum_dispatch(HittableBehavior)]
#[derive(Clone)]
pub enum Hittable {
    Sphere,
    MovingSphere,
    BVHNode,
}

pub fn hit_list(hittables: &Vec<Hittable>, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
    let mut hit_rec = Option::None;
    let mut closest_so_far = t_max;

    for hittable in hittables {
        match hittable.hit(r, t_min, closest_so_far) {
            Some(rec) => {
                closest_so_far = rec.t;
                hit_rec = Option::Some(rec);
            }
            None => {}
        }
    }

    return hit_rec;
}
