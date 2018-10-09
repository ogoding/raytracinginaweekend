//#![feature(nll)]

extern crate rand;
extern crate time;
extern crate xorshift;
#[macro_use] extern crate lazy_static;
extern crate rayon;
extern crate image as imagers;

mod scenes;
mod image;
mod vec3;
mod ray;
mod hitable;
mod sphere;
mod camera;
mod material;
mod random;
mod texture;
mod perlin;
mod aabb;
mod aarect;
//mod cube;
mod transform;
mod volume;
mod scene;

#[allow(unused_imports)]
use scenes::{make_scene, make_random_scene, make_random_moving_scene, make_two_spheres_scene, make_two_perlin_spheres_scene, make_earth_scene, make_simple_light_scene, make_cornell_box, make_cornell_smoke};
//use scenes::{make_scene, make_random_scene, make_random_moving_scene, make_two_spheres_scene, make_two_perlin_spheres_scene, make_earth_scene, make_cornell_box_new};
use vec3::Vec3;
//use image::{PixelPusher, Image, RGB};
use image::{Image, RGB};
use ray::{Ray, RAY_COUNT};
use hitable::Hitable;
use camera::Camera;
use random::drand48;

use time::PreciseTime;
use std::sync::atomic::Ordering;

    use rayon::prelude::*;

    // TODO: Create structs for Window/Scene descriptors

fn calculate_colour<T: Hitable>(ray: &Ray, world: &T, depth: u8) -> Vec3 {
    if let Some(hit_record) = world.hit(ray, 0.001, std::f32::MAX) {
        // TODO: Try removing the &mut arguments
        let mut scattered = Ray::zero();
        let mut attenuation = Vec3::zero();
        let emitted = hit_record.material.emitted(hit_record.u, hit_record.v, &hit_record.p);

        if depth < 50 && hit_record.material.scatter(ray, &hit_record, &mut attenuation, &mut scattered) {
            // TODO: Can this be a fused multiply/add?
            emitted +  attenuation * calculate_colour(&scattered, world, depth + 1)
        } else {
            emitted
//            Vec3::zero()
        }
    } else {
//        let unit_direction = ray.direction().unit();
//        let t = 0.5 * (unit_direction.y() + 1.0);
//        (1.0 - t) * Vec3::uniform(1.0) + t * Vec3::new(0.5, 0.7, 1.0)
        Vec3::zero()
    }
}

fn gamma(vec: Vec3) -> Vec3 {
    Vec3::new(vec.x().sqrt(), vec.y().sqrt(), vec.z().sqrt())
}

fn calculate_pixel<T: Hitable>(index: usize, width: usize, height: usize, samples: usize, world: &T, cam: &Camera) -> RGB {
    let col = index % width;
    let row = height - 1 - (index / width);

    let mut colour = Vec3::zero();
    for _ in 0..samples {
        let u = (col as f32 + drand48()) / width as f32;
        let v = (row as f32 + drand48()) / height as f32;
        let r = cam.get_ray(u, v);

        colour += calculate_colour(&r, world, 0);
    }

    colour = gamma(colour / samples as f32);

    RGB::new_scaled(colour.r(), colour.g(), colour.b())
}

fn run() {
    let nx: usize = 600;
    let ny: usize = 400;
    let ns: usize = 100;
//    let (world, cam) = make_scene(nx as u32, ny as u32);
//    let (world, cam) = make_random_scene(nx as u32, ny as u32);
//    let (world, cam) = make_random_moving_scene(nx as u32, ny as u32);
//    let (world, cam) = make_two_spheres_scene(nx as u32, ny as u32);
//    let (world, cam) = make_earth_scene(nx as u32, ny as u32);
//    let (world, cam) = make_two_perlin_spheres_scene(nx as u32, ny as u32);
//    let (world, cam) = make_simple_light_scene(nx as u32, ny as u32);
    let (world, cam) = make_cornell_box(nx as u32, ny as u32);
//    let (scene, window) = make_cornell_box_new(nx as u32, ny as u32, ns as u32);

    let start = PreciseTime::now();

    // TODO: Implement some type for holding primatives/hitable objects

    let pixels: Vec<RGB> = (0..nx*ny)
        .into_par_iter()
        .map(|idx| calculate_pixel(idx, nx, ny, ns, &world, &cam))
//        .map(|idx| calculate_pixel(idx, window.width as usize, window.height as usize, window.samples as usize, &scene.world, &window.camera))
        .collect();
    let image = Image::from_vec(pixels, nx as u32, ny as u32);
//    let image = Image::from_vec(pixels, window.width, window.height);

    let duration: f32 = start.to(PreciseTime::now()).num_milliseconds() as f32 / 1000.0;
    println!("{} seconds for run.", duration);
    println!("Created {:?} rays ({} rays / second).", RAY_COUNT, RAY_COUNT.load(Ordering::Relaxed) as f32 / duration);

    std::fs::write("images/current_progress.ppm", image.to_ppm());
}

fn main() {
    run();
}

// TODO: Remaining Chapters of Book 2
// TODO: 2: Bounded Volume Hierarchies
// TODO: 7: Instances
// TODO: 8: Volumes - Needs testing and implementing #7
// TODO: 9: A Scene Testing - All new features