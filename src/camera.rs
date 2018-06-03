use vec3::Vec3;
use ray::Ray;
use std::f32::consts::PI;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3
}

impl Camera {
    // vfov is top to bottom in degrees
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f32, aspect: f32) -> Camera {
        let theta = vfov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = (lookfrom - lookat).unit();
        let u = Vec3::cross(&vup, &w).unit();
        let v = Vec3::cross(&w, &u);

        let origin = lookfrom;
        let lower_left_corner = origin - half_width * u - half_height * v - w;
        let horizontal = 2.0 * half_width * u;
        let vertical = 2.0 * half_height * v;

        Camera{ origin, lower_left_corner, horizontal, vertical }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + (s * self.horizontal) + (t * self.vertical) - self.origin)
    }
}
