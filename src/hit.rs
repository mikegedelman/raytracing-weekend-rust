use crate::ray::*;
use crate::vec3::*;

#[derive(Debug, PartialEq)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

#[derive(Debug, PartialEq)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
}

impl Hittable for Sphere {
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
        let (normal, front_face) = if Vec3::dot(&r.dir, &outward_normal) > 0.0 {
            (-outward_normal, false) // ???
        } else {
            (outward_normal, true)
        };

        return Option::Some(HitRecord {
            t: root,
            p,
            normal,
            front_face,
        });
    }
}
