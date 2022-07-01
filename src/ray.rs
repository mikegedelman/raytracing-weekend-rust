use crate::hit::{hit_list, Hittable};
use crate::material::MaterialBehavior;
use crate::util::INFINITY;
use crate::vec3::{Color, Point3, Vec3};

pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
    pub time: f32,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3, time: f32) -> Ray {
        Ray { orig, dir, time }
    }

    // Calculate P for P(t) = A + (b * t) where A is the origin of the ray, and b is the direction.
    pub fn at(&self, t: f32) -> Point3 {
        self.orig + t * self.dir
    }
}

pub fn ray_color(r: &Ray, hittables: &Vec<Hittable>, depth: i32) -> Color {
    if depth <= 0 {
        return Color::zero();
    }

    match hit_list(hittables, r, 0.001, INFINITY) {
        Some(rec) => {
            let m = rec.material;
            return match m.scatter(&r, &rec) {
                (Some(scattered_ray), attenuation) => {
                    attenuation * ray_color(&scattered_ray, hittables, depth - 1)
                }
                (None, _) => Color::zero(),
            };
        }
        None => {}
    };

    // Background gradient
    let unit_direction = Vec3::unit_vector(&r.dir);
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + (t * Color::new(0.5, 0.7, 1.0))
}
