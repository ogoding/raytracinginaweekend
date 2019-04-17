#![allow(dead_code, unused_variables)]

use imagers::RgbImage;
use perlin;
use vec3::Vec3;

pub trait Texture {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3;
}

pub struct ConstantTexture {
    colour: Vec3,
}

impl ConstantTexture {
    pub fn zero() -> ConstantTexture {
        ConstantTexture {
            colour: Vec3::zero(),
        }
    }

    pub fn new(colour: Vec3) -> ConstantTexture {
        ConstantTexture { colour }
    }
}

impl Texture for ConstantTexture {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
        self.colour
    }
}

pub struct CheckerTexture<T: Texture> {
    odd: T,
    even: T,
}

impl<T: Texture> CheckerTexture<T> {
    pub fn new(t0: T, t1: T) -> CheckerTexture<T> {
        CheckerTexture { odd: t0, even: t1 }
    }
}

impl CheckerTexture<ConstantTexture> {
    pub fn new_solid(colour1: Vec3, colour2: Vec3) -> CheckerTexture<ConstantTexture> {
        CheckerTexture {
            odd: ConstantTexture::new(colour1),
            even: ConstantTexture::new(colour2),
        }
    }
}

impl<T: Texture> Texture for CheckerTexture<T> {
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
    scale: f32,
}

impl ScaledPerlinTexture {
    pub fn new(scale: f32) -> ScaledPerlinTexture {
        ScaledPerlinTexture { scale }
    }
}

impl Texture for ScaledPerlinTexture {
    fn value(&self, _u: f32, _v: f32, p: &Vec3) -> Vec3 {
        Vec3::uniform(1.0) * perlin::noise(&(*p * self.scale))
    }
}

pub struct ScaledTurbulencePerlinTexture {
    scale: f32,
}

impl ScaledTurbulencePerlinTexture {
    pub fn new(scale: f32) -> ScaledTurbulencePerlinTexture {
        ScaledTurbulencePerlinTexture { scale }
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
    img: RgbImage,
}

impl ImageTexture {
    pub fn new(image_path: &str) -> ImageTexture {
        ImageTexture {
            img: ::imagers::open(image_path).unwrap().to_rgb(),
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
        let (nx, ny) = self.img.dimensions();
        let mut i = (u * nx as f32) as u32;
        let mut j = ((1.0 - v) * ny as f32 - 0.001) as u32;

        i = i.min(nx - 1).max(0);
        j = j.min(ny - 1).max(0);

        let pixel = self.img[(i as u32, j as u32)];

        let r = f32::from(pixel[0]) / 255.0;
        let g = f32::from(pixel[1]) / 255.0;
        let b = f32::from(pixel[2]) / 255.0;

        Vec3::new(r, g, b)
    }
}



pub type TextureRef = usize;

#[derive(Debug)]
pub enum TextureEnum {
    Constant(Vec3),
    Checker(TextureRef, TextureRef),
    Perlin,
    ScaledPerlin(f32),
    ScaledTurbulencePerlin(f32),
    Image(RgbImage)
}

impl TextureEnum {
    // TODO: Any benefit in splitting these out into their own functions? e.g. reduce the value function size
    pub fn value(&self, textures: &[TextureEnum], u: f32, v: f32, p: &Vec3) -> Vec3 {
        match self {
            TextureEnum::Constant(albedo) => *albedo,
            TextureEnum::Checker(odd_tex, even_tex) => {
                let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();

                if sines < 0.0 {
                    textures[*odd_tex].value(textures, u, v, p)
                } else {
                    textures[*even_tex].value(textures, u, v, p)
                }
            },
            TextureEnum::Perlin => Vec3::uniform(1.0) * perlin::noise(p),
            TextureEnum::ScaledPerlin(scale) => Vec3::uniform(1.0) * perlin::noise(&(*p * *scale)),
            TextureEnum::ScaledTurbulencePerlin(scale) => Vec3::uniform(1.0) * 0.5 * (1.0 + (*scale * p.z() + 10.0 * perlin::turb(p, 7)).sin()),
            TextureEnum::Image(image) => {
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
        }
    }
}