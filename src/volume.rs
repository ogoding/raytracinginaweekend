use vec3::Vec3;
use ray::Ray;
use hitable::{Hitable, HitRecord};
use texture::Texture;
use material::{Material, Isotropic};
use random::drand48;

use std::f32;

pub struct ConstantMedium {
    // FIXME: Is using Box right? - replace with a ref?
    boundary: Box<Hitable>,
    density: f32,
    // FIXME: Is using Box right? - replace with a ref?
    phase_function: Box<Material>
}

impl ConstantMedium {
    // FIXME: Is using  'static right?
    fn new<T: Texture + 'static>(boundary: Box<Hitable>, density: f32, phase_function: T) -> ConstantMedium {
        ConstantMedium{ boundary, density, phase_function: Box::new(Isotropic::new(phase_function)) }
    }

    // FIXME: Is using  'static right?
    pub fn new_boxed<T: Texture + 'static>(boundary: Box<Hitable>, density: f32, phase_function: T) -> Box<ConstantMedium> {
        Box::new(ConstantMedium::new(boundary, density, phase_function))
    }
}

/*
bool constant_medium::hit(const ray& r, float t_min, float t_max, hit_record& rec) const {
    bool db = (drand48() < 0.00001);
    db = false;
    hit_record rec1, rec2;
    if (boundary->hit(r, f32::MIN, f32::MAX, rec1)) {
        if (boundary->hit(r, rec1.t+0.0001, f32::MAX, rec2)) {
    if (db) std::cerr << "\nt0 t1 " << rec1.t << " " << rec2.t << "\n";
            if (rec1.t < t_min)
                rec1.t = t_min;
            if (rec2.t > t_max)
                rec2.t = t_max;
            if (rec1.t >= rec2.t)
                return false;
            if (rec1.t < 0)
                rec1.t = 0;
            float distance_inside_boundary = (rec2.t - rec1.t)*r.direction().length();
            float hit_distance = -(1/density)*log(drand48());
            if (hit_distance < distance_inside_boundary) {
            if (db) std::cerr << "hit_distance = " <<  hit_distance << "\n";
                rec.t = rec1.t + hit_distance / r.direction().length();
            if (db) std::cerr << "rec.t = " <<  rec.t << "\n";
                rec.p = r.point_at_parameter(rec.t);
            if (db) std::cerr << "rec.p = " <<  rec.p << "\n";
                rec.normal = vec3(1,0,0);  // arbitrary
                rec.mat_ptr = phase_function;
                return true;
            }
        }
    }
    return false;
}
*/

impl Hitable for ConstantMedium {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
//        let db = drand48() < 0.00001;
//        let db = false;

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
                    return Some(HitRecord::new(t, ray.point_at_parameter(t), 0.0, 0.0, Vec3::new(1.0, 0.0, 0.0), self.phase_function.as_ref()))
                }
            }
        }

        None
    }
}