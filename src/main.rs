//#![feature(nll)]

extern crate rand;
extern crate time;
extern crate xorshift;
#[macro_use] extern crate lazy_static;
extern crate rayon;
extern crate image as imagers;

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
//mod volume;

use vec3::Vec3;
//use image::{PixelPusher, Image, RGB};
use image::{Image, RGB};
use ray::{Ray, RAY_COUNT};
use hitable::{Hitable, HitableList, FlipNormals};
use sphere::{Sphere, MovingSphere};
use camera::Camera;
use aarect::{XYRect, XZRect, YZRect};
use material::{Material, Lambertian, LambertianTextured, Metal, Dieletric, DiffuseLight};
use texture::{ConstantTexture, CheckerTexture, PerlinTexture, ScaledPerlinTexture, ScaledTurbulencePerlinTexture, ImageTexture};
//use volume::ConstantMedium;
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

#[allow(dead_code)]
//fn make_scene() -> HitableList<Sphere> {
fn make_scene(nx: u32, ny: u32) -> (HitableList, Camera) {
    let world = HitableList::new(vec![
        Sphere::new_boxed(Vec3::new(0.0, 0.0, -1.0), 0.5, Box::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5)))),
        Sphere::new_boxed(Vec3::new(0.0, -100.5, -1.0), 100.0, Box::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)))),
        Sphere::new_boxed(Vec3::new(1.0, 0.0, -1.0), 0.5, Box::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.3))),
        Sphere::new_boxed(Vec3::new(-1.0, 0.0, -1.0), 0.5, Box::new(Dieletric::new(1.5))),
        Sphere::new_boxed(Vec3::new(-1.0, 0.0, -1.0), -0.45, Box::new(Dieletric::new(1.5)))
    ]);

    (world, make_default_camera(nx, ny))
}

#[allow(dead_code)]
//fn make_random_scene() -> HitableList<Sphere> {
fn make_random_scene(nx: u32, ny: u32) -> (HitableList, Camera) {
//    let mut spheres: Vec<Box<Hitable>> = vec![Sphere::new_boxed(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Box::new(Lambertian::new(Vec3::uniform(0.5))))];
//    let mut spheres: Vec<Box<Hitable>> = vec![Sphere::new_boxed(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Box::new(LambertianTextured::new(ConstantTexture::new(Vec3::uniform(0.5)))))];
    let mut spheres: Vec<Box<Hitable>> = vec![Sphere::new_boxed(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Box::new(LambertianTextured::new(CheckerTexture::new_solid(Vec3::new(0.2, 0.3, 0.1), Vec3::uniform(0.9)))))];

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

    (HitableList::new(spheres), make_default_camera(nx, ny))
}

#[allow(dead_code)]
fn make_random_moving_scene(nx: u32, ny: u32) -> (HitableList, Camera) {
//    let mut spheres: Vec<Box<Hitable>> = vec![Sphere::new_boxed(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Box::new(Lambertian::new(Vec3::uniform(0.5))))];
//    let mut spheres: Vec<Box<Hitable>> = vec![Sphere::new_boxed(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Box::new(LambertianTextured::new(ConstantTexture::new(Vec3::uniform(0.5)))))];
    let mut spheres: Vec<Box<Hitable>> = vec![Sphere::new_boxed(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Box::new(LambertianTextured::new(CheckerTexture::new_solid(Vec3::new(0.2, 0.3, 0.1), Vec3::uniform(0.9)))))];

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

    (HitableList::new(spheres), make_default_camera(nx, ny))
}

#[allow(dead_code)]
fn make_two_spheres_scene(nx: u32, ny: u32) -> (HitableList, Camera) {
    // TODO: Remove duplication
    let checker = CheckerTexture::new_solid(Vec3::new(0.2, 0.3, 0.1), Vec3::uniform(0.9));
    let checker2 = CheckerTexture::new_solid(Vec3::new(0.2, 0.3, 0.1), Vec3::uniform(0.9));

    let world = HitableList::new(
    vec![Sphere::new_boxed(Vec3::new(0.0, -10.0, 0.0), 10.0, Box::new(LambertianTextured::new(checker))),
         Sphere::new_boxed(Vec3::new(0.0, 10.0, 0.0), 10.0, Box::new(LambertianTextured::new(checker2)))]);

    (world, make_default_camera(nx, ny))
}

