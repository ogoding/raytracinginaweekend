#![allow(dead_code)]

use std::fmt;
use vec3::Vec3;
use std::sync::atomic::{AtomicUsize, Ordering};

pub static RAY_COUNT: AtomicUsize = AtomicUsize::new(0);

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
    time: f32
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3, time: f32) -> Ray {
        RAY_COUNT.fetch_add(1, Ordering::Relaxed);
        Ray{ origin, direction, time }
    }

    pub fn zero() -> Ray {
        RAY_COUNT.fetch_add(1, Ordering::Relaxed);
        Ray{ origin: Vec3::zero(), direction: Vec3::zero(), time: 0.0 }
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn time(&self) -> f32 {
        self.time
    }

    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}

impl fmt::Display for Ray {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "origin: {}, direction: {}", self.origin(), self.direction())
    }
}
