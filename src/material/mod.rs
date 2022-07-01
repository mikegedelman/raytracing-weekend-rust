use enum_dispatch::enum_dispatch;

use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::vec3::Color;

mod dialectric;
mod lambertian;
mod metal;

pub use dialectric::Dialectric;
pub use lambertian::Lambertian;
pub use metal::Metal;

#[enum_dispatch]
pub trait MaterialBehavior: Sized {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> (Option<Ray>, Color);
}

#[derive(Clone, Copy)]
#[enum_dispatch(MaterialBehavior)]
pub enum Material {
    Lambertian,
    Metal,
    Dialectric,
}
