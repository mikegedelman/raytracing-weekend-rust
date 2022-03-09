mod util;
mod ray;
mod vec3;
mod hit;
mod camera;


use util::*;
use ray::*;
use std::io::{self, Write};
use vec3::*;
use hit::*;
use camera::*;


fn ray_color<H: Hittable>(r: &Ray, hittables: &Vec<H>) -> Color {
    for hittable in hittables {
        match hittable.hit(r, 0.0, INFINITY) {
            Some(rec) => {
                return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
            }
            _ => {}
        };
    }

    let unit_direction = Vec3::unit_vector(&r.dir);
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + (t * Color::new(0.5, 0.7, 1.0))
}

fn write_color(color: Color) -> io::Result<()> {
    write!(
        io::stdout(),
        "{} {} {}\n",
        (255.999 * color.x) as u32,
        (255.999 * color.y) as u32,
        (255.999 * color.z) as u32
    )
}

fn main() -> io::Result<()> {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as i32;

    // Camera
    let camera = Camera::new();

    // Render
    write!(io::stdout(), "P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        write!(io::stderr(), "\r Scanlines remaining: {} \n", j);
        for i in 0..image_width {
            let u = i as f32 / (image_width as f32 - 1.0);
            let v = j as f32 / (image_height as f32 - 1.0);

            let hittables = vec![
                Sphere {
                    center: Point3::new(0.0, 0.0, -1.0),
                    radius: 0.5,
                },
                Sphere {
                    center: Point3::new(0.0, -100.5, -1.0),
                    radius: 100.0,
                },
            ];

            let r = camera.get_ray(u, v);
            let pixel_color = ray_color(&r, &hittables);

            // let color = Color::new(r, g, b);
            write_color(pixel_color);
        }
    }

    Ok(())
}
