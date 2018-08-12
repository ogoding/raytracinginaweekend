#![allow(dead_code)]

#[derive(Copy, Clone)]
pub struct RGB {
    red: u8,
    green: u8,
    blue: u8
}

impl RGB {
    pub fn new(red: u8, green: u8, blue: u8) -> RGB {
        RGB{ red, green, blue }
    }

    pub fn new_scaled(red: f32, green: f32, blue: f32) -> RGB {
        RGB::new((255.99 * red) as u8, (255.99 * green) as u8, (255.99 * blue) as u8)
    }

    fn to_ppm(&self) -> String {
        self.red.to_string() + " " + &self.green.to_string() + " " + &self.blue.to_string() + "\n"
    }
}

pub struct PixelPusher {
    image: Image,
    width_idx: u32,
    height_idx: u32
}

impl PixelPusher {
    pub fn new(image: Image) -> PixelPusher {
        PixelPusher{ image, width_idx: 0, height_idx: 0 }
    }

    // TODO Add the scaled RGB pixel thing as a function here and remove from RGB type?

    // TODO Change args to be individual rgb valuess?
    pub fn push_pixel(&mut self, rgb: RGB) -> bool {
        if self.width_idx >= self.width() && self.height_idx >= self.height() {
            return false;
        }

        self.image.set(self.height_idx, self.width_idx, rgb);

        self.width_idx += 1;
        if self.width_idx >= self.width() {
            self.width_idx = 0;
            self.height_idx += 1;
        }

        true
    }

    pub fn width(&self) -> u32 {
        self.image.width
    }

    pub fn height(&self) -> u32 {
        self.image.height
    }

    pub fn into_image(self) -> Image {
        self.image
    }
}

pub struct Image {
    pub pixels: Vec<RGB>,
    pub width: u32,
    pub height: u32
}

impl Image {
    pub fn new(width: u32, height: u32) -> Image {
        Image{ pixels: vec![RGB::new(0, 0, 0); (width * height) as usize], width, height }
    }

    pub fn set(&mut self, row: u32, col: u32, rgb: RGB) {
        self.pixels[(row * self.width + col) as usize] = rgb;
    }
//    pub fn load_ppm(file: &str) -> Image {
//
//    }

    pub fn to_ppm(&self) -> String {
        let mut ppm_string: String = String::from("P3\n");
        ppm_string.push_str(&(self.width.to_string() + " " + &self.height.to_string() + "\n"));
        ppm_string.push_str("255\n");

        // TODO Change this to only write newlines at the end of a row
        for pixel in &self.pixels {
            ppm_string.push_str(&pixel.to_ppm());
        }

        ppm_string
    }
}
