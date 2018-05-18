mod image;
mod vec3;

use vec3::Vec3;
use image::{Image, RGB};

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

fn main() {
    example_1();
    example_2();
}
