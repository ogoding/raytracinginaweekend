use vec3::Vec3;
use ray::Ray;
use hitable::{Hitable, HitRecord};
use aabb::AABBVolume;

use std::f32;
use std::f32::consts::PI;

pub struct Translate<T: Hitable> {
    ptr: T,
    offset: Vec3
}

impl <T: Hitable> Translate<T> {
    pub fn new(ptr: T, offset: Vec3) -> Translate<T> {
        Translate { ptr, offset }
    }

    pub fn new_boxed(ptr: T, offset: Vec3) -> Box<Translate<T>> {
        Box::new(Translate::new(ptr, offset))
    }
}

impl <T: Hitable> Hitable for Translate<T> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let moved_ray = Ray::new(ray.origin() - self.offset, ray.direction(), ray.time());

//        let hit = self.ptr.hit(ray, t_min, t_max);
        if let Some(hit) = self.ptr.hit(&moved_ray, t_min, t_max) {
            let mut hit = hit;
            hit.p += self.offset;
            Some(hit)
        } else {
            None
        }
    }
}

pub struct RotateY<T: Hitable> {
    ptr: T,
    sin_theta: f32,
    cos_theta: f32,
    aabb_box: Option<AABBVolume>
}

impl <T: Hitable> RotateY<T> {
    pub fn new(ptr: T, angle: f32) -> RotateY<T> {
        // TODO: Implement this
        // TODO: Use something builtin if available
        let radians = (PI / 180.0) * angle;
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        // TODO: Work out if this is needed
//        let has_box = ptr.bounding_box(0.0, 1.0);
//        let aabb_box = ptr.bounding_box(0.0, 1.0);
//        let min = Vec3::uniform(f32::MAX);
//        let max = Vec3::uniform(f32::MIN);

        // TODO: Finish this
//        for i in 0..2 {
//            for j in 0..2 {
//                for k in 0..2 {
//                    let x = ;
//                    let y = ;
//                    let z = ;
//                }
//            }
//        }

        // TODO: Implement the aabb box calculation
        RotateY{ ptr, sin_theta, cos_theta, aabb_box: None }
    }

    pub fn new_boxed(ptr: T, angle: f32) -> Box<RotateY<T>> {
        Box::new(RotateY::new(ptr, angle))
    }
}

// FIXME: Cleanup this function
impl <T: Hitable> Hitable for RotateY<T> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let ray_origin = ray.origin();
        let ray_direction = ray.direction();

        // FIXME: Cleanup this function
        let origin = Vec3::new(self.cos_theta * ray_origin.x() - self.sin_theta * ray_origin.z(),
                               ray_origin.y(),
                               self.sin_theta * ray_origin.x() + self.cos_theta * ray_origin.z());

        let direction = Vec3::new(self.cos_theta * ray_direction.x() - self.sin_theta * ray_direction.z(),
                                    ray_direction.y(),
                                    self.sin_theta * ray_direction.x() + self.cos_theta * ray_direction.z());

        let rotated_ray = Ray::new(origin, direction, ray.time());

        // FIXME: Cleanup this function
        if let Some(hit) = self.ptr.hit(&rotated_ray, t_min, t_max) {
            let mut hit = hit;
            let p = Vec3::new(self.cos_theta * hit.p.x() + self.sin_theta * hit.p.z(),
                                    hit.p.y(),
                                    -self.sin_theta * hit.p.x() + self.cos_theta * hit.p.z());
            let normal = Vec3::new(self.cos_theta * hit.normal.x() + self.sin_theta * hit.normal.z(),
                                        hit.normal.y(),
                                        -self.sin_theta * hit.normal.x() + self.cos_theta * hit.normal.z());

            hit.p = p;
            hit.normal = normal;

            Some(hit)
        } else {
            None
        }
    }
}