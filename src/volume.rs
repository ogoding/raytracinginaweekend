use vec3::Vec3;
use ray::Ray;
use hitable::{Hitable, HitRecord};
use aabb::AABBVolume;
use material::MaterialIndex;
use random::drand48;

use std::f32;

pub struct ConstantMedium<H: Hitable> {
    boundary: H,
    density: f32,
    phase_function: MaterialIndex
}

impl <H: Hitable> ConstantMedium<H> {
    fn new(boundary: H, density: f32, phase_function: MaterialIndex) -> ConstantMedium<H> {
        ConstantMedium{ boundary, density, phase_function }
    }

    pub fn new_boxed(boundary: H, density: f32, phase_function: MaterialIndex) -> Box<ConstantMedium<H>> {
        Box::new(ConstantMedium::new(boundary, density, phase_function))
    }
}

impl <H: Hitable> Hitable for ConstantMedium<H> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if let Some(hit1) = self.boundary.hit(ray, f32::MIN, f32::MAX) {
            if let Some(hit2) = self.boundary.hit(ray, hit1.t + 0.0001, f32::MAX) {
                let mut hit1 = hit1;
                let mut hit2 = hit2;

                if hit1.t < t_min { hit1.t = t_min; }
                if hit2.t > t_max { hit2.t = t_max; }
                if hit1.t >= hit2.t { return None; }
                if hit1.t < 0.0 { hit1.t = 0.0; }

                let distance_inside_boundary = (hit2.t - hit1.t) * ray.direction().length();
                // TODO: Confirm that it should be log2, not some other power/base/exponent
                let hit_distance = -(1.0 / self.density) * drand48().log2();
                if hit_distance < distance_inside_boundary {
                    let t = hit1.t + hit_distance / ray.direction().length();
                    return Some(HitRecord::new(t, ray.point_at_parameter(t), 0.0, 0.0, Vec3::new(1.0, 0.0, 0.0), self.phase_function))
                }
            }
        }

        None
    }

    fn bounding_box(&self, t_min: f32, t_max: f32) -> Option<AABBVolume> {
        self.boundary.bounding_box(t_min, t_max)
    }
}