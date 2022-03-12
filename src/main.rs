mod camera;
mod hit;
mod material;
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
use material::*;
use ray::*;
use util::*;
use vec3::*;

fn hit_list<H: Hittable>(hittables: &Vec<H>, r: &Ray, t_min: f32, t_max: f32,) -> Option<HitRecord> {
    let mut hit_rec = Option::None;
    let mut closest_so_far = t_max;

    for hittable in hittables {
        match hittable.hit(r, t_min, closest_so_far) {
            Some(rec) => {
                closest_so_far = rec.t;
                hit_rec = Option::Some(rec);
            },
            None => {},
        }
    }

    return hit_rec;
}

fn ray_color<H: Hittable>(r: &Ray, hittables: &Vec<H>, depth: i32) -> Color {
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
            }
        },
        None => {},
    };

    // Background gradient
    let unit_direction = Vec3::unit_vector(&r.dir);
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + (t * Color::new(0.5, 0.7, 1.0))
}

fn write_color(
    w: &mut BufWriter<&mut File>,
    color: Color,
    samples_per_pixel: i32,
) -> io::Result<()> {
    // sqrt: gamma correction is raise to the power of 1/gamma, and we're using gamma=2, so pow(1/2) -> sqrt
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
    // Image parameters
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 500;
    let image_height = (image_width as f32 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // Camera
    let camera = Camera::new();

    // Scene
    let material_ground = Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0),
    };
    let material_center = Lambertian {
        albedo: Color::new(0.7, 0.3, 0.3),
    };
    let material_left = Metal {
        albedo: Color::new(0.8, 0.8, 0.8),
    };
    let material_right = Metal {
        albedo: Color::new(0.1, 0.1, 0.1),
    };

    let hittables: Vec<Sphere> = vec![
        Sphere {
            center: Point3::new(1.0, 0.0, -1.0),
            radius: 0.5,
            material: material_right.into(),
        },
        Sphere {
            center: Point3::new(0.0, 0.0, -1.0),
            radius: 0.5,
            material: material_center.into(),
        },
        Sphere {
            center: Point3::new(-1.0, 0.0, -1.0),
            radius: 0.5,
            material: material_left.into(),
        },
        Sphere {
            center: Point3::new(0.0, -100.5, -0.5),
            radius: 100.0,
            material: material_ground.into(),
        },
    ];

    // Render
    println!("{} Render...", style("[2/3]").bold().dim());
    let pb = ProgressBar::new(image_height as u64);
    let before_render = Instant::now();
    let range: Vec<i32> = (0..image_height).rev().collect();
    let rows: Vec<Vec<Vec3>> = range
        .into_par_iter() // Use Rayon to parallelize this iterator for basically no effort
        .progress_with(pb) // Show a progress bar of rows
        .map(|j| {
            // For each row..
            (0..image_width)
                .into_iter()
                .map(|i| {
                    // For each column..
                    // Run $samples_per_pixel rays through the pixel, at random positions within the pixel
                    (0..samples_per_pixel).fold(Color::new(0.0, 0.0, 0.0), |a, _| {
                        let u = (i as f32 + random_f32()) / (image_width as f32 - 1.0);
                        let v = (j as f32 + random_f32()) / (image_height as f32 - 1.0);

                        let r = camera.get_ray(u, v); // Get a vector representing the ray out of the camera.
                        a + ray_color(&r, &hittables, max_depth) // Determine the color of the ray reflected back at the camera
                    })
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
