use vec3::Vec3;
use ray::Ray;
use material::Material;
use hitable::{Hitable, HitRecord};

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Material
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Material) -> Sphere {
        Sphere{ center, radius, material }
    }

    #[inline(always)]
    pub fn create_hit_record(&self, ray: &Ray, t: f32) -> HitRecord {
        let p = ray.point_at_parameter(t);
        HitRecord::new(t, p, (p - self.center) / self.radius, self.material)
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = Vec3::dot(&ray.direction(), &ray.direction());
        let b = Vec3::dot(&oc, &ray.direction());
        let c = Vec3::dot(&oc, &oc) - self.radius.powi(2);
        let discriminant = b.powi(2) - a * c;

        if discriminant > 0.0 {
            let discrim_sqrt = discriminant.sqrt();
            let mut t = (-b - discrim_sqrt) / a;
            if t_max > t && t > t_min {
                return Some(self.create_hit_record(ray, t));
            }

            t = (-b + discrim_sqrt) / a;
            if t_max > t && t > t_min {
                return Some(self.create_hit_record(ray, t));
            }
        }

        None
    }
}