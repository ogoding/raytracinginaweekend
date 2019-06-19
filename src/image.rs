use std::io::Result;
use imagers::{RgbImage, ImageBuffer, Rgb};
use std::path::Path;
use std::iter::once;

pub type RGB = Rgb<u8>;

fn clamp(value: f32, min: f32, max: f32) -> f32 {
    value.max(min).min(max)
}

#[inline(always)]
fn scale_f32_to_u8(float: f32) -> u8 {
    // TODO: Ask Peter Shirley whether there is a better method of doing this (and that the book doesn't cover it)
    // TODO: Handle a higher dynamic range than 0.0 -> 1.0
    (clamp(float, 0.0, 1.0) * 255.99) as u8
}

pub fn vec_to_rgb(red: f32, green: f32, blue: f32) -> RGB {
    Rgb {
        data: [
            scale_f32_to_u8(red),
            scale_f32_to_u8(green),
            scale_f32_to_u8(blue)
        ]
    }
}

pub struct Image {
    pub image: RgbImage
}

impl Image {

    pub fn from_vec(pixels: Vec<RGB>, width: u32, height: u32) -> Image {
        let pixels: Vec<u8> = pixels.iter()
            .flat_map(|&pixel| {
                once(pixel[0]).chain(once(pixel[1])).chain(once(pixel[2]))
            }).collect();
        Image {
            image: ImageBuffer::from_vec(width, height, pixels).unwrap()
        }
    }

    pub fn save<T: AsRef<Path>>(&self, path: T) -> Result<()> {
        self.image.save(path)
    }
}