#[allow(dead_code)]
fn make_two_perlin_spheres_scene(nx: u32, ny: u32) -> (HitableList, Camera) {
    // TODO: Remove duplication
//    let checker = PerlinTexture{};
//    let checker2 = PerlinTexture{};
//    let checker = ScaledPerlinTexture::new(1.0);
//    let checker2 = ScaledPerlinTexture::new(1.0);
    let checker = ScaledTurbulencePerlinTexture::new(4.0);
    let checker2 = ScaledTurbulencePerlinTexture::new(4.0);

    let world = HitableList::new(
        vec![Sphere::new_boxed(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Box::new(LambertianTextured::new(checker))),
             Sphere::new_boxed(Vec3::new(0.0, 2.0, 0.0), 2.0, Box::new(LambertianTextured::new(checker2)))]);
    (world, make_default_camera(nx, ny))
}

#[allow(dead_code)]
fn make_earth_scene(nx: u32, ny: u32) -> (HitableList, Camera) {
    let world = HitableList::new(vec![
        Sphere::new_boxed(Vec3::zero(), 2.0, Box::new(LambertianTextured::new(ImageTexture::new("earthmap.jpg"))))
    ]);

    (world, make_default_camera(nx, ny))
}

#[allow(dead_code)]
fn make_simple_light_scene(nx: u32, ny: u32) -> (HitableList, Camera) {
    // TODO: Remove duplication
    let checker = ScaledTurbulencePerlinTexture::new(4.0);
    let checker2 = ScaledTurbulencePerlinTexture::new(4.0);

    let world = HitableList::new(vec![
        Sphere::new_boxed(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Box::new(LambertianTextured::new(checker))),
        Sphere::new_boxed(Vec3::new(0.0, 2.0, 0.0), 2.0, Box::new(LambertianTextured::new(checker2))),
        Sphere::new_boxed(Vec3::new(0.0, 7.0, 0.0), 2.0, Box::new(DiffuseLight::new(ConstantTexture::new(Vec3::uniform(4.0))))),
        XYRect::new_boxed(3.0, 5.0, 1.0, 3.0, -2.0, Box::new(DiffuseLight::new(ConstantTexture::new(Vec3::uniform(4.0)))))
    ]);

//    (world, make_default_camera(nx, ny))
    (world, make_camera(Vec3::new(18.0, 5.0, 3.0), Vec3::new(0.0, 0.0, 0.0), 40.0, nx as u32, ny as u32, 0.1, 10.0))
}

#[allow(dead_code)]
fn make_cornell_box(nx: u32, ny: u32) -> (HitableList, Camera) {
    let red = LambertianTextured::new(ConstantTexture::new(Vec3::new(0.65, 0.05, 0.05)));
    let green= LambertianTextured::new(ConstantTexture::new(Vec3::new(0.12, 0.45, 0.15)));
    // TODO: Remove duplication
    let white = LambertianTextured::new(ConstantTexture::new(Vec3::uniform(0.73)));
    let white2 = LambertianTextured::new(ConstantTexture::new(Vec3::uniform(0.73)));
    let white3 = LambertianTextured::new(ConstantTexture::new(Vec3::uniform(0.73)));
    let light = DiffuseLight::new(ConstantTexture::new(Vec3::uniform(15.0)));

    let world = HitableList::new(vec![
        FlipNormals::new_boxed(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, Box::new(green))),        // Left plane
        YZRect::new_boxed(0.0, 555.0, 0.0, 555.0, 0.0, Box::new(red)),                                   // Right plane
        XZRect::new_boxed(213.0, 343.0, 227.0, 322.0, 554.0, Box::new(light)),                           // Top light
        XZRect::new_boxed(0.0, 555.0, 0.0, 555.0, 0.0, Box::new(white)),                                 // Bottom plane
        FlipNormals::new_boxed(XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, Box::new(white2))),       // Top plane
        FlipNormals::new_boxed(XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, Box::new(white3)))        // Back plane
    ]);

    (world, make_camera(Vec3::new(278.0, 278.0, -800.0), Vec3::new(278.0, 278.0, 0.0), 40.0, nx as u32, ny as u32, 0.0, 1.0))
}

