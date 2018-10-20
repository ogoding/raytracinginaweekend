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
    fn hit_ptr(&self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let mut hit1 = HitRecord::zero();
        let mut hit2 = HitRecord::zero();

        if self.boundary.hit_ptr(ray, f32::MIN, f32::MAX, &mut hit1) {
            if self.boundary.hit_ptr(ray, hit1.t + 0.0001, f32::MAX, &mut hit2) {
                if hit1.t < t_min { hit1.t = t_min; }
                if hit2.t > t_max { hit2.t = t_max; }
                if hit1.t >= hit2.t { return false; }
                if hit1.t < 0.0 { hit1.t = 0.0; }

                let distance_inside_boundary = (hit2.t - hit1.t) * ray.direction().length();
                // TODO: Confirm that it should be log2, not some other power/base/exponent
                let hit_distance = -(1.0 / self.density) * drand48().log2();
                if hit_distance < distance_inside_boundary {
                    let t = hit1.t + hit_distance / ray.direction().length();
                    hit_record.t = t;
                    hit_record.p = ray.point_at_parameter(t);
                    hit_record.normal = Vec3::new(1.0, 0.0, 0.0);
                    hit_record.material = self.phase_function;
                    return true;
                }
            }
        }

        false
    }


    fn bounding_box(&self, t_min: f32, t_max: f32) -> Option<AABBVolume> {
        self.boundary.bounding_box(t_min, t_max)
    }
}