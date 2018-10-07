#![allow(dead_code, unused_variables)]

use vec3::Vec3;
use perlin;
use imagers::{RgbImage};

pub trait Texture {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3;
}

pub struct ConstantTexture {
    colour: Vec3
}

impl ConstantTexture {
    pub fn zero() -> ConstantTexture {
        ConstantTexture{ colour: Vec3::zero() }
    }

    pub fn new(colour: Vec3) -> ConstantTexture {
        ConstantTexture{ colour }
    }
}

impl Texture for ConstantTexture {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
        self.colour
    }
}

pub struct CheckerTexture<T: Texture> {
    odd: T,
    even: T
}

impl <T: Texture> CheckerTexture<T> {
    pub fn new(t0: T, t1: T) -> CheckerTexture<T> {
        CheckerTexture{ odd: t0, even: t1 }
    }
}

impl CheckerTexture<ConstantTexture> {
    pub fn new_solid(colour1: Vec3, colour2: Vec3) -> CheckerTexture<ConstantTexture> {
        CheckerTexture{ odd: ConstantTexture::new(colour1), even: ConstantTexture::new(colour2) }
    }
}

impl <T: Texture> Texture for CheckerTexture<T> {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
        let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();

        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

pub struct PerlinTexture {}

impl Texture for PerlinTexture {
    fn value(&self, _u: f32, _v: f32, p: &Vec3) -> Vec3 {
        Vec3::uniform(1.0) * perlin::noise(p)
    }
}

pub struct ScaledPerlinTexture {
    scale: f32
}

impl ScaledPerlinTexture {
    pub fn new(scale: f32) -> ScaledPerlinTexture {
        ScaledPerlinTexture{ scale }
    }
}

impl Texture for ScaledPerlinTexture {
    fn value(&self, _u: f32, _v: f32, p: &Vec3) -> Vec3 {
        Vec3::uniform(1.0) * perlin::noise(&(*p * self.scale))
    }
}

pub struct ScaledTurbulencePerlinTexture {
    scale: f32
}

impl ScaledTurbulencePerlinTexture {
    pub fn new(scale: f32) -> ScaledTurbulencePerlinTexture {
        ScaledTurbulencePerlinTexture{ scale }
    }
}

impl Texture for ScaledTurbulencePerlinTexture {
    fn value(&self, _u: f32, _v: f32, p: &Vec3) -> Vec3 {
//        Vec3::uniform(1.0) * 0.5 * perlin::turb(&(*p * self.scale), 7)
//        Vec3::uniform(1.0) * perlin::turb(&(*p * self.scale), 7)
        Vec3::uniform(1.0) * 0.5 * (1.0 + (self.scale * p.z() + 10.0 * perlin::turb(p, 7)).sin())
    }
}

pub struct ImageTexture {
    img: RgbImage
}

impl ImageTexture {
    pub fn new(image_path: &str) -> ImageTexture {
        ImageTexture{ img: ::imagers::open(image_path).unwrap().to_rgb() }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
        let (nx, ny) = self.img.dimensions();
        let mut i = u as u32 * nx;
        let mut j = ((1.0 - v) * ny as f32 - 0.001) as u32;
        i = i.min(0).max(nx - 1);
        j = j.min(0).max(ny - 1);
//        if i < 0 { i = 0 };
//        if j < 0 { j = 0 };
//        if i > nx - 1 { i = nx - 1 };
//        if j > ny - 1 { j = ny - 1 };

        let pixel = self.img[(i as u32, j as u32)];

        let r = pixel[0] as f32 / 255.0;
        let g = pixel[1] as f32 / 255.0;
        let b = pixel[2] as f32 / 255.0;
//        let r = self.data[3 * i + 3 * nx * j] / 255.0;
//        let g = self.data[3 * i + 3 * nx * j + 1] / 255.0;
//        let b = self.data[3 * i + 3 * nx * j + 2] / 255.0;

        Vec3::new(r, g, b)
    }
}
