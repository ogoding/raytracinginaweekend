//#![feature(nll)]
// Does this need to go somewhere else?
#![cfg_attr(test, feature(test))]

// TODO: Move most of this stuff into a lib.rs?
// TODO: run clippy and rustfmt

extern crate rand;
extern crate time;
extern crate xorshift;
#[macro_use]
extern crate lazy_static;
extern crate image as imagers;
extern crate rayon;


mod aabb;
mod aarect;
mod bvh;
mod camera;
mod cube;
mod hitable;
mod image;
mod material;
mod perlin;
mod random;
mod ray;
mod scene;
mod scenes;
mod sphere;
mod texture;
mod transform;
mod vec3;
mod volume;

use bvh::Bvh;
use camera::Camera;
use hitable::{HitableList, HitRecord};
use image::{Image, RGB, new_rgb};
use material::MaterialList;
use random::drand48;
use ray::{Ray, RAY_COUNT};
use vec3::Vec3;

use std::sync::atomic::Ordering;
use std::u16;
use time::PreciseTime;

use rayon::prelude::*;

use scene::Resources;
use std::collections::HashMap;

const MAX_RAY_DEPTH: u8 = 50;

fn trace_ray(
    ray: &Ray,
    world: &HitableList,
    bvh: &Bvh,
    materials: &MaterialList,
//    resources: &Resources,
    depth: u8,
) -> Vec3 {
    // TODO: Test if this is faster or not
//    let mut hit_record = HitRecord::zero();
//    if bvh.hit(world, ray, 0.001, std::f32::MAX, &mut hit_record) {
    if let Some(hit_record) = bvh.hit(world, ray, 0.001, std::f32::MAX) {
        let mut scattered = Ray::zero();
        let mut attenuation = Vec3::zero();

        // TODO: Instead of using Index trait, use a method like scene.get_material(usize) so that the same pattern can be applied to the geometry/primatives/hitables themselves
        // TODO: After changing HitRecord and other Primitives to use a raw MaterialIndex, Retrieve texture+material here (and default to an error material + texture)
        let material = &materials[hit_record.material as usize];
        let emitted = material.emitted(hit_record.u, hit_record.v, &hit_record.p);
//        let material = resources.get_material(hit_record.material as usize);
//        let emitted = material.emitted(&resources.textures, hit_record.u, hit_record.v, &hit_record.p);

        if depth < MAX_RAY_DEPTH
            && material.scatter(ray, &hit_record, &mut attenuation, &mut scattered)
//            && material.scatter(&resources.textures, ray, &hit_record, &mut attenuation, &mut scattered)
        {
            emitted + attenuation * trace_ray(&scattered, world, bvh, materials, depth + 1)
//            emitted + attenuation * trace_ray(&scattered, world, bvh, resources, depth + 1)
        } else {
            emitted
//                        Vec3::zero()
        }
    } else {
        // This causes earlier scenes to not be visible due to not enough light
//                let unit_direction = ray.direction().unit();
//                let t = 0.5 * (unit_direction.y() + 1.0);
//                (1.0 - t) * Vec3::uniform(1.0) + t * Vec3::new(0.5, 0.7, 1.0)
        Vec3::zero()
    }
}

fn gamma(vec: Vec3) -> Vec3 {
    Vec3::new(vec.x().sqrt(), vec.y().sqrt(), vec.z().sqrt())
}

// TODO: Change this to only take in pixel index + scene + window structs
fn calculate_pixel(
    index: usize,
    width: usize,
    height: usize,
    samples: usize,
    world: &HitableList,
    bvh: &Bvh,
    materials: &MaterialList,
//    resources: &Resources,
    cam: &Camera,
) -> RGB {
    let col = index % width;
    let row = height - 1 - (index / width);

    let mut colour = Vec3::zero();
    for _ in 0..samples {
        let u = (col as f32 + drand48()) / width as f32;
        let v = (row as f32 + drand48()) / height as f32;
        let r = cam.get_ray(u, v);

        colour += trace_ray(&r, world, bvh, &materials, 0);
//        colour += trace_ray(&r, world, bvh, resources, 0);
    }

    colour = gamma(colour / samples as f32);

//    RGB::new_scaled(colour.r(), colour.g(), colour.b())
    new_rgb(colour.r(), colour.g(), colour.b())
}

// TODO: Do a breadth based trace implementation
// for each pixel create a RGB pixel (0,0,0 to start?)
// also create a ray + sample count
//struct Pixel {
//    rgb: Vec3
//}
//
//struct RayBuffer {
//    // TODO: Reformat into SoA style
//    buffer: Vec<Ray>
//    // ray_x: Vec<f32>
//    // ray_y: Vec<f32>
//    // ray_z: Vec<f32>
//}
//
// TODO: make the buffer of x * y * samples long
// TODO: Check batches of rays progressively against bvh - e.g. group them together to avoid redundant bvh checks

fn run() {
    let nx: usize = 600;
    let ny: usize = 400;
    let ns: usize = 100;

    use scene::{Scene, Window};
    use scenes::load_scene;

    let scene = "cornell_box";
    let (mut scene, window) = load_scene(scene, nx as u32, ny as u32, ns as u32)?;
    let bvh = Bvh::new(&mut scene.world, 0.0, 1.0);

    assert!(
//        (scene.resources.materials.len() as u16) < u16::MAX,
        (scene.materials.len() as u16) < u16::MAX,
        format!("The maximum supported number of materials is {}", u16::MAX)
    );

    let start = PreciseTime::now();

    // TODO: Support args for output file name + which scene to run
    // TODO: Implement a version of this that builds buffers of rays to process (maybe store as SoA?)
    // TODO: How to handle multiple types of Hitable object? Turn everything into meshes/triangles? How would spheres be done?

    // Is this the best way to do it? or is parallelism over sub images/tiles better?
    let pixels: Vec<RGB> = (0..nx * ny)
//        .into_par_iter()
                .into_iter()
        .map(|idx| {
            calculate_pixel(
                idx,
                window.width as usize,
                window.height as usize,
                window.samples as usize,
                &scene.world,
                &bvh,
                &scene.materials,
//                &scene.resources,
                &window.camera,
            )
        })
        .collect();

    let image = Image::from_vec(pixels, window.width, window.height);

    // TODO: Work out whether MaterialEnum/TextureEnum are faster than trait solution using the final scene

    let duration: f32 = start.to(PreciseTime::now()).num_milliseconds() as f32 / 1000.0;
    println!("{} seconds for run.", duration);
    println!(
        "Created {:?} rays ({} rays / second).",
        RAY_COUNT,
        RAY_COUNT.load(Ordering::Relaxed) as f32 / duration
    );

    image.save("images/current_progress.jpg");
}

fn main() {
    run();
}

// TODO: Remaining Chapters of Book 2
// TODO: 8: Volumes - Fix the weird lighting artifacts that are going on (happening in make_simple_light_scene too)
// TODO: 9: A Scene Testing - All new features
