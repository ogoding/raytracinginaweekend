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
    // TODO: Make this return the resultant hit_record?
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

        // Could this escape loop early? and/or be a map/reduce?
        for hitable in self.hitable_list.iter() {
            if hitable.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                // TODO: Change Hitable.hit to return the HitRecord result and assign here instead of at end of fn
                *record = temp_rec;
            }
        }

//        *record = temp_rec;

        hit_anything
    }
}