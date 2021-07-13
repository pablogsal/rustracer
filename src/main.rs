use std::{f64::INFINITY, usize};

use rand::Rng;

use rayon::prelude::*;

mod camera;
mod materials;
mod objects;
mod ray;
mod vec3;

use camera::*;
use materials::*;
use objects::*;
use ray::*;
use vec3::*;

fn write_color(color: &Color, samples_per_pixel: i32) {
    let mut col = *color / samples_per_pixel as f64;
    col = Vec3::new(col.x().sqrt(), col.y().sqrt(), col.z().sqrt());

    let ir = (255.99 * col.x()) as u32;
    let ig = (255.99 * col.y()) as u32;
    let ib = (255.99 * col.z()) as u32;
    println!("{} {} {}", ir, ig, ib)
}

fn ray_color(ray: &Ray, objects: &HittableCollection, bounces: i32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if bounces <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(record) = objects.hit(ray, 0.001, INFINITY) {
        if let Some((attenuation, sub_ray)) = record.material.scatter(ray, &record) {
            return attenuation * ray_color(&sub_ray, objects, bounces - 1);
        }
        return Color::new(0.0, 0.0, 0.0);
    }

    let unit_dir: Vec3 = ray.direction().normalize();
    let t = 0.5 * (unit_dir.y() + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
}

fn main() {
    let mut rng = rand::thread_rng();
    // Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width: usize = 1920;
    let image_height: usize = (image_width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 500;
    let max_bounces = 50;

    // World

    let ground_material = Box::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));

    let mut objs = HittableCollection::new();
    objs.add(Box::new(Sphere {
        center: Point3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: ground_material,
    }));

    for a in -11..11 {
        for b in -11..11 {
            let x = (a as f64) + 0.9 * rng.gen_range(0.0..1.0);
            let y = (b as f64) + 0.9 * rng.gen_range(0.0..1.0);
            let choose_mat: f64 = rng.gen();
            let center = Point3::new(x, 0.2, y);

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.6 {
                    // Diffuse
                    let albedo = Color::random() * Color::random();
                    objs.add(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Box::new(Lambertian::new(albedo)),
                    }));
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    objs.add(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Box::new(Metal::new(albedo, fuzz)),
                    }));
                } else {
                    // Glass
                    objs.add(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Box::new(Dielectric::new(1.5)),
                    }));
                }
            }
        }
    }

    objs.add(Box::new(Sphere {
        center: Point3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Box::new(Dielectric::new(1.5)),
    }));

    objs.add(Box::new(Sphere {
        center: Point3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Box::new(Lambertian::new(Color::new(0.4, 0.2, 0.1))),
    }));

    objs.add(Box::new(Sphere {
        center: Point3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Box::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.01)),
    }));

    // Camera

    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);

    let camera = Camera::new(
        look_from,
        look_at,
        Point3::new(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        0.1,
        10.0,
    );

    // Render

    let mut screen = vec![Vec3::new(0.0, 0.0, 0.0); (image_height * image_width) as usize];

    println!("P3\n{} {}\n255", image_width, image_height);

    screen
        .par_iter_mut()
        .enumerate()
        .for_each(|(index, pixel)| {
            let mut rng = rand::thread_rng();
            let i = index % image_width;
            let j = image_height - index / image_width;
            for _ in 0..samples_per_pixel {
                let u = ((i as f64) + rng.gen::<f64>()) / (image_width - 1) as f64;
                let v = ((j as f64) + rng.gen::<f64>()) / (image_height - 1) as f64;
                let ray = camera.get_ray(u, v);
                *pixel += ray_color(&ray, &objs, max_bounces);
            }
        });

    for pixel in screen {
        write_color(&pixel, samples_per_pixel);
    }
}
