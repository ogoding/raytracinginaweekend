use vec3::Vec3;
use ray::Ray;
use hitable::{Hitable, HitRecord};

pub struct Sphere {
    center: Vec3,
    radius: f32
}

impl Sphere {
    pub fn zero() -> Sphere {
        Sphere::new(Vec3::zero(), 0.0)
    }

    pub fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere{ center, radius }
    }

    pub fn update_hit_record(&self, record: &mut HitRecord, ray: &Ray, t: f32) {
        record.t = t;
        record.p = ray.point_at_parameter(record.t);
        record.normal = (record.p - self.center) / self.radius;
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool {
        let oc = ray.origin() - self.center;
        let ray_direction = ray.direction();
        let a = Vec3::dot(&ray_direction, &ray_direction);
        let b = Vec3::dot(&oc, &ray_direction);
        let c = Vec3::dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        // TODO Refactor this to be simpler
        if discriminant > 0.0 {
            let mut temp = (-b - discriminant.sqrt()) / a;
            if t_max > temp && temp > t_min {
                self.update_hit_record(record, ray, temp);
                return true;
            }

            temp = (-b + discriminant.sqrt()) / a;
            if t_max > temp && temp > t_min {
                self.update_hit_record(record, ray, temp);
                return true;
            }
        }

        false
    }
}