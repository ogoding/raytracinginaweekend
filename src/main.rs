//#![feature(nll)]

extern crate rand;
extern crate time;
extern crate xorshift;
//#[macro_use] extern crate lazy_static;
extern crate rayon;

mod image;
mod vec3;
mod ray;
mod hitable;
mod sphere;
mod camera;
mod material;
mod random;
//mod texture;
//mod perlin;

use vec3::Vec3;
//use image::{PixelPusher, Image, RGB};
use image::{Image, RGB};
use ray::{Ray, RAY_COUNT};
use hitable::{Hitable, HitableList};
use sphere::{Sphere, MovingSphere};
use camera::Camera;
use material::{Material, Lambertian, Metal, Dieletric};
use random::drand48;

use time::PreciseTime;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use rayon::prelude::*;

// TODO: Create structs for Window/Scene descriptors

fn calculate_colour<T: Hitable>(ray: &Ray, world: &T, depth: u8) -> Vec3 {
    if let Some(hit_record) = world.hit(ray, 0.001, std::f32::MAX) {
        // TODO: Try removing the &mut arguments
        let mut scattered = Ray::zero();
        let mut attenuation = Vec3::zero();

        if depth < 50 && hit_record.material.scatter(ray, &hit_record, &mut attenuation, &mut scattered) {
            attenuation * calculate_colour(&scattered, world, depth + 1)
        } else {
            Vec3::zero()
        }
    } else {
        let unit_direction = ray.direction().unit();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Vec3::uniform(1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn gamma(vec: Vec3) -> Vec3 {
    Vec3::new(vec.x().sqrt(), vec.y().sqrt(), vec.z().sqrt())
}

#[allow(dead_code)]
//fn make_scene() -> HitableList<Sphere> {
fn make_scene() -> HitableList {
    HitableList::new(vec![
        Sphere::new_boxed(Vec3::new(0.0, 0.0, -1.0), 0.5, Box::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5)))),
        Sphere::new_boxed(Vec3::new(0.0, -100.5, -1.0), 100.0, Box::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)))),
        Sphere::new_boxed(Vec3::new(1.0, 0.0, -1.0), 0.5, Box::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.3))),
        Sphere::new_boxed(Vec3::new(-1.0, 0.0, -1.0), 0.5, Box::new(Dieletric::new(1.5))),
        Sphere::new_boxed(Vec3::new(-1.0, 0.0, -1.0), -0.45, Box::new(Dieletric::new(1.5)))
    ])
}

//#[allow(dead_code)]
//fn make_perlin_scene() -> HitableList<Sphere> {
//
//}

#[allow(dead_code)]
//fn make_random_scene() -> HitableList<Sphere> {
fn make_random_scene() -> HitableList {
    let mut spheres: Vec<Box<Hitable>> = vec![Sphere::new_boxed(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Box::new(Lambertian::new(Vec3::uniform(0.5))))];

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = drand48();
            let center = Vec3::new(a as f32 + 0.9 * drand48(), 0.2, b as f32 + 0.9 * drand48());
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    spheres.push(Sphere::new_boxed(center, 0.2,
                                             Box::new(Lambertian::new(Vec3::new(drand48() * drand48(),
                                                           drand48() * drand48(),
                                                           drand48() * drand48())))));
                } else if choose_mat < 0.95 {
                    spheres.push(Sphere::new_boxed(center, 0.2,
                                             Box::new(Metal::new(Vec3::new(0.5 * (1.0 + drand48()),
                                                           0.5 * (1.0 + drand48()),
                                                           0.5 * (1.0 + drand48())),
                                                         0.5 * drand48()))));
                } else {
                    spheres.push(Sphere::new_boxed(center, 0.2, Box::new(Dieletric::new(1.5))));
                }
            }
        }
    }

    spheres.push(Sphere::new_boxed(Vec3::new(0.0, 1.0, 0.0), 1.0, Box::new(Dieletric::new(1.5))));
    spheres.push(Sphere::new_boxed(Vec3::new(-4.0, 1.0, 0.0), 1.0, Box::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)))));
    spheres.push(Sphere::new_boxed(Vec3::new(4.0, 1.0, 0.0), 1.0, Box::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0))));

    HitableList::new(spheres)
}

#[allow(dead_code)]
fn make_random_moving_scene() -> HitableList {
    let mut spheres: Vec<Box<Hitable>> = vec![Sphere::new_boxed(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Box::new(Lambertian::new(Vec3::uniform(0.5))))];

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = drand48();
            let center = Vec3::new(a as f32 + 0.9 * drand48(), 0.2, b as f32 + 0.9 * drand48());
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    spheres.push(MovingSphere::new_boxed(center, center + Vec3::new(0.0, 0.5 * drand48(), 0.0),
                                             0.0, 1.0, 0.2,
                                             Box::new(Lambertian::new(Vec3::new(drand48() * drand48(),
                                                           drand48() * drand48(),
                                                           drand48() * drand48())))));
                } else if choose_mat < 0.95 {
                    spheres.push(Sphere::new_boxed(center, 0.2,
                                             Box::new(Metal::new(Vec3::new(0.5 * (1.0 + drand48()),
                                                           0.5 * (1.0 + drand48()),
                                                           0.5 * (1.0 + drand48())),
                                                         0.5 * drand48()))));
                } else {
                    spheres.push(Sphere::new_boxed(center, 0.2, Box::new(Dieletric::new(1.5))));
                }
            }
        }
    }

    spheres.push(Sphere::new_boxed(Vec3::new(0.0, 1.0, 0.0), 1.0, Box::new(Dieletric::new(1.5))));
    spheres.push(Sphere::new_boxed(Vec3::new(-4.0, 1.0, 0.0), 1.0, Box::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)))));
    spheres.push(Sphere::new_boxed(Vec3::new(4.0, 1.0, 0.0), 1.0, Box::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0))));

    HitableList::new(spheres)
}

fn make_camera(nx: u32, ny: u32) -> Camera {
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    Camera::new(lookfrom, lookat, Vec3::new(0.0, 1.0, 0.0), 20.0, nx as f32 / ny as f32, aperture, dist_to_focus, 0.0, 1.0)
}

fn calculate_pixel<T: Hitable>(index: usize, width: usize, height: usize, samples: usize, world: &T, cam: &Camera) -> RGB {
    let col = index % width;
    let row = height - 1 - (index / width);

    let mut colour = Vec3::zero();
    for _ in 0..samples {
        let u = (col as f32 + drand48()) / width as f32;
        let v = (row as f32 + drand48()) / height as f32;
        let r = cam.get_ray(u, v);

        // TODO: Have a list of typed buffers?
        colour += calculate_colour(&r, world, 0);
    }

    colour = gamma(colour / samples as f32);

    RGB::new_scaled(colour.r(), colour.g(), colour.b())
}

fn run() {
    let nx: usize = 600;
    let ny: usize = 400;
    let ns: usize = 10;
//    let world = make_scene();
    let world = make_random_scene();
    let cam = make_camera(nx as u32, ny as u32);

    let start = PreciseTime::now();

    // TODO: Implement some type for holding primatives/hitable objects

    let pixels: Vec<RGB> = (0..nx*ny)
        .into_par_iter()
        .map(|idx| calculate_pixel(idx, nx, ny, ns, &world, &cam))
        .collect();
    let image = Image::from_vec(pixels, nx as u32, ny as u32);

    let duration: f32 = start.to(PreciseTime::now()).num_milliseconds() as f32 / 1000.0;
    println!("{} seconds for run.", duration);
    println!("Created {:?} rays ({} rays / second).", RAY_COUNT, RAY_COUNT.load(Ordering::Relaxed) as f32 / duration);

    std::fs::write("images/current_progress.ppm", image.to_ppm());
}

fn main() {
    run();
}
