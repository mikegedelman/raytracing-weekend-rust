use enum_dispatch::enum_dispatch;

use crate::hit::*;
use crate::ray::*;
use crate::vec3::*;


#[enum_dispatch]
pub trait MaterialBehavior: Sized {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> (Option<Ray>, Color);
}


#[derive(Clone, Copy, Debug)]
pub struct Lambertian {
    pub albedo: Color,
}

impl MaterialBehavior for Lambertian {
    fn scatter(&self, _: &Ray, rec: &HitRecord) -> (Option<Ray>, Color) {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction);
        (Option::Some(scattered), self.albedo)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Metal {
    pub albedo: Color,
}

impl MaterialBehavior for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> (Option<Ray>, Color) {
        let reflected = Vec3::reflect(&Vec3::unit_vector(&ray.dir), &rec.normal);
        let scattered = Ray::new(rec.p, reflected);

        if Vec3::dot(&scattered.dir, &rec.normal) > 0.0 {
            (Option::Some(scattered), self.albedo)
        } else {
            (Option::None, self.albedo)
        }
    }
}



// #[derive(Debug, PartialEq)]
#[derive(Clone, Copy)]
#[enum_dispatch(MaterialBehavior)]
pub enum Material {
    Lambertian,
    Metal,
}
