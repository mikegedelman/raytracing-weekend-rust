use crate::vec3::*;
use crate::ray::*;
use crate::hit::*;

pub trait Material: Sized {
    fn scatter(&self, ray: &Ray, rec: &HitRecord<Self>) -> (Option<Ray>, Color);
}

#[derive(Clone, Copy, Debug)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, rec: &HitRecord<Self>) -> (Option<Ray>, Color) {
        let scatter_direction = rec.normal + Vec3::random_unit_vector();
        let scattered = Ray::new(rec.p, scatter_direction);
        (Option::Some(scattered), self.albedo)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Metal {
    pub albedo: Color,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord<Self>) -> (Option<Ray>, Color) {
        let reflected = Vec3::reflect(&Vec3::unit_vector(&ray.dir), &rec.normal);
        let scattered = Ray::new(rec.p, reflected);

        if Vec3::dot(&scattered.dir, &rec.normal) > 0.0 {
            (Option::Some(scattered), self.albedo)
        } else {
            (Option::None, self.albedo)
        }
    }
}