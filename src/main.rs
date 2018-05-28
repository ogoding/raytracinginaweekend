extern crate rand;
extern crate time;

mod image;
mod vec3;
mod ray;
mod hitable;
mod sphere;
mod camera;

use vec3::Vec3;
use image::{PixelPusher, Image, RGB};
use ray::Ray;
use hitable::{Hitable, HitableList, HitRecord};
use sphere::Sphere;
use camera::Camera;

use time::PreciseTime;

fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = 2.0 * Vec3::new(rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>()) - Vec3::uniform(1.0);

        if p.squared_length() >= 1.0 {
            return p;
        }
    }
}

fn diffuse_lerp_colour<T: Hitable>(ray: &Ray, world: &T) -> Vec3 {
    let mut hit_record = HitRecord::zero();
    if world.hit(ray, 0.001, std::f32::MAX, &mut hit_record) {
        let target = hit_record.p + hit_record.normal + random_in_unit_sphere();
        0.5 * diffuse_lerp_colour( &Ray::new(hit_record.p, target - hit_record.p), world)
    } else {
        let unit_direction = ray.direction().unit();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Vec3::uniform(1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn gamma(vec: Vec3) -> Vec3 {
    Vec3::new(vec.x().sqrt(), vec.y().sqrt(), vec.z().sqrt())
}

fn lerp_colour<T: Hitable>(ray: &Ray, world: &T) -> Vec3 {
    let mut hit_record = HitRecord::zero();
    if world.hit(ray, 0.0, std::f32::MAX, &mut hit_record) {
        0.5 * (hit_record.normal + Vec3::uniform(1.0))
    } else {
        let unit_direction = ray.direction().unit();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Vec3::uniform(1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn make_scene() -> HitableList<Sphere> {
    HitableList::new(vec![
        Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5),
        Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)
    ])
}

fn run() {
    let nx: u32 = 200;
    let ny: u32 = 100;
    let ns: u32 = 100;
    let world = make_scene();
    let cam = Camera::new();

    let mut image = PixelPusher::new(Image::new(nx, ny));

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut colour = Vec3::zero();
            for _s in 0..ns {
                let u = (i as f32 + rand::random::<f32>()) / nx as f32;
                let v = (j as f32 + rand::random::<f32>()) / ny as f32;
                let r = cam.get_ray(u, v);
                colour += diffuse_lerp_colour(&r, &world);
            }

            colour = colour / ns as f32;
//            colour /= ns as f32;
            colour = gamma(colour);

            image.push_pixel(RGB::new_scaled(colour.r(), colour.g(), colour.b()));
        }
    }

    std::fs::write("images/current_progress.ppm", image.into_image().to_ppm());
}

fn main() {
    let start = PreciseTime::now();
    run();
    println!("{} seconds for run.", start.to(PreciseTime::now()));
}
