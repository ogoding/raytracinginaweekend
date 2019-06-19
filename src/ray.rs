#![allow(dead_code)]

use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};
use vec3::Vec3;

pub static RAY_COUNT: AtomicUsize = AtomicUsize::new(0);

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub _origin: Vec3,
    pub _direction: Vec3,
    time: f32,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3, time: f32) -> Ray {
        RAY_COUNT.fetch_add(1, Ordering::Relaxed);
        Ray {
            _origin: origin,
            _direction: direction,
            time,
        }
    }

    pub fn zero() -> Ray {
        RAY_COUNT.fetch_add(1, Ordering::Relaxed);
        Ray {
            _origin: Vec3::zero(),
            _direction: Vec3::zero(),
            time: 0.0,
        }
    }

    pub fn origin(&self) -> Vec3 {
        self._origin
    }

    pub fn direction(&self) -> Vec3 {
        self._direction
    }

    pub fn time(&self) -> f32 {
        self.time
    }

    pub fn inverse_direction(&self) -> Vec3 {
        self._direction.recip()
    }

    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self._origin + t * self._direction
    }
}

impl fmt::Display for Ray {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "origin: {}, direction: {}",
            self._origin,
            self._direction
        )
    }
}
