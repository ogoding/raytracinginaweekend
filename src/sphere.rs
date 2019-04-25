use aabb::{surrounding_box, AABBVolume};
use hitable::{HitRecord, Hitable};
use scene::Entities;
use scene::MaterialRef;
use ray::Ray;
use vec3::Vec3;

use std::f32::consts::{FRAC_PI_2, PI};

fn get_sphere_uv(p: &Vec3) -> (f32, f32) {
    let phi = p.z().atan2(p.x());
    let theta = p.y().asin();
    (1.0 - (phi + PI) / (2.0 * PI), (theta + FRAC_PI_2) / PI)
}

#[derive(Debug, Clone)]
pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: MaterialRef,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: MaterialRef) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }

    #[inline(always)]
    pub fn update_hit_record(&self, ray: &Ray, t: f32, hit_record: &mut HitRecord) {
        let p = ray.point_at_parameter(t);
        let (u, v) = get_sphere_uv(&((p - self.center) / self.radius));
        hit_record.t = t;
        hit_record.p = p;
        hit_record.u = u;
        hit_record.v = v;
        hit_record.normal = (p - self.center) / self.radius;
        hit_record.material = self.material;
    }
}

impl Hitable for Sphere {
    fn hit_ptr(&self, entities: &Entities, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let oc = ray.origin() - self.center;
        let ray_direction = ray.direction();
        let a = ray_direction.squared_length();
        let b = Vec3::dot(&oc, &ray_direction);
        let c = oc.squared_length() - self.radius.powi(2);
        let discriminant = b.powi(2) - a * c;

        if discriminant > 0.0 {
            let discrim_sqrt = discriminant.sqrt();
            let mut temp = (-b - discrim_sqrt) / a;
            if t_max > temp && temp > t_min {
                self.update_hit_record(ray, temp, hit_record);
                return true;
            }

            temp = (-b + discrim_sqrt) / a;
            if t_max > temp && temp > t_min {
                self.update_hit_record(ray, temp, hit_record);
                return true;
            }
        }

        false
    }

    fn bounding_box(&self, _t_min: f32, _t_max: f32) -> Option<AABBVolume> {
        Some(AABBVolume::new(
            self.center - Vec3::uniform(self.radius),
            self.center + Vec3::uniform(self.radius),
        ))
    }
}

#[derive(Debug)]
pub struct MovingSphere {
    center0: Vec3,
    center1: Vec3,
    radius: f32,
    material: MaterialRef,
    time0: f32,
    time1: f32,
}

impl MovingSphere {
    pub fn new(
        center0: Vec3,
        center1: Vec3,
        t0: f32,
        t1: f32,
        radius: f32,
        material: MaterialRef,
    ) -> MovingSphere {
        MovingSphere {
            center0,
            center1,
            radius,
            material,
            time0: t0,
            time1: t1,
        }
    }

    #[inline(always)]
    pub fn update_hit_record(&self, ray: &Ray, t: f32, hit_record: &mut HitRecord) {
        let p = ray.point_at_parameter(t);
        let (u, v) = get_sphere_uv(&((p - self.center(t)) / self.radius));
        hit_record.t = t;
        hit_record.p = p;
        hit_record.u = u;
        hit_record.v = v;
        hit_record.normal = (p - self.center(ray.time())) / self.radius;
        hit_record.material = self.material;
    }

    #[inline]
    fn center(&self, time: f32) -> Vec3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl Hitable for MovingSphere {
    fn hit_ptr(&self, entities: &Entities, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let oc = ray.origin() - self.center(ray.time());
        let ray_direction = ray.direction();
        let a = ray_direction.squared_length();
        let b = Vec3::dot(&oc, &ray_direction);
        let c = oc.squared_length() - self.radius.powi(2);
        let discriminant = b.powi(2) - a * c;

        if discriminant > 0.0 {
            let discrim_sqrt = discriminant.sqrt();
            let mut temp = (-b - discrim_sqrt) / a;
            if t_max > temp && temp > t_min {
                self.update_hit_record(ray, temp, hit_record);
                return true;
            }

            temp = (-b + discrim_sqrt) / a;
            if t_max > temp && temp > t_min {
                self.update_hit_record(ray, temp, hit_record);
                return true;
            }
        }

        false
    }

    fn bounding_box(&self, t_min: f32, t_max: f32) -> Option<AABBVolume> {
        let center_min = self.center(t_min);
        let center_max = self.center(t_max);
        let box0 = AABBVolume::new(
            center_min - Vec3::uniform(self.radius),
            center_min + Vec3::uniform(self.radius),
        );
        let box1 = AABBVolume::new(
            center_max - Vec3::uniform(self.radius),
            center_max + Vec3::uniform(self.radius),
        );

        Some(surrounding_box(box0, box1))
    }
}
