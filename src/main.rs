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

fn hit_list<H: Hittable>(hittables: &Vec<H>, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
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
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + (t * Color::new(0.5, 0.7, 1.0))
}

fn write_color(
    w: &mut BufWriter<&mut File>,
    color: Color,
    samples_per_pixel: i32,
) -> io::Result<()> {
    // sqrt: gamma correction is raise to the power of 1/gamma, and we're using gamma=2, so pow(1/2) -> sqrt
    let scale = 1.0 / samples_per_pixel as f32;
    let r = f32::sqrt(color.x() * scale);
    let b = f32::sqrt(color.y() * scale);
    let g = f32::sqrt(color.z() * scale);

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
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1000;
    let image_height = (image_width as f32 / aspect_ratio) as i32;
    let samples_per_pixel = 200;
    let max_depth = 50;

    // Camera
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    // let lookfrom = Point3::new(0.0, 0.0, 0.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Point3::new(0.0, 1.0, 0.0);
    let fov = 20.0;
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(lookfrom, lookat, vup, fov, aspect_ratio, aperture, dist_to_focus);

    // Scene
    let mut world = vec![];
    world.push(Sphere {
        center: Point3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Lambertian {
            albedo: Color::new(0.5, 0.5, 0.5),
        }.into(),
    });

    // for a in -11..11 {
    //     for b in -11..11 {
    //         let choose_mat = random_f32();

    //         let center = Point3::new(
    //             a as f32 + 0.9 * random_f32(),
    //             0.2,
    //             b as f32 + 0.9 + random_f32(),
    //         );
    //         let radius = 0.2;

    //         if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
    //             if choose_mat < 0.8 {
    //                 // diffuse
    //                 let albedo = Color::random() * Color::random();
    //                 let material = Lambertian {
    //                     albedo,
    //                 };
    //                 world.push(Sphere {
    //                     center,
    //                     radius,
    //                     material: material.into(),
    //                 });
    //             } else if choose_mat < 0.95 {
    //                 // metal
    //                 let albedo = Color::random_range(0.5, 1.0);
    //                 let fuzz = random_f32_range(0.0, 0.5);
    //                 let material = Metal {
    //                     albedo,
    //                     fuzz,
    //                 };
    //                 world.push(Sphere {
    //                     center,
    //                     radius,
    //                     material: material.into(),
    //                 });
    //             } else {
    //                 // glass
    //                 let material = Dialectric {
    //                     index_of_refraction: 1.5,
    //                 };
    //                 world.push(Sphere {
    //                     center,
    //                     radius,
    //                     material: material.into(),
    //                 });
    //             }
    //         }
    //     }
    // }

    world.push(Sphere {
        center: Point3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Dialectric {
            index_of_refraction: 1.5,
        }.into(),
    });
    world.push(Sphere {
        center: Point3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Lambertian {
            albedo: Color::new(0.4, 0.2, 0.1),
        }.into(),
    });
    world.push(Sphere {
        center: Point3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Metal {
            albedo: Color::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        }.into(),
    });

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
                        a + ray_color(&r, &world, max_depth) // Determine the color of the ray reflected back at the camera
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
