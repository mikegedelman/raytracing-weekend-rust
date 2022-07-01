use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};

use super::MaterialBehavior;

#[derive(Clone, Copy, Debug)]
pub struct Lambertian {
    pub albedo: Color,
}

impl MaterialBehavior for Lambertian {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> (Option<Ray>, Color) {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction, ray.time);
        (Option::Some(scattered), self.albedo)
    }
}
