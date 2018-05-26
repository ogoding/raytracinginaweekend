extern crate rand;

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

use rand::Rng;

fn example_1() {
    let mut image = PixelPusher::new(Image::new(3, 2));
    image.push_pixel(RGB::new(255, 0, 0));
    image.push_pixel(RGB::new(0, 255, 0));
    image.push_pixel(RGB::new(0, 0, 255));
    image.push_pixel(RGB::new(255, 255, 0));
    image.push_pixel(RGB::new(255, 255, 255));
    image.push_pixel(RGB::new(0, 0, 0));

    std::fs::write("example_1.ppm", image.into_image().to_ppm());
}

fn example_2() {
    let nx: u32 = 200;
    let ny: u32 = 100;
    let mut image = PixelPusher::new(Image::new(nx, ny));

    for j in (0..ny).rev() {
        for i in 0..nx {
            let col = Vec3::new(i as f32 / nx as f32, j as f32 / ny as f32, 0.2);
            image.push_pixel(RGB::new_scaled(col.r(), col.g(), col.b()));
        }
    }

    std::fs::write("example_2.ppm", image.into_image().to_ppm());
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

fn make_example_3_list() -> HitableList<Sphere> {
    HitableList::new(vec![
        Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)
    ])
}

fn make_example_4_list() -> HitableList<Sphere> {
    HitableList::new(vec![
        Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5),
        Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)
    ])
}

fn example_3() {
    let nx: u32 = 200;
    let ny: u32 = 100;
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::uniform(0.0);
    let world = make_example_3_list();

    let mut image = PixelPusher::new(Image::new(nx, ny));

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let r = Ray::new(origin, lower_left_corner + (u * horizontal) + (v * vertical));
            let colour = lerp_colour(&r, &world);
            image.push_pixel(RGB::new_scaled(colour.r(), colour.g(), colour.b()));
        }
    }

    std::fs::write("example_3.ppm", image.into_image().to_ppm());
}

fn example_4() {
    let nx: u32 = 200;
    let ny: u32 = 100;
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::uniform(0.0);
    let world = make_example_4_list();

    let mut image = PixelPusher::new(Image::new(nx, ny));

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let r = Ray::new(origin, lower_left_corner + (u * horizontal) + (v * vertical));
            let colour = lerp_colour(&r, &world);
            image.push_pixel(RGB::new_scaled(colour.r(), colour.g(), colour.b()));
        }
    }

    std::fs::write("example_4.ppm", image.into_image().to_ppm());
}

fn example_5() {
    let nx: u32 = 200;
    let ny: u32 = 100;
    let ns: u32 = 100;
    let world = make_example_4_list();
    let cam = Camera::new();

    let mut image = PixelPusher::new(Image::new(nx, ny));

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut colour = Vec3::zero();
            for s in 0..ns {
                let u = (i as f32 + rand::random::<f32>()) / nx as f32;
                let v = (j as f32 + rand::random::<f32>()) / ny as f32;
                let r = cam.get_ray(u, v);
                colour += lerp_colour(&r, &world);
            }

            colour = colour / ns as f32;
//            colour /= ns as f32;

            image.push_pixel(RGB::new_scaled(colour.r(), colour.g(), colour.b()));
        }
    }

    std::fs::write("example_5.ppm", image.into_image().to_ppm());
}

fn main() {
    example_1();
    example_2();
    example_3();
    example_4();
    example_5();
}
