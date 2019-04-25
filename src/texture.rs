#![allow(dead_code, unused_variables)]

use imagers::RgbImage;
use perlin;
use vec3::Vec3;

pub type TextureRef = usize;

fn checker(textures: &[Texture], u: f32, v: f32, p: &Vec3, odd_tex: usize, even_tex: usize) -> Vec3 {
    let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();

    if sines < 0.0 {
        textures[odd_tex].value(textures, u, v, p)
    } else {
        textures[even_tex].value(textures, u, v, p)
    }
}

fn image(u: f32, v: f32, image: &RgbImage) -> Vec3 {
    let (nx, ny) = image.dimensions();
    let mut i = (u * nx as f32) as u32;
    let mut j = ((1.0 - v) * ny as f32 - 0.001) as u32;

    i = i.min(nx - 1).max(0);
    j = j.min(ny - 1).max(0);

    let pixel = image[(i as u32, j as u32)];

    let r = f32::from(pixel[0]) / 255.0;
    let g = f32::from(pixel[1]) / 255.0;
    let b = f32::from(pixel[2]) / 255.0;

    Vec3::new(r, g, b)
}

#[derive(Debug)]
pub enum Texture {
    Constant(Vec3),
    Checker(TextureRef, TextureRef),
    Perlin,
    ScaledPerlin(f32),
    ScaledTurbulencePerlin(f32),
    Image(RgbImage)
}

impl Texture {
    // TODO: Any benefit in splitting these out into their own functions? e.g. reduce the value function size
    pub fn value(&self, textures: &[Texture], u: f32, v: f32, p: &Vec3) -> Vec3 {
        match self {
            Texture::Constant(albedo) => *albedo,
            Texture::Checker(odd_tex, even_tex) => checker(textures, u, v, p, *odd_tex, *even_tex),
            Texture::Perlin => Vec3::uniform(1.0) * perlin::noise(p),
            Texture::ScaledPerlin(scale) => Vec3::uniform(1.0) * perlin::noise(&(*p * *scale)),
            Texture::ScaledTurbulencePerlin(scale) => Vec3::uniform(1.0) * 0.5 * (1.0 + (*scale * p.z() + 10.0 * perlin::turb(p, 7)).sin()),
            Texture::Image(image_t) => image(u, v, image_t)
        }
    }
}
