mod vec3;
mod ray;

use std::io::{self, Write};
use vec3::*;
use ray::*;

fn write_color(color: Color) -> io::Result<()> {
    write!(io::stdout(), "{} {} {}\n", (255.999 * color.x) as u32, (255.999 * color.y) as u32, (255.999 * color.z) as u32)
}

fn hit_sphere(center: &Point3, radius: f32, r: &Ray) -> bool {
    let oc = r.orig - *center;
    let a = Vec3::dot(&r.dir, &r.dir);
    let b = 2.0 * Vec3::dot(&oc, &r.dir);
    let c = Vec3::dot(&oc, &oc) - (radius * radius);
    let discriminant = (b * b) - (4.0 * a * c);
    return discriminant > 0.0;
}

fn ray_color(r: &Ray) -> Color {
    let sphere_center = Point3::new(0.0, 0.0, -1.0);
    if hit_sphere(&sphere_center, 0.5, r) {
        return Color::new(1.0, 0.0, 0.0);
    }
    let unit_direction = r.dir.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + (t * Color::new(0.5, 0.7, 1.0))
}

fn main() -> io::Result<()> {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render
    write!(io::stdout(), "P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        write!(io::stderr(), "\r Scanlines remaining: {} \n", j);
        for i in 0..image_width {
            // let r = i as f32 / (image_width as f32 - 1.0);
            // let g = j as f32 / (image_height as f32 - 1.0);
            // let b: f32 = 0.25;

            let u = i as f32 / (image_width as f32 - 1.0);
            let v = j as f32 / (image_height as f32 - 1.0);

            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical - origin);
            let pixel_color = ray_color(&r);

            // let color = Color::new(r, g, b);
            write_color(pixel_color);
        }
    }

    Ok(())
}