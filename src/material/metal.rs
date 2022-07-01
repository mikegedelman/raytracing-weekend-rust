use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};

use super::MaterialBehavior;

#[derive(Clone, Copy, Debug)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

impl MaterialBehavior for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> (Option<Ray>, Color) {
        let reflected = Vec3::reflect(&Vec3::unit_vector(&ray.dir), &rec.normal);
        let scattered = Ray::new(
            rec.p,
            reflected + (self.fuzz * Vec3::random_in_unit_sphere()),
            ray.time,
        );

        if Vec3::dot(&scattered.dir, &rec.normal) > 0.0 {
            (Option::Some(scattered), self.albedo)
        } else {
            (Option::None, self.albedo)
        }
    }
}
