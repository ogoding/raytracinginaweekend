use vec3::Vec3;
use ray::Ray;

#[derive(Debug, Copy, Clone)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3
}

impl HitRecord {
    pub fn zero() -> HitRecord {
        HitRecord::new(0.0, Vec3::zero(), Vec3::zero())
    }

    pub fn new(t: f32, p: Vec3, normal: Vec3) -> HitRecord {
        HitRecord{ t, p, normal }
    }
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool;
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
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::zero();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for hitable in self.hitable_list.iter() {
            if hitable.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
            }
        }

        *record = temp_rec;

        hit_anything
    }
}