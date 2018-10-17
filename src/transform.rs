use vec3::Vec3;
use ray::Ray;
use hitable::{Hitable, HitRecord};
use aabb::AABBVolume;

use std::f32;

// TODO: Add a wrapper type for storing a reference to some Primative/Geometry in order to do proper instancing- e.g. Translate::new(GeometryRef(2), Vec3::uniform(5.0))

pub struct FlipNormals<H> {
    ptr: H
}

impl <H: Hitable> FlipNormals<H> {
    pub fn new(ptr: H) -> FlipNormals<H> {
        FlipNormals{ ptr }
    }

    pub fn new_boxed(ptr: H) -> Box<FlipNormals<H>> {
        Box::new(FlipNormals::new(ptr))
    }
}

impl <H: Hitable> Hitable for FlipNormals<H> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
//        let hit = self.ptr.hit(ray, t_min, t_max);
//        if let Some(hit) = self.ptr.hit(ray, t_min, t_max) {
//            let mut hit = hit;
//            hit.normal = -hit.normal;
//            Some(hit)
//        } else {
//            None
//        }

        // TODO: Confirm that this is faster than the above code
        let mut hit = self.ptr.hit(ray, t_min, t_max);
        if let Some(ref mut record) = hit { record.normal = -record.normal }
        hit
    }

    fn bounding_box(&self, t_min: f32, t_max: f32) -> Option<AABBVolume> {
        self.ptr.bounding_box(t_min, t_max)
    }
}

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
//        if let Some(hit) = self.ptr.hit(&moved_ray, t_min, t_max) {
//            let mut hit = hit;
//            hit.p += self.offset;
//            Some(hit)
//        } else {
//            None
//        }
        // TODO: Confirm that this is faster than the above code
        let mut hit = self.ptr.hit(&moved_ray, t_min, t_max);
        if let Some(ref mut record) = hit { record.p += self.offset; }
        hit
    }

    fn bounding_box(&self, t_min: f32, t_max: f32) -> Option<AABBVolume> {
        if let Some(aabb) = self.ptr.bounding_box(t_min, t_max) {
            Some(AABBVolume::new(aabb.min() + self.offset, aabb.max() + self.offset))
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
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        // TODO: Work out if this is needed
//        let has_box = self.ptr.bounding_box(0.0, 1.0);
        // TODO: Work out correct default
        let aabb_box = ptr.bounding_box(0.0, 1.0).unwrap_or(AABBVolume::zero());
        let mut min = Vec3::uniform(f32::MAX);
        let mut max = Vec3::uniform(f32::MIN);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f32 * aabb_box.max().x() + (1 - i) as f32  * aabb_box.min().x();
                    let y = j as f32 * aabb_box.max().y() + (1 - j) as f32  * aabb_box.min().y();
                    let z = k as f32 * aabb_box.max().z() + (1 - k) as f32  * aabb_box.min().z();
                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;
                    let tester = Vec3::new(newx, y, newz);

                    for c in 0..3 {
                        if tester[c] > max[c] { max[c] = tester[c]; }
                        if tester[c] < min[c] { min[c] = tester[c]; }
                    }
                }
            }
        }

        RotateY{ ptr, sin_theta, cos_theta, aabb_box: Some(AABBVolume::new(min, max)) }
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

    fn bounding_box(&self, _t_min: f32, _t_max: f32) -> Option<AABBVolume> {
        self.aabb_box
    }
}
