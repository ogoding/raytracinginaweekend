use vec3::Vec3;
use ray::Ray;
use material::Material;
use aabb::{AABBVolume, surrounding_box};

//#[derive(Debug)]
#[derive(Clone)]
pub struct HitRecord<'mat> {
    // FIXME: Improve naming
    pub t: f32,
    // FIXME: Improve naming
    pub p: Vec3,
    pub u: f32,
    pub v: f32,
    pub normal: Vec3,
    // What if this was a MaterialIndex? like what I'm planning on doing with SphereIndex/PrimativeIndex?
    pub material: &'mat Material
}

impl <'mat> HitRecord<'mat> {
    pub fn new(t: f32, p: Vec3, u: f32, v: f32, normal: Vec3, material: &'mat Material) -> HitRecord {
        HitRecord{ t, p, u, v, normal, material }
    }
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, t_min: f32, t_max: f32) -> Option<AABBVolume>;
}

// TODO: Remove generics and create push methods to box and add Hitable to internal list
// TODO: This will likely require changing internal list to Vec<Box<dyn Hitable>> or do something fancy with an untyped arena/allocator
pub struct HitableList {
    // TODO: Do something smart with a Arena or map of typeid, Vec<SomeHitableType>
    list: Vec<Box<dyn Hitable>>
}

impl HitableList {
    pub fn new(list: Vec<Box<dyn Hitable>>) -> HitableList {
        HitableList{ list: list }
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
//            for i in 1..self.list.len() {
//                if self.list[i].bounding_box(t0, t1) {
//
//                } else {
//                    return None;
//                }
//            }

            Some(result)
        } else {
            None
        }
    }
}

//pub struct HitableList<T: Hitable> {
//    list: Vec<T>
//}

//impl <T: Hitable> HitableList<T> {
//    pub fn new(list: Vec<T>) -> HitableList<T> {
//        HitableList{ list: list }
//    }
//}

//impl <T: Hitable> Hitable for HitableList<T> {
//    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
//        let mut temp_rec = None;
//        let mut closest_so_far = t_max;
//
//        // Could this escape loop early? and/or be a map/reduce?
//        for hitable in self.list.iter() {
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

// TODO: Move this into transform.rs?
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
        if let Some(hit) = self.ptr.hit(ray, t_min, t_max) {
            let mut hit = hit;
            hit.normal = -hit.normal;
            Some(hit)
        } else {
            None
        }
    }

    fn bounding_box(&self, t_min: f32, t_max: f32) -> Option<AABBVolume> {
        self.ptr.bounding_box(t_min, t_max)
    }
}