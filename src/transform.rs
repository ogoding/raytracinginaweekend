use aabb::AABBVolume;
use hitable::{HitRecord, Hitable};
use scene::Entities;
use ray::Ray;
use vec3::Vec3;

use std::f32;

// TODO: Add a wrapper type for storing a reference to some Primitive/Geometry in order to do proper instancing- e.g. Translate::new(GeometryRef(2), Vec3::uniform(5.0))

#[derive(Debug)]
pub struct FlipNormals<H> {
    ptr: H,
}

impl<H: Hitable> FlipNormals<H> {
    pub fn new(ptr: H) -> FlipNormals<H> {
        FlipNormals { ptr }
    }
}

impl<H: Hitable> Hitable for FlipNormals<H> {
    fn hit_ptr(&self, entities: &Entities, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let hit = self.ptr.hit_ptr(entities, ray, t_min, t_max, hit_record);
        if hit {
            hit_record.normal = -hit_record.normal;
            //            hit_record.normal *= -1.0;
        }
        hit
    }

    fn bounding_box(&self, t_min: f32, t_max: f32) -> Option<AABBVolume> {
        self.ptr.bounding_box(t_min, t_max)
    }
}

#[derive(Debug)]
pub struct Translate<T: Hitable> {
    ptr: T,
    offset: Vec3,
}

impl<T: Hitable> Translate<T> {
    pub fn new(ptr: T, offset: Vec3) -> Translate<T> {
        Translate { ptr, offset }
    }
}

impl<T: Hitable> Hitable for Translate<T> {
    fn hit_ptr(&self, entities: &Entities, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let moved_ray = Ray::new(ray.origin() - self.offset, ray.direction(), ray.time());

        let hit = self.ptr.hit_ptr(entities, &moved_ray, t_min, t_max, hit_record);
        if hit {
            hit_record.p += self.offset;
        }
        hit
    }

    fn bounding_box(&self, t_min: f32, t_max: f32) -> Option<AABBVolume> {
        if let Some(aabb) = self.ptr.bounding_box(t_min, t_max) {
            Some(AABBVolume::new(
                aabb.min() + self.offset,
                aabb.max() + self.offset,
            ))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct RotateY<T: Hitable> {
    ptr: T,
    sin_theta: f32,
    cos_theta: f32,
    // Pre-calculate and store bbox as calculating on the fly is too expensive
    aabb_box: Option<AABBVolume>,
}

impl<T: Hitable> RotateY<T> {
    pub fn new(ptr: T, angle: f32) -> RotateY<T> {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        let aabb_box = ptr.bounding_box(0.0, 1.0).unwrap_or_else(AABBVolume::zero);
        let mut min = Vec3::uniform(f32::MAX);
        let mut max = Vec3::uniform(f32::MIN);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f32 * aabb_box.max().x() + (1 - i) as f32 * aabb_box.min().x();
                    let y = j as f32 * aabb_box.max().y() + (1 - j) as f32 * aabb_box.min().y();
                    let z = k as f32 * aabb_box.max().z() + (1 - k) as f32 * aabb_box.min().z();
                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;
                    let tester = Vec3::new(newx, y, newz);

                    for c in 0..3 {
                        if tester[c] > max[c] {
                            max[c] = tester[c];
                        }
                        if tester[c] < min[c] {
                            min[c] = tester[c];
                        }
                    }
                }
            }
        }

        RotateY {
            ptr,
            sin_theta,
            cos_theta,
            aabb_box: Some(AABBVolume::new(min, max)),
        }
    }
}

impl<T: Hitable> Hitable for RotateY<T> {
    fn hit_ptr(&self, entities: &Entities, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let ray_origin = ray.origin();
        let ray_direction = ray.direction();

        let origin = Vec3::new(
            self.cos_theta * ray_origin.x() - self.sin_theta * ray_origin.z(),
            ray_origin.y(),
            self.sin_theta * ray_origin.x() + self.cos_theta * ray_origin.z(),
        );

        let direction = Vec3::new(
            self.cos_theta * ray_direction.x() - self.sin_theta * ray_direction.z(),
            ray_direction.y(),
            self.sin_theta * ray_direction.x() + self.cos_theta * ray_direction.z(),
        );

        let rotated_ray = Ray::new(origin, direction, ray.time());

        if self.ptr.hit_ptr(entities, &rotated_ray, t_min, t_max, hit_record) {
            let p = Vec3::new(
                self.cos_theta * hit_record.p.x() + self.sin_theta * hit_record.p.z(),
                hit_record.p.y(),
                -self.sin_theta * hit_record.p.x() + self.cos_theta * hit_record.p.z(),
            );
            let normal = Vec3::new(
                self.cos_theta * hit_record.normal.x() + self.sin_theta * hit_record.normal.z(),
                hit_record.normal.y(),
                -self.sin_theta * hit_record.normal.x() + self.cos_theta * hit_record.normal.z(),
            );

            hit_record.p = p;
            hit_record.normal = normal;

            true
        } else {
            false
        }
    }

    fn bounding_box(&self, _t_min: f32, _t_max: f32) -> Option<AABBVolume> {
        self.aabb_box
    }
}
