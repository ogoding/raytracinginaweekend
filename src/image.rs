// TODO: Replace Image struct with image crate - support range of image formats, etc

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

pub struct Image {
    pub pixels: Vec<RGB>,
    pub width: u32,
    pub height: u32
}

impl Image {
    pub fn new(width: u32, height: u32) -> Image {
        Image{ pixels: vec![RGB::new(0, 0, 0); (width * height) as usize], width, height }
    }

    pub fn from_vec(pixels: Vec<RGB>, width: u32, height: u32) ->  Image {
        Image{ pixels, width, height }
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
