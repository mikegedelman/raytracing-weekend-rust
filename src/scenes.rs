use crate::hit::{BVHNode, Hittable, MovingSphere, Sphere};
use crate::material::{Dialectric, Lambertian, Metal};
use crate::util::{random_f32, random_f32_range};
use crate::vec3::{Color, Point3, Vec3};

pub fn raytracing_weekend_scene() -> Vec<Hittable> {
    let mut world: Vec<Hittable> = vec![];
    world.push(
        Sphere {
            center: Point3::new(0.0, -1000.0, 0.0),
            radius: 1000.0,
            material: Lambertian {
                albedo: Color::new(0.5, 0.5, 0.5),
            }
            .into(),
        }
        .into(),
    );

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f32();

            let center = Point3::new(
                a as f32 + 0.9 * random_f32(),
                0.2,
                b as f32 + 0.9 + random_f32(),
            );
            let radius = 0.2;

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let material = Lambertian { albedo };
                    let center1 = center + Vec3::new(0.0, random_f32_range(0.0, 0.5), 0.0);
                    world.push(
                        MovingSphere {
                            center0: center,
                            center1,
                            time0: 0.0,
                            time1: 1.0,
                            radius,
                            material: material.into(),
                        }
                        .into(),
                    );
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_f32_range(0.0, 0.5);
                    let material = Metal { albedo, fuzz };
                    world.push(
                        Sphere {
                            center,
                            radius,
                            material: material.into(),
                        }
                        .into(),
                    );
                } else {
                    // glass
                    let material = Dialectric {
                        index_of_refraction: 1.5,
                    };
                    world.push(
                        Sphere {
                            center,
                            radius,
                            material: material.into(),
                        }
                        .into(),
                    );
                }
            }
        }
    }

    world.push(
        Sphere {
            center: Point3::new(0.0, 1.0, 0.0),
            radius: 1.0,
            material: Dialectric {
                index_of_refraction: 1.5,
            }
            .into(),
        }
        .into(),
    );
    world.push(
        Sphere {
            center: Point3::new(-4.0, 1.0, 0.0),
            radius: 1.0,
            material: Lambertian {
                albedo: Color::new(0.4, 0.2, 0.1),
            }
            .into(),
        }
        .into(),
    );
    world.push(
        Sphere {
            center: Point3::new(4.0, 1.0, 0.0),
            radius: 1.0,
            material: Metal {
                albedo: Color::new(0.7, 0.6, 0.5),
                fuzz: 0.0,
            }
            .into(),
        }
        .into(),
    );

    vec![BVHNode::new(&world, 0.0, 1.0).into()]
}
