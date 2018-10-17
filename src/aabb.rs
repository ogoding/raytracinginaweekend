use vec3::Vec3;
use ray::Ray;

use std::mem::swap;

#[inline]
fn ffmin(a: f32, b: f32) -> f32 {
    if a < b { a } else { b }
}

#[inline]
fn ffmax(a: f32, b: f32) -> f32 {
    if a > b { a } else { b }
}

pub fn surrounding_box(box0: AABBVolume, box1: AABBVolume) -> AABBVolume {
    AABBVolume{
        min: Vec3::new(ffmin(box0.min.x(), box1.min.x()),
                       ffmin(box0.min.y(), box1.min.y()),
                       ffmin(box0.min.z(), box1.min.z())),
        max: Vec3::new(ffmax(box0.max.x(), box1.max.x()),
                       ffmax(box0.max.y(), box1.max.y()),
                       ffmax(box0.max.z(), box1.max.z()))
    }
}

#[derive(Debug, Copy, Clone)]
pub struct AABBVolume {
    min: Vec3,
    max: Vec3
}

impl AABBVolume {
    pub fn zero() -> AABBVolume {
        AABBVolume{ min: Vec3::zero(), max: Vec3::zero() }
    }

    pub fn new(min: Vec3, max: Vec3) -> AABBVolume {
        AABBVolume{ min, max }
    }

    pub fn min(&self) -> Vec3 {
        self.min
    }

    pub fn max(&self) -> Vec3 {
        self.max
    }

    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / ray.direction()[a];
            let mut t0 = (self.min[a] - ray.origin()[a]) * inv_d;
            let mut t1 = (self.max[a] - ray.origin()[a]) * inv_d;
            if inv_d < 0.0 {
                swap(&mut t0, &mut t1);
            }

            let t_min = ffmax(t0, t_min);
            let t_max = ffmin(t1, t_max);
            if t_max <= t_min {
                return false;
            }
        }

        true
    }
}