use vec3::Vec3;
use ray::Ray;
use std::f32::consts::PI;

// TODO: Blurring is borked, doesn't blend properly
fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = 2.0 * Vec3::random_2d() - Vec3::uniform_2d(1.0);

        if Vec3::dot(&p, &p) >= 1.0 {
            return p;
        }
    }
}

#[derive(Debug)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32
}

impl Camera {
    // vfov is top to bottom in degrees
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f32, aspect: f32, aperture: f32, focus_dist: f32) -> Camera {
        let theta = vfov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        println!("aspect = {}, half_height = {}, half_width = {}", aspect, half_height, half_width);

        let w = (lookfrom - lookat).unit();
        let u = Vec3::cross(&vup, &w).unit();
        let v = Vec3::cross(&w, &u);

        let origin = lookfrom;
        let lower_left_corner = origin - half_width * focus_dist * u - half_height * focus_dist * v - focus_dist * w;
        let horizontal = 2.0 * half_width * focus_dist * u;
        let vertical = 2.0 * half_height * focus_dist * v;

        let cam = Camera{ origin, lower_left_corner, horizontal, vertical, u, v, w, lens_radius: aperture / 2.0 };
        println!("{:?}", cam);
        cam
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        // TODO: Blurring is borked, doesn't blend properly
//        let rd = self.lens_radius * random_in_unit_disk();
//        let offset = self.u * rd.x() + self.v * rd.y();
//        Ray::new(self.origin + offset, self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset)
        Ray::new(self.origin, self.lower_left_corner + (s * self.horizontal) + (t * self.vertical) - self.origin)
    }
}
