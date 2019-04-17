use aabb::{surrounding_box, AABBVolume};
use material::MaterialIndex;
use ray::Ray;
use vec3::Vec3;

use std::ops::Index;

#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    // FIXME: Improve naming
    pub t: f32,
    // FIXME: Improve naming
    pub p: Vec3,
    pub u: f32,
    pub v: f32,
    pub normal: Vec3,
    pub material: MaterialIndex,
}

impl HitRecord {
    pub fn new(
        t: f32,
        p: Vec3,
        u: f32,
        v: f32,
        normal: Vec3,
        material: MaterialIndex,
    ) -> HitRecord {
        HitRecord {
            t,
            p,
            u,
            v,
            normal,
            material,
        }
    }

    pub fn zero() -> HitRecord {
        HitRecord::new(0.0, Vec3::zero(), 0.0, 0.0, Vec3::zero(), 0)
    }
}

// TODO: Try making an enum of all hitable things like material and texture?
pub trait Hitable {
    fn hit_ptr(&self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool;
    fn bounding_box(&self, t_min: f32, t_max: f32) -> Option<AABBVolume>;
}

// TODO: Rework this so that list_as_mut isn't required - Have some kind of NotYetFinalisedHitableList or MutableHitableList that is converted to an ImmutableHitableList
// TODO: This will likely require changing internal list to Vec<Box<dyn Hitable>> or do something fancy with an untyped arena/allocator
pub struct HitableList {
    // TODO: Do something smart with a Arena or map of typeid, Vec<SomeHitableType>
    // TODO: Try Cell/RefCell/AtomicCell or AtomicPtr or using explicit lifetimes or even a Weak or raw ptrs (not ideal)
    list: Vec<Box<dyn Hitable>>,
}

impl HitableList {
    pub fn new(list: Vec<Box<dyn Hitable>>) -> HitableList {
        HitableList { list }
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
    fn hit_ptr(&self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::zero();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        // Could this escape loop early? and/or be a map/reduce?
        for hitable in self.list.iter() {
            if hitable.hit_ptr(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
            }
        }

        *hit_record = temp_rec;

        hit_anything
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABBVolume> {
        if !self.list.is_empty() {
            let mut result = match self.list[0].bounding_box(t0, t1) {
                Some(bounding_box) => bounding_box,
                None => return None,
            };

            for hitable in &self.list[1..] {
                match hitable.bounding_box(t0, t1) {
                    Some(bounding_box) => result = surrounding_box(result, bounding_box),
                    None => return None,
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
