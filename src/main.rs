mod image;
mod vec3;
mod ray;

use vec3::Vec3;
use image::{Image, RGB};
use ray::Ray;

fn example_1() {
    let mut image = Image::new(3, 2);
    image.set(0, 0, RGB::new(255, 0, 0));
    image.set(0, 1, RGB::new(0, 255, 0));
    image.set(0, 2, RGB::new(0, 0, 255));
    image.set(1, 0, RGB::new(255, 255, 0));
    image.set(1, 1, RGB::new(255, 255, 255));
    image.set(1, 2, RGB::new(0, 0, 0));

    std::fs::write("example_1.ppm", image.to_ppm());
}

fn example_2() {
    let nx: u32 = 200;
    let ny: u32 = 100;
    let mut image = Image::new(nx, ny);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let col = Vec3::new(i as f32 / nx as f32, j as f32 / ny as f32, 0.2);
            image.set((ny - 1) - j, i, RGB::new_scaled(col.r(), col.g(), col.b()));
        }
    }

    std::fs::write("example_2.ppm", image.to_ppm());
}

fn hit_sphere(center: Vec3, radius: f32, ray: &Ray) -> bool {
    let oc = ray.origin() - center;
    let ray_dir = ray.direction();
    let a = Vec3::dot(&ray_dir, &ray_dir);
    let b = Vec3::dot(&oc, &ray_dir) * 2.0;
    let c = Vec3::dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}

fn lerp_colour(ray: &Ray) -> Vec3 {
    if hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, ray) {
        return Vec3::new(1.0, 0.0, 0.0);
    }
    let unit_direction = ray.direction().unit();
    let t = 0.5 * (unit_direction.y() + 1.0);
    Vec3::uniform(1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn example_3() {
    let nx: u32 = 200;
    let ny: u32 = 100;
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::uniform(0.0);
    let mut image = Image::new(nx, ny);



    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let r = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);
            let col = lerp_colour(&r);
            image.set((ny - 1) - j, i, RGB::new_scaled(col.r(), col.g(), col.b()));
        }
    }

    std::fs::write("example_3.ppm", image.to_ppm());
}

fn main() {
    example_1();
    example_2();
    example_3();
}
