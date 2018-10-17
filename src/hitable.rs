use vec3::Vec3;
use ray::Ray;
use material::MaterialIndex;
use aabb::{AABBVolume, surrounding_box};
//use typed_buffer::TypedBuffer;

use std::ops::Index;

#[derive(Debug, Clone)]
pub struct HitRecord {
    // FIXME: Improve naming
    pub t: f32,
    // FIXME: Improve naming
    pub p: Vec3,
    pub u: f32,
    pub v: f32,
    pub normal: Vec3,
    pub material: MaterialIndex
}

impl HitRecord {
    pub fn new(t: f32, p: Vec3, u: f32, v: f32, normal: Vec3, material: MaterialIndex) -> HitRecord {
        HitRecord{ t, p, u, v, normal, material }
    }
}

pub trait Hitable {
    // TODO: Change this to pass in a reference to the materials list (and make a type that falls back to something like a pink/checkerboard error texture+material)
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, t_min: f32, t_max: f32) -> Option<AABBVolume>;
}

// TODO: Remove generics and create push methods to box and add Hitable to internal list
// TODO: This will likely require changing internal list to Vec<Box<dyn Hitable>> or do something fancy with an untyped arena/allocator
pub struct HitableList {
    // TODO: Do something smart with a Arena or map of typeid, Vec<SomeHitableType>
    // TODO: Try Cell/RefCell/AtomicCell or AtomicPtr or using explicit lifetimes or even a Weak or raw ptrs (not ideal)
    list: Vec<Box<dyn Hitable>>
}

impl HitableList {
    pub fn new(list: Vec<Box<dyn Hitable>>) -> HitableList {
        HitableList{ list }
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn list_as_mut(&mut self) -> &mut Vec<Box<dyn Hitable>> {
        &mut self.list
    }
}

unsafe impl Send for HitableList {}
unsafe impl Sync for HitableList {}

impl Hitable for HitableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut temp_rec = None;
        let mut closest_so_far = t_max;

        // Could this escape loop early? and/or be a map/reduce?
        for hitable in self.list.iter() {
//            if let Some(record) = hitable.hit(ray, t_min, closest_so_far) {
//                closest_so_far = record.t;
//                temp_rec = Some(record);
//            }

            let record = hitable.hit(ray, t_min, closest_so_far);
            if record.is_some() {
                closest_so_far = record.clone().unwrap().t;
                temp_rec = record;
            }
        }

        temp_rec
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABBVolume> {
        if self.list.len() > 0 {
            let mut result = match self.list[0].bounding_box(t0, t1) {
                Some(bounding_box) => bounding_box,
                None => return None
            };

            for hitable in &self.list[1..] {
                match hitable.bounding_box(t0, t1) {
                    Some(bounding_box) => result = surrounding_box(result, bounding_box),
                    None => return None
                }
            }

            Some(result)
        } else {
            None
        }
    }
}

impl Index<usize> for HitableList {
    type Output = Box<Hitable>;

    fn index(&self, idx: usize) -> &Box<Hitable> {
        &self.list[idx]
    }
}
