// TODO: Move most of this stuff into a lib.rs?
// TODO: run clippy and rustfmt

extern crate rand;
extern crate time;
extern crate xorshift;
#[macro_use]
extern crate lazy_static;
extern crate image as imagers;
extern crate rayon;

extern crate cgmath;


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
mod linear_bvh;

use bvh::{Accelerator, CompactBvh, Bvh};
use hitable::HitRecord;
use image::{Image, RGB, vec_to_rgb};
use random::drand48;
use ray::{Ray, RAY_COUNT};
use vec3::Vec3;

use std::sync::atomic::Ordering;
use std::u16;
use time::PreciseTime;

use rayon::prelude::*;

use scenes::load_scene;
use scene::{Window, Scene, Resources};

const MAX_RAY_DEPTH: u8 = 50;

// TODO: Make a non-recursive trace_ray?

fn trace_ray<A: Accelerator>(
    ray: &Ray,
    world: &Resources,
    bvh: &A,
    depth: u8,
) -> Vec3 {
    let mut hit_record = HitRecord::zero();
    if bvh.hit(&world.entities, ray, 0.001, std::f32::MAX, &mut hit_record) {
        let mut scattered = Ray::zero();
        let mut attenuation = Vec3::zero();

        let material = world.get_material(hit_record.material as usize);
        let emitted = material.emitted(&world.textures, hit_record.u, hit_record.v, &hit_record.p);

        if depth < MAX_RAY_DEPTH
            && material.scatter(&world.textures, ray, &hit_record, &mut attenuation, &mut scattered)
        {
            emitted + attenuation * trace_ray(&scattered, world, bvh, depth + 1)
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

fn calculate_pixel<A: Accelerator>(
    index: usize,
    window: &Window,
    scene: &Scene,
    bvh: &A,
) -> RGB {
    let width = window.width as usize;
    let height = window.height as usize;
    let col = index % width;
    let row = height - 1 - (index / width);

    let mut colour = Vec3::zero();
    for _ in 0..window.samples {
        let u = (col as f32 + drand48()) / width as f32;
        let v = (row as f32 + drand48()) / height as f32;
        let r = window.camera.get_ray(u, v);

        colour += trace_ray(&r, &scene.resources, bvh, 0);
    }

    colour = gamma(colour / window.samples as f32);
    vec_to_rgb(colour.r(), colour.g(), colour.b())
}

fn run() -> Result<(), String> {
    let nx: usize = 600;
    let ny: usize = 600;
    let ns: usize = 100;

//    let scene = "simple_light";
    let scene = "cornell_box";
//    let scene = "final_scene";
    let (scene, window) = load_scene(scene, nx as u32, ny as u32, ns as u32)?;
    let bvh = Bvh::new(&scene.resources.entities, 0.0, 1.0);
//    let bvh2 = CompactBvh::new(&mut scene.resources.entities, 0.0, 1.0);

    assert!(
        (scene.resources.materials.len() as u16) < u16::MAX,
//        (scene.materials.len() as u16) < u16::MAX,
        format!("The maximum supported number of materials is {}", u16::MAX)
    );

    let start = PreciseTime::now();

    // TODO: Support args for output file name + which scene to run
    // TODO: Implement a version of this that builds buffers of rays to process (maybe store as SoA?)
    // TODO: How to handle multiple types of Hitable object? Turn everything into meshes/triangles? How would spheres be done?

    // Is this the best way to do it? or is parallelism over sub images/tiles better?

    let mut pixels: Vec<RGB> = vec![RGB{data: [0, 0, 0]}; nx * ny];
    // Make each thread only process a row
    pixels
        .par_chunks_mut(nx)
//        .chunks_mut(nx)
        .enumerate()
        .for_each(|(chunk_idx, chunk)| {
            let chunk_idx = chunk_idx * nx;
            for (idx, pixel) in chunk.iter_mut().enumerate() {
                *pixel = calculate_pixel(chunk_idx + idx, &window, &scene, &bvh)
            }
        });
//    pixels
////        .iter_mut()
//        .par_iter_mut()
//        .enumerate()
//        .for_each(|(idx, pixel)| {
//            *pixel = calculate_pixel(idx, &window, &scene, &bvh)
//        });

    let image = Image::from_vec(pixels, window.width, window.height);

    // TODO: Work out whether MaterialEnum/TextureEnum are faster than trait solution using the final scene

    let duration: f32 = start.to(PreciseTime::now()).num_milliseconds() as f32 / 1000.0;
    println!("{} seconds for run.", duration);
    println!(
        "Created {:?} rays ({} rays / second).",
        RAY_COUNT,
        RAY_COUNT.load(Ordering::Relaxed) as f32 / duration
    );

    match image.save("images/current_progress.jpg") {
        Ok(()) => Ok(()),
        Err(e) => Err(e.to_string())
    }
}

fn main() -> Result<(), String> {
    run()

}
