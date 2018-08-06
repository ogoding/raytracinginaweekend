extern crate rand;
extern crate time;
extern crate xorshift;

mod image;
mod vec3;
mod ray;
mod hitable;
mod sphere;
mod camera;
mod material;
mod random;

use vec3::Vec3;
//use image::{PixelPusher, Image, RGB};
use image::{Image, RGB};
use ray::{Ray, RAY_COUNT};
use hitable::{Hitable, HitableList, HitRecord};
use sphere::Sphere;
use camera::Camera;
use material::Material;
use random::drand48;

use time::{PreciseTime, Duration};
use std::sync::atomic::Ordering;

fn material_colour<T: Hitable>(ray: &Ray, world: &T, depth: u8) -> Vec3 {
    if let Some(hit_record) = world.hit(ray, 0.001, std::f32::MAX) {
        let mut scattered = Ray::zero();
        let mut attenuation = Vec3::zero();

        if depth < 50 && hit_record.material.unwrap().scatter(ray, &hit_record, &mut attenuation, &mut scattered) {
            attenuation * material_colour(&scattered, world, depth + 1)
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

fn make_scene() -> HitableList<Sphere> {
    HitableList::new(vec![
        Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Material::Lambertian(Vec3::new(0.1, 0.2, 0.5))),
        Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Material::Lambertian(Vec3::new(0.8, 0.8, 0.0))),
        Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, Material::Metal(Vec3::new(0.8, 0.6, 0.2), 0.3)),
        Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, Material::Dieletric(1.5)),
        Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.45, Material::Dieletric(1.5))
    ])
}

fn make_random_scene() -> HitableList<Sphere> {
    let mut spheres = vec![Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Material::Lambertian(Vec3::uniform(0.5)))];

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = drand48();
            let center = Vec3::new(a as f32 + 0.9 * drand48(), 0.2, b as f32 + 0.9 * drand48());
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    spheres.push(Sphere::new(center, 0.2,
                                             Material::Lambertian(Vec3::new(drand48() * drand48(),
                                                           drand48() * drand48(),
                                                           drand48() * drand48()))));
                } else if choose_mat < 0.95 {
                    spheres.push(Sphere::new(center, 0.2,
                                             Material::Metal(Vec3::new(0.5 * (1.0 + drand48()),
                                                           0.5 * (1.0 + drand48()),
                                                           0.5 * (1.0 + drand48())),
                                                         0.5 * drand48())))
                } else {
                    spheres.push(Sphere::new(center, 0.2, Material::Dieletric(1.5)))
                }
            }
        }
    }

    spheres.push(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, Material::Dieletric(1.5)));
    spheres.push(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, Material::Lambertian(Vec3::new(0.4, 0.2, 0.1))));
    spheres.push(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, Material::Metal(Vec3::new(0.7, 0.6, 0.5), 0.0)));

    HitableList::new(spheres)
}

fn make_camera(nx: u32, ny: u32) -> Camera {
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    Camera::new(lookfrom, lookat, Vec3::new(0.0, 1.0, 0.0), 20.0, nx as f32 / ny as f32, aperture, dist_to_focus)
}

fn run() {
    let nx: u32 = 600;
    let ny: u32 = 400;
    let ns: u32 = 10;
//    let world = make_scene();
    let world = make_random_scene();
    let cam = make_camera(nx, ny);

//    let mut image = PixelPusher::new(Image::new(nx, ny));
    let mut image = Image::new(nx, ny);

    let start = PreciseTime::now();
    for j in 0..ny {
        for i in 0..nx {
            let mut colour = Vec3::zero();
            for _ in 0..ns {
                let u = (i as f32 + drand48()) / nx as f32;
                let v = (j as f32 + drand48()) / ny as f32;
                let r = cam.get_ray(u, v);

                colour += material_colour(&r, &world, 0);
            }

            colour = gamma(colour / ns as f32);

//            image.push_pixel(RGB::new_scaled(colour.r(), colour.g(), colour.b()));
            image.set(ny - 1 - j, i, RGB::new_scaled(colour.r(), colour.g(), colour.b()));
        }
    }

    let duration: f32 = start.to(PreciseTime::now()).num_milliseconds() as f32 / 1000.0;
    println!("{} seconds for run.", duration);
    println!("Created {:?} rays ({} rays / second).", RAY_COUNT, RAY_COUNT.load(Ordering::Relaxed) as f32 / duration);

    std::fs::write("images/current_progress.ppm", image.to_ppm());
}

fn main() {
    run();
}
