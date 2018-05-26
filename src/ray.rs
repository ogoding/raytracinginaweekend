#![allow(dead_code)]

use std::fmt;
use vec3::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray{ origin, direction }
    }

    pub fn zero() -> Ray {
        Ray{ origin: Vec3::zero(), direction: Vec3::zero() }
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}

impl fmt::Display for Ray {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "o: {}, dir: {}", self.origin(), self.direction())
    }
}
