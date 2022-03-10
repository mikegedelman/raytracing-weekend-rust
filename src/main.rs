mod camera;
mod hit;
mod ray;
mod util;
mod vec3;

use std::fs::File;
use std::io::{self, BufWriter, Seek, Write};
use std::time::Instant;

use console::style;
use indicatif::{HumanBytes, ParallelProgressIterator, ProgressBar};
use rayon::iter::ParallelIterator;
use rayon::prelude::*;

use camera::*;
use hit::*;
use ray::*;
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

fn write_color(
    w: &mut BufWriter<&mut File>,
    color: Color,
    samples_per_pixel: i32,
) -> io::Result<()> {
    // sqrt: gamma correction is raise to the power of 1/gamma, and we're using gamma=2
    let scale = 1.0 / samples_per_pixel as f32;
    let r = f32::sqrt(color.x * scale);
    let b = f32::sqrt(color.y * scale);
    let g = f32::sqrt(color.z * scale);

    write!(
        w,
        "{} {} {}\n",
        (256.0 * clamp(r, 0.0, 0.999)) as u32,
        (256.0 * clamp(b, 0.0, 0.999)) as u32,
        (256.0 * clamp(g, 0.0, 0.999)) as u32
    )
}

fn main() -> io::Result<()> {
    println!("{} Setup...", style("[1/3]").bold().dim());
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

    println!("{} Render...", style("[2/3]").bold().dim());
    let pb = ProgressBar::new(image_height as u64);
    let before_render = Instant::now();
    let range: Vec<i32> = (0..image_height).rev().collect();
    let rows: Vec<Vec<Vec3>> = range
        .into_par_iter()
        .progress_with(pb)
        .map(|j| {
            (0..image_width)
                .into_iter()
                .map(|i| {
                    let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                    for _ in 0..samples_per_pixel {
                        let u = (i as f32 + random_f32()) / (image_width as f32 - 1.0);
                        let v = (j as f32 + random_f32()) / (image_height as f32 - 1.0);

                        let r = camera.get_ray(u, v);
                        pixel_color += ray_color(&r, &hittables, max_depth);
                    }
                    pixel_color
                })
                .collect()
        })
        .collect();

    let render_elapsed = before_render.elapsed();

    println!("{} Write to disk...", style("[3/3]").bold().dim());
    let before_write = Instant::now();
    let mut f = File::create("./image.ppm").unwrap();
    let mut writer = BufWriter::new(&mut f);

    write!(&mut writer, "P3\n{} {}\n255\n", image_width, image_height)?;
    for row in rows {
        for color in row {
            write_color(&mut writer, color, samples_per_pixel)?;
        }
    }
    writer.flush()?;
    let write_elapsed = before_write.elapsed();

    println!("Complete!");
    println!("Render time: {:?}", style(render_elapsed).bold());
    println!(
        "File write time: {} in {:?}",
        style(HumanBytes(writer.stream_position().unwrap())).bold(),
        style(write_elapsed).bold()
    );

    Ok(())
}
