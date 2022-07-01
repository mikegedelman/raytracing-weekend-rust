#![feature(portable_simd)]

mod camera;
mod hit;
mod material;
mod ray;
mod scenes;
mod util;
mod vec3;

use std::fs::File;
use std::io::{self, BufWriter, Seek, Write};
use std::time::Instant;

use console::style;
use indicatif::{HumanBytes, ParallelProgressIterator, ProgressBar};
use rayon::iter::ParallelIterator;
use rayon::prelude::*;

use camera::Camera;
use hit::Hittable;
use ray::ray_color;
use util::{clamp, random_f32};
use vec3::{Color, Point3};

use self::scenes::raytracing_weekend_scene;

fn post_process(color: Color, samples_per_pixel: i32) -> Vec<u8> {
    // sqrt: gamma correction is raise to the power of 1/gamma, and we're using gamma=2, so pow(1/2) -> sqrt
    let scale = 1.0 / samples_per_pixel as f32;
    let r = f32::sqrt(color.x() * scale);
    let b = f32::sqrt(color.y() * scale);
    let g = f32::sqrt(color.z() * scale);

    vec![
        (256.0 * clamp(r, 0.0, 0.999)) as u8,
        (256.0 * clamp(b, 0.0, 0.999)) as u8,
        (256.0 * clamp(g, 0.0, 0.999)) as u8,
    ]
}

fn write_color(
    w: &mut BufWriter<&mut File>,
    bytes: Vec<u8>,
    // samples_per_pixel: i32,
) -> io::Result<()> {
    write!(
        w,
        "{} {} {}\n",
        bytes[0], bytes[1], bytes[2]
    )
}

fn make_camera(aspect_ratio: f32) -> Camera {
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Point3::new(0.0, 1.0, 0.0);
    let fov = 20.0;
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    Camera::new(
        lookfrom,
        lookat,
        vup,
        fov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    )
}

fn render(
    objects: &Vec<Hittable>,
    camera: &Camera,
    image_width: i32,
    image_height: i32,
    samples_per_pixel: i32,
    max_depth: i32,
    pb: ProgressBar,
) ->Vec<u8> {
    let range: Vec<i32> = (0..image_height).rev().collect();
    let intermediate: Vec<Vec<Vec<u8>>> = range
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
                        a + ray_color(&r, &objects, max_depth) // Determine the color of the ray reflected back at the camera
                    })
                })
                .map(|color| post_process(color, samples_per_pixel))
                .collect()
        })
        .collect();

    let flatten1: Vec<Vec<u8>> = intermediate.into_iter().flatten().collect();
    let flatten2: Vec<u8> = flatten1.into_iter().flatten().collect();
    flatten2
}

fn main() -> io::Result<()> {
    println!("{} Setup...", style("[1/3]").bold().dim());
    // Image parameters
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 500;
    let image_height = (image_width as f32 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let camera = make_camera(aspect_ratio);
    let world = raytracing_weekend_scene();

    // Render
    println!("{} Render...", style("[2/3]").bold().dim());
    let pb = ProgressBar::new(image_height as u64);
    let before_render = Instant::now();
    let pixels = render(
        &world,
        &camera,
        image_width,
        image_height,
        samples_per_pixel,
        max_depth,
        pb,
    );
    let render_elapsed = before_render.elapsed();

    println!("{} Write to disk...", style("[3/3]").bold().dim());
    let mut f = File::create("./image.ppm").unwrap();
    let mut writer = BufWriter::new(&mut f);

    write!(&mut writer, "P3\n{} {}\n255\n", image_width, image_height)?;
    let mut counter = 0;
    for byte in pixels {
        // write_color(&mut writer, bytes_vec)?;
        write!(&mut writer, "{}", byte)?;
        counter += 1;
        if counter == 3 {
            write!(writer, "\n")?;
            counter = 0;
        } else {
            write!(writer, " ")?;
        }
    }
    writer.flush()?;

    println!("Complete!");
    println!("Render time: {:?}", style(render_elapsed).bold());
    println!(
        "Image size: {}",
        style(HumanBytes(writer.stream_position().unwrap())).bold()
    );

    Ok(())
}