#[allow(dead_code)]
fn make_cornell_smoke(nx: u32, ny: u32) -> (HitableList, Camera) {
    let red = LambertianTextured::new(ConstantTexture::new(Vec3::new(0.65, 0.05, 0.05)));
    let green= LambertianTextured::new(ConstantTexture::new(Vec3::new(0.12, 0.45, 0.15)));
    // TODO: Remove duplication
    let white = LambertianTextured::new(ConstantTexture::new(Vec3::uniform(0.73)));
    let white2 = LambertianTextured::new(ConstantTexture::new(Vec3::uniform(0.73)));
    let white3 = LambertianTextured::new(ConstantTexture::new(Vec3::uniform(0.73)));
    let white4 = LambertianTextured::new(ConstantTexture::new(Vec3::uniform(0.73)));
    let white5 = LambertianTextured::new(ConstantTexture::new(Vec3::uniform(0.73)));
    let light = DiffuseLight::new(ConstantTexture::new(Vec3::uniform(15.0)));

//    let b1 = Translate::new(RotateY::new(Cube::new(Vec3::uniform(0.0), Vec3::uniform(165.0), white4), -18.0), Vec3::new(130.0, 0.0, 65.0));
//    let b1 = Translate::new(RotateY::new(Cube::new(Vec3::uniform(0.0), Vec3::new(165.0, 330.0, 165.0), white5), 15.0), Vec3::new(265.0, 0.0, 295.0));

    let world = HitableList::new(vec![
        FlipNormals::new_boxed(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, Box::new(green))),        // Left plane
        YZRect::new_boxed(0.0, 555.0, 0.0, 555.0, 0.0, Box::new(red)),                                   // Right plane
        XZRect::new_boxed(213.0, 343.0, 227.0, 322.0, 554.0, Box::new(light)),                           // Top light
        XZRect::new_boxed(0.0, 555.0, 0.0, 555.0, 0.0, Box::new(white)),                                 // Bottom plane
        FlipNormals::new_boxed(XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, Box::new(white2))),       // Top plane
        FlipNormals::new_boxed(XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, Box::new(white3))),       // Back plane
//        ConstantMedium::new_boxed(b1, 0.01, ConstantTexture::new(Vec3::uniform(1.0))),
//        ConstantMedium::new_boxed(b2, 0.01, ConstantTexture::new(Vec3::uniform(0.0)))
    ]);

    (world, make_camera(Vec3::new(278.0, 278.0, -800.0), Vec3::new(278.0, 278.0, 0.0), 40.0, nx as u32, ny as u32, 0.0, 1.0))
}

fn make_camera(lookfrom: Vec3, lookat: Vec3, vfov: f32, nx: u32, ny: u32, aperture: f32, dist_to_focus: f32) -> Camera {
    Camera::new(lookfrom, lookat, Vec3::new(0.0, 1.0, 0.0), vfov, nx as f32 / ny as f32, aperture, dist_to_focus, 0.0, 1.0)
}

fn make_default_camera(nx: u32, ny: u32) -> Camera {
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
    let ns: usize = 100;
//    let (world, cam) = make_scene(nx as u32, ny as u32);
//    let (world, cam) = make_random_scene(nx as u32, ny as u32);
//    let (world, cam) = make_random_moving_scene(nx as u32, ny as u32);
//    let (world, cam) = make_two_spheres_scene(nx as u32, ny as u32);
//    let (world, cam) = make_earth_scene(nx as u32, ny as u32);
//    let (world, cam) = make_two_perlin_spheres_scene(nx as u32, ny as u32);
//    let (world, cam) = make_simple_light_scene(nx as u32, ny as u32);
    let (world, cam) = make_cornell_box(nx as u32, ny as u32);

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

// TODO: Remaining Chapters of Book 2
// TODO: 2: Bounded Volume Hierarchies
// TODO: 7: Instances
// TODO: 8: Volumes - Needs testing and implementing #7
// TODO: 9: A Scene Testing - All new features