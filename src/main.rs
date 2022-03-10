mod camera;
mod hit;
mod ray;
mod util;
mod vec3;

use std::time::Instant;

use rayon::prelude::*;


use camera::*;
use hit::*;
use rand::seq::index::sample;
use ray::*;
use std::io::{self, Write};
use util::*;
use vec3::*;

fn ray_color<H: Hittable>(r: &Ray, hittables: &Vec<H>, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    for hittable in hittables {
        match hittable.hit(r, 0.001, INFINITY) {
            Some(rec) => {
                let target = rec.p + rec.normal + Vec3::random_unit_vector();
                let new_ray = Ray::new(rec.p, target - rec.p);
                return 0.5 * ray_color(&new_ray, hittables, depth - 1);
            }
            _ => {}
        };
    }

    let unit_direction = Vec3::unit_vector(&r.dir);
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + (t * Color::new(0.5, 0.7, 1.0))
}

fn write_color(color: Color, samples_per_pixel: i32) -> io::Result<()> {
    // sqrt: gamma correction is raise to the power of 1/gamma, and we're using gamma=2
    let scale = 1.0 / samples_per_pixel as f32;
    let r = f32::sqrt(color.x * scale);
    let b = f32::sqrt(color.y * scale);
    let g = f32::sqrt(color.z * scale);

    write!(
        io::stdout(),
        "{} {} {}\n",
        (256.0 * clamp(r, 0.0, 0.999)) as u32,
        (256.0 * clamp(b, 0.0, 0.999)) as u32,
        (256.0 * clamp(g, 0.0, 0.999)) as u32
    )
}

fn main() -> io::Result<()> {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1200;
    let image_height = (image_width as f32 / aspect_ratio) as i32;
    let samples_per_pixel = 500;
    let max_depth = 50;

    // Camera
    let camera = Camera::new();

    // Render

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

    let before = Instant::now();

    let mut rows = vec![];
    for j in (0..image_height).rev() {
        // write!(io::stderr(), "\r Scanlines remaining: {} \n", j);
        let row_colors: Vec<Vec3> = (0..image_width).into_par_iter().map(|i| {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            // let mut pixel_color: Vec3 = (0..samples_per_pixel)
            //     .into_iter()
            //     .map(|_| {
                    for _ in 0..samples_per_pixel {
                        let u = (i as f32 + random_f32()) / (image_width as f32 - 1.0);
                        let v = (j as f32 + random_f32()) / (image_height as f32 - 1.0);

                        let r = camera.get_ray(u, v);
                        pixel_color += ray_color(&r, &hittables, max_depth);
                    }
                    // ray_color(&r, &hittables, max_depth)
                // })

                // .sum();

            // let color = Color::new(r, g, b);
            // write_color(pixel_color, samples_per_pixel);
            pixel_color
        }).collect();
        rows.push(row_colors);
        eprintln!("Scanline {} completed", j);
    }

    let elapsed = before.elapsed();
    eprintln!("Render time: {:.2?}", elapsed);

    write!(io::stdout(), "P3\n{} {}\n255\n", image_width, image_height);
    for row in rows {
        for color in row {
            write_color(color, samples_per_pixel);
        }
    }

    Ok(())
}
