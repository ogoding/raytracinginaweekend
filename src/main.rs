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
mod cube;
mod transform;
mod volume;
mod scene;
mod bvh;

use scenes::*;
use vec3::Vec3;
use image::{Image, RGB};
use ray::{Ray, RAY_COUNT};
use hitable::{Hitable, HitableList};
use camera::Camera;
use random::drand48;
use material::MaterialList;
use bvh::Bvh;

use time::PreciseTime;
use std::sync::atomic::Ordering;

use rayon::prelude::*;

fn calculate_colour(ray: &Ray, world: &HitableList, bvh: &Bvh, materials: &MaterialList, depth: u8) -> Vec3 {
    if let Some(hit_record) = bvh.hit(world, ray, 0.001, std::f32::MAX) {
        // TODO: Try removing the &mut arguments
        let mut scattered = Ray::zero();
        let mut attenuation = Vec3::zero();

        // TODO: Instead of using Index trait, use a method like scene.get_material(usize) so that the same pattern can be applied to the geometry/primatives/hitables themselves
        // TODO: After changing HitRecord and other Primatives to use a raw MaterialIndex, Retrieve texture+material here (and default to an error material + texture)
        let material = &materials[hit_record.material];
        let emitted = material.emitted(hit_record.u, hit_record.v, &hit_record.p);

        if depth < 50 && material.scatter(ray, &hit_record, &mut attenuation, &mut scattered) {
            emitted + attenuation * calculate_colour(&scattered, world, bvh, materials, depth + 1)
        } else {
            emitted
//            Vec3::zero()
        }
    } else {
        // This causes earlier scenes to not be visible due to not enough light
//        let unit_direction = ray.direction().unit();
//        let t = 0.5 * (unit_direction.y() + 1.0);
//        (1.0 - t) * Vec3::uniform(1.0) + t * Vec3::new(0.5, 0.7, 1.0)
        Vec3::zero()
    }
}

fn gamma(vec: Vec3) -> Vec3 {
    Vec3::new(vec.x().sqrt(), vec.y().sqrt(), vec.z().sqrt())
}

// TODO: Change this to only take in pixel index + scene + window structs
fn calculate_pixel(index: usize, width: usize, height: usize, samples: usize, world: &HitableList, bvh: &Bvh, materials: &MaterialList, cam: &Camera) -> RGB {
    let col = index % width;
    let row = height - 1 - (index / width);

    let mut colour = Vec3::zero();
    for _ in 0..samples {
        let u = (col as f32 + drand48()) / width as f32;
        let v = (row as f32 + drand48()) / height as f32;
        let r = cam.get_ray(u, v);

        colour += calculate_colour(&r, world, bvh, &materials,0);
    }

    colour = gamma(colour / samples as f32);

    RGB::new_scaled(colour.r(), colour.g(), colour.b())
}

fn run() {
    let nx: usize = 600;
    let ny: usize = 400;
    let ns: usize = 100;
//    let (mut scene, window) = make_scene(nx as u32, ny as u32, ns as u32);
//    let (mut scene, window) = make_random_scene(nx as u32, ny as u32, ns as u32);
//    let (mut scene, window) = make_random_moving_scene(nx as u32, ny as u32, ns as u32);
//    let (mut scene, window) = make_two_spheres_scene(nx as u32, ny as u32, ns as u32);
    // TODO: Test the earth_scene/image_texture_scene
//    let (mut scene, window) = make_earth_scene(nx as u32, ny as u32, ns as u32);
//    let (mut scene, window) = make_two_perlin_spheres_scene(nx as u32, ny as u32, ns as u32);
    // FIXME: There are some weird light artifacts around edges of lights and at top of sphere
//    let (mut scene, window) = make_simple_light_scene(nx as u32, ny as u32, ns as u32);
    let (mut scene, window) = make_cornell_box(nx as u32, ny as u32, ns as u32);
    // FIXME: Update the light to match the book values (seems to bug out when using their z1 value
//    let (mut scene, window) = make_cornell_smoke(nx as u32, ny as u32, ns as u32);
    let bvh = Bvh::new(&mut scene.world, 0.0, 1.0);

    let start = PreciseTime::now();

    // TODO: Support args for output file name + which scene to run

    let pixels: Vec<RGB> = (0..nx*ny)
        .into_par_iter()
        .map(|idx| calculate_pixel(idx, window.width as usize, window.height as usize, window.samples as usize, &scene.world, &bvh, &scene.materials, &window.camera))
        .collect();

    let image = Image::from_vec(pixels, window.width, window.height);

    let duration: f32 = start.to(PreciseTime::now()).num_milliseconds() as f32 / 1000.0;
    println!("{} seconds for run.", duration);
    println!("Created {:?} rays ({} rays / second).", RAY_COUNT, RAY_COUNT.load(Ordering::Relaxed) as f32 / duration);

    std::fs::write("images/current_progress.ppm", image.to_ppm());
}

fn main() {
    run();
}

// TODO: Remaining Chapters of Book 2
// TODO: 5: Image Texture - Confirm that this actually works
// TODO: 8: Volumes - Fix the weird lighting artfacts that are going on (happening in make_simple_light_scene too)
// TODO: 9: A Scene Testing - All new features