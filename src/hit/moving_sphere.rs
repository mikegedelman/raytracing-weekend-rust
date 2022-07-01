use super::{HitRecord, HittableBehavior, AABB};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

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
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl HittableBehavior for MovingSphere {
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
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

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
            minimum: self.center(time0) - Vec3::new(self.radius, self.radius, self.radius),
            maximum: self.center(time0) + Vec3::new(self.radius, self.radius, self.radius),
        };

        let box1 = AABB {
            minimum: self.center(time1) - Vec3::new(self.radius, self.radius, self.radius),
            maximum: self.center(time1) + Vec3::new(self.radius, self.radius, self.radius),
        };

        Some(AABB::surrounding_box(&box0, &box1))
    }
}