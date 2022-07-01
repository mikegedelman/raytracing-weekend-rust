use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::util::random_f32;
use crate::vec3::{Color, Vec3};

use super::MaterialBehavior;

#[derive(Clone, Copy, Debug)]
pub struct Dialectric {
    pub index_of_refraction: f32,
}

fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
    // Use Schlick's approximation for reflectance.
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    return r0 + (1.0 - r0) * f32::powf(1.0 - cosine, 5.0);
}

impl MaterialBehavior for Dialectric {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> (Option<Ray>, Color) {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };

        let unit_direction = Vec3::unit_vector(&ray.dir);

        let cos_theta = f32::min(Vec3::dot(&(-unit_direction), &rec.normal), 1.0);
        let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract = (refraction_ratio * sin_theta) > 1.0;
        let direction = if cannot_refract || reflectance(cos_theta, refraction_ratio) > random_f32()
        {
            Vec3::reflect(&unit_direction, &rec.normal)
        } else {
            Vec3::refract(&unit_direction, &rec.normal, refraction_ratio)
        };

        let refracted = Vec3::refract(&direction, &rec.normal, refraction_ratio);
        let scattered = Ray::new(rec.p, refracted, ray.time);

        (Option::Some(scattered), attenuation)
    }
}
