use vec3::Vec3;
use ray::Ray;
use material::Material;
use std::sync::Arc;

//#[derive(Debug)]
#[derive(Clone)]
pub struct HitRecord<'mat> {
    // FIXME: Improve naming
    pub t: f32,
    // FIXME: Improve naming
    pub p: Vec3,
    pub normal: Vec3,
    // What if this was a MaterialIndex? like what I'm planning on doing with SphereIndex/PrimativeIndex?
    pub material: &'mat Material
}

impl <'mat> HitRecord<'mat> {
    pub fn new(t: f32, p: Vec3, normal: Vec3, material: &'mat Material) -> HitRecord {
        HitRecord{ t, p, normal, material }
    }
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

// TODO: Remove generics and create push methods to box and add Hitable to internal list
// TODO: This will likely require changing internal list to Vec<Box<dyn Hitable>> or do something fancy with an untyped arena/allocator
pub struct HitableList {
    // TODO: Do something smart with a Arena or map of typeid, Vec<SomeHitableType>
    hitable_list: Vec<Box<dyn Hitable>>
}

impl HitableList {
    pub fn new(list: Vec<Box<dyn Hitable>>) -> HitableList {
        HitableList{ hitable_list: list }
    }
}

unsafe impl Send for HitableList {}
unsafe impl Sync for HitableList {}

impl Hitable for HitableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut temp_rec = None;
        let mut closest_so_far = t_max;

        // Could this escape loop early? and/or be a map/reduce?
        for hitable in self.hitable_list.iter() {
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
}

//pub struct HitableList<T: Hitable> {
//    hitable_list: Vec<T>
//}

//impl <T: Hitable> HitableList<T> {
//    pub fn new(list: Vec<T>) -> HitableList<T> {
//        HitableList{ hitable_list: list }
//    }
//}

//impl <T: Hitable> Hitable for HitableList<T> {
//    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
//        let mut temp_rec = None;
//        let mut closest_so_far = t_max;
//
//        // Could this escape loop early? and/or be a map/reduce?
//        for hitable in self.hitable_list.iter() {
////            if let Some(record) = hitable.hit(ray, t_min, closest_so_far) {
////                closest_so_far = record.t;
////                temp_rec = Some(record);
////            }
//
//            let record = hitable.hit(ray, t_min, closest_so_far);
//            if record.is_some() {
//                closest_so_far = record.unwrap().t;
//                temp_rec = record;
//            }
//        }
//
//        temp_rec
//    }
//}
