use vec3::Vec3;
use ray::Ray;
use material::Material;

#[derive(Debug, Copy, Clone)]
pub struct HitRecord {
    // FIXME: Improve naming
    pub t: f32,
    // FIXME: Improve naming
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Option<Material>
}

impl HitRecord {
    pub fn zero() -> HitRecord {
        HitRecord::new(0.0, Vec3::zero(), Vec3::zero(), None)
    }

    pub fn new(t: f32, p: Vec3, normal: Vec3, material: Option<Material>) -> HitRecord {
        HitRecord{ t, p, normal, material }
    }
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HitableList<T: Hitable> {
    hitable_list: Vec<T>
}

impl <T: Hitable> HitableList<T> {
    pub fn new(list: Vec<T>) -> HitableList<T> {
        HitableList{ hitable_list: list }
    }
}

impl <T: Hitable> Hitable for HitableList<T> {
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
                closest_so_far = record.unwrap().t;
                temp_rec = record;
            }
        }

        temp_rec
    }
}