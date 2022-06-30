use enum_dispatch::enum_dispatch;

use crate::material::*;
use crate::ray::*;
use crate::util::random_usize;
use crate::vec3::*;
use crate::bvh::*;

// #[derive(Debug, PartialEq)]
#[derive(Clone, Copy)]
pub struct HitRecord {
    pub p: Point3,        // Point3 where the ray hit the hittable
    pub normal: Vec3,     // Normal pointing outwards from the object at p
    pub t: f32,           // ?? not used yet
    pub front_face: bool, // ?? not used yet
    pub material: Material,
}

#[enum_dispatch]
pub trait HittableBehavior {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB>;
}

#[derive(Clone, Copy)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
    pub material: Material,
}

impl HittableBehavior for Sphere{
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        // Calculate the discriminant (the part under the sqrt) of the quadratic equation.
        let oc = r.orig - self.center;
        let a = r.dir.length_squared();
        let half_b = Vec3::dot(&oc, &r.dir);
        let c = oc.length_squared() - (self.radius * self.radius);
        let discriminant = (half_b * half_b) - (a * c);

        // If the discriminant is <0, there is no intersection with the sphere.
        if discriminant < 0.0 {
            return Option::None;
        }

        // Finish solving the quadratic equation in terms of t.
        // Find the nearest root that lies in the acceptable range.
        let sqrtd = f32::sqrt(discriminant);
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return Option::None;
            }
        }

        // Root is a t, solve P = (A * t) + b with it.
        let p = r.at(root);

        // Check whether the ray is moving the same direction as the outward normal.
        let outward_normal = (p - self.center) / self.radius;
        let front_face = Vec3::dot(&r.dir, &outward_normal) < 0.0;
        let normal = if front_face { outward_normal } else { -outward_normal };

        return Option::Some(HitRecord {
            t: root,
            p,
            normal,
            front_face,
            material: self.material.into(),
        });
    }

    fn bounding_box(&self, _: f32, _: f32) -> Option<AABB> {
        Some(AABB {
            minimum: self.center - Vec3::new(self.radius, self.radius, self.radius),
            maximum: self.center + Vec3::new(self.radius, self.radius, self.radius),
        })
    }
}

#[derive(Clone, Copy)]
pub struct MovingSphere {
    pub center0: Point3,
    pub center1: Point3,
    pub time0: f32,
    pub time1: f32,
    pub radius: f32,
    pub material: Material,
}

impl MovingSphere {
    fn center(&self, time: f32) -> Point3 {
        self.center0 + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl HittableBehavior for MovingSphere{
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        // Calculate the discriminant (the part under the sqrt) of the quadratic equation.
        let oc = r.orig - self.center(r.time);
        let a = r.dir.length_squared();
        let half_b = Vec3::dot(&oc, &r.dir);
        let c = oc.length_squared() - (self.radius * self.radius);
        let discriminant = (half_b * half_b) - (a * c);

        // If the discriminant is <0, there is no intersection with the sphere.
        if discriminant < 0.0 {
            return Option::None;
        }

        // Finish solving the quadratic equation in terms of t.
        // Find the nearest root that lies in the acceptable range.
        let sqrtd = f32::sqrt(discriminant);
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return Option::None;
            }
        }

        // Root is a t, solve P = (A * t) + b with it.
        let p = r.at(root);

        // Check whether the ray is moving the same direction as the outward normal.
        let outward_normal = (p - self.center(r.time)) / self.radius;
        let front_face = Vec3::dot(&r.dir, &outward_normal) < 0.0;
        let normal = if front_face { outward_normal } else { -outward_normal };

        return Option::Some(HitRecord {
            t: root,
            p,
            normal,
            front_face,
            material: self.material.into(),
        });
    }

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        let box0 = AABB {
            minimum: self.center(time0  ) - Vec3::new(self.radius, self.radius, self.radius),
            maximum: self.center(time0) + Vec3::new(self.radius, self.radius, self.radius),
        };

        let box1 = AABB {
            minimum: self.center(time1 ) - Vec3::new(self.radius, self.radius, self.radius),
            maximum: self.center(time1) + Vec3::new(self.radius, self.radius, self.radius),
        };

        Some(AABB::surrounding_box(&box0, &box1))
    }
}


#[derive(Clone)]
pub struct BVHNode {
    left: Box<Hittable>,
    right: Box<Hittable>,
    aabb_box: AABB,
}

impl HittableBehavior for BVHNode {
    fn hit(&self, r: &Ray, t_min:f32 ,t_max:f32) -> Option<HitRecord> {
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
    pub fn new(src_objects: &Vec<Hittable>, start: usize, end: usize, time0: f32, time1: f32) -> BVHNode {
        // let mut objects = src_objects.clone();

        let axis = random_usize(0, 3);
        let comparator = |a: &Hittable, b: &Hittable| Self::box_compare(a, b, axis);

        let num_objects = start - end;
        let (left, right) = match num_objects {
            0 => {
                panic!("BVHNode::new() got a list of 0 objects.");
            }
            1 => {
                (src_objects[start].clone(), src_objects[start].clone())
            },
            2 => {
                if comparator(&src_objects[start], &src_objects[start + 1]) == std::cmp::Ordering::Greater {
                    (src_objects[start].clone(), src_objects[start + 1].clone())
                } else {
                    (src_objects[start + 1].clone(), src_objects[start].clone())
                }
            },
            _ => {
                let mut objects = Vec::from_iter(src_objects[start..end].iter().cloned());
                objects.sort_by(comparator);

                let mid = start + (num_objects / 2);
                ( 
                    BVHNode::new(&objects, start, end, time0, time1).into(), 
                    BVHNode::new(&objects, start, end, time0, time1).into()
                )
            }
        };

        let box_left = left.bounding_box(time0, time1).unwrap();
        let box_right = left.bounding_box(time0, time1).unwrap();
        let aabb_box = AABB::surrounding_box(&box_left, &box_right);

        BVHNode {
            left: Box::new(left), right: Box::new(right), aabb_box
        }
    }

    fn box_compare(a: &Hittable, b: &Hittable, axis: usize) -> std::cmp::Ordering {
        let box_a = a.bounding_box(0.0, 0.0).unwrap();
        let box_b = b.bounding_box(0.0, 0.0).unwrap();

        // box_a.minimum[axis].cmp(&box_b.minimum[axis])
        let cmp = box_a.minimum[axis] - box_b.minimum[axis];
        if cmp < 0.0 {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    }
}


#[enum_dispatch(HittableBehavior)]
#[derive(Clone)]
pub enum Hittable {
    Sphere,
    MovingSphere,
    BVHNode,
}
