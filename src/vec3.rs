#![allow(dead_code)]

use random::drand48;

use std::fmt;
use std::num::ParseFloatError;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};
use std::str::FromStr;

#[inline]
fn ffmin(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}

#[inline]
fn ffmax(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3 {
    data: [f32; 3],
}

impl Vec3 {
    pub fn zero() -> Vec3 {
        Vec3::uniform(0.0)
    }

    pub fn uniform(v: f32) -> Vec3 {
        Vec3::new(v, v, v)
    }

    pub fn uniform_2d(v: f32) -> Vec3 {
        Vec3::new(v, v, 0.0)
    }

    pub fn random() -> Vec3 {
        Vec3::new(drand48(), drand48(), drand48())
    }

    pub fn random_2d() -> Vec3 {
        Vec3::new(drand48(), drand48(), 0.0)
    }

    pub fn new(d0: f32, d1: f32, d2: f32) -> Vec3 {
        Vec3 { data: [d0, d1, d2] }
    }

    pub fn recip(&self) -> Vec3 {
        Vec3::new(self[0].recip(), self[1].recip(), self[2].recip())
    }

    pub fn max(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            ffmax(self[0], other[0]),
            ffmax(self[1], other[1]),
            ffmax(self[2], other[2]),
        )
        //        Vec3::new(self[0].max(other[0]), self[1].max(other[1]), self[2].max(other[2]))
    }

    pub fn min(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            ffmin(self[0], other[0]),
            ffmin(self[1], other[1]),
            ffmin(self[2], other[2]),
        )
        //        Vec3::new(self[0].min(other[0]), self[1].min(other[1]), self[2].min(other[2]))
    }

    pub fn max_component(&self) -> f32 {
        ffmax(self[0], ffmax(self[1], self[2]))
        //        self[0].max(self[1].max(self[2]))
    }

    pub fn min_component(&self) -> f32 {
        ffmin(self[0], ffmin(self[1], self[2]))
        //        self[0].min(self[1].min(self[2]))
    }

    pub fn unit(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn length(&self) -> f32 {
        self.squared_length().sqrt()
    }

    pub fn squared_length(&self) -> f32 {
        Vec3::dot(self, self)
    }

    // TODO: rename to convert_to_unit
    pub fn make_unit_vector(&mut self) {
        //        let k = 1.0 / self.length();
        let k = self.length().recip();
        *self *= k;
    }

//    pub fn sum(&self) -> f32 {
//        self[0] + self[1] + self[2]
//    }

    // TODO Change this to be Vec3 instead of &Vec3?
    pub fn dot(v1: &Vec3, v2: &Vec3) -> f32 {
//        (*v1 + *v2).sum()
        v1.x() * v2.x() + v1.y() * v2.y() + v1.z() * v2.z()
    }

    // TODO Change this to be Vec3 instead of &Vec3?
    pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
        Vec3::new(
            v1.y() * v2.z() - v1.z() * v2.y(),
            -(v1.x() * v2.z() - v1.z() * v2.x()),
            v1.x() * v2.y() - v1.y() * v2.x(),
        )
    }

    pub fn x(&self) -> f32 {
        self.data[0]
    }

    pub fn y(&self) -> f32 {
        self.data[1]
    }

    pub fn z(&self) -> f32 {
        self.data[2]
    }

    pub fn r(&self) -> f32 {
        self.data[0]
    }

    pub fn g(&self) -> f32 {
        self.data[1]
    }

    pub fn b(&self) -> f32 {
        self.data[2]
    }
}

impl FromStr for Vec3 {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let values: Vec<&str> = s.split_whitespace().collect();

        if values.len() != 3 {
            // TODO: Change panic to something else
            panic!("Invalid number of vec3 values ({}: {})", values.len(), s);
        } else {
            let x = values[0].parse::<f32>()?;
            let y = values[1].parse::<f32>()?;
            let z = values[2].parse::<f32>()?;
            Ok(Vec3::new(x, y, z))
        }
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.x(), self.y(), self.z())
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, idx: usize) -> &f32 {
        &self.data[idx]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, idx: usize) -> &mut <Self as Index<usize>>::Output {
        &mut self.data[idx]
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

impl Add<Vec3> for f32 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self + other.x(), self + other.y(), self + other.z())
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        )
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self = Vec3::new(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        );
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        )
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        *self = Vec3::new(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        );
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self * other.x(), self * other.y(), self * other.z())
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f32) -> Vec3 {
        Vec3::new(self.x() * other, self.y() * other, self.z() * other)
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, other: f32) {
        *self = Vec3::new(self.x() * other, self.y() * other, self.z() * other);
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.x() * other.x(),
            self.y() * other.y(),
            self.z() * other.z(),
        )
    }
}

impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        *self = Vec3::new(
            self.x() * other.x(),
            self.y() * other.y(),
            self.z() * other.z(),
        );
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f32) -> Vec3 {
        Vec3::new(self.x() / other, self.y() / other, self.z() / other)
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, other: f32) {
        *self = Vec3::new(self.x() / other, self.y() / other, self.z() / other);
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.x() / other.x(),
            self.y() / other.y(),
            self.z() / other.z(),
        )
    }
}

impl DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, other: Vec3) {
        *self = Vec3::new(
            self.x() / other.x(),
            self.y() / other.y(),
            self.z() / other.z(),
        );
    }
}
