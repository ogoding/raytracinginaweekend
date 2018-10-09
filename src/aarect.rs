use vec3::Vec3;
use ray::Ray;
use hitable::{Hitable, HitRecord};
//use aabb::AABBVolume;
use material::Material;

pub struct XYRect {
    x0: f32,
    x1: f32,
    y0: f32,
    y1: f32,
    k: f32,
    // TODO: Make this a ref (and store materials somewhere more efficient than random heap objects)?
//    material: &'mat Material
    material: Box<Material>
}

impl XYRect {
    pub fn new(x0: f32, x1: f32, y0: f32, y1: f32, k: f32, material: Box<Material>) -> XYRect {
        XYRect{ x0, x1, y0, y1, k, material }
    }

    pub fn new_boxed(x0: f32, x1: f32, y0: f32, y1: f32, k: f32, material: Box<Material>) -> Box<XYRect> {
        Box::new(XYRect::new(x0, x1, y0, y1, k, material))
    }
}

impl Hitable for XYRect {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - ray.origin().z()) / ray.direction().z();
        if t < t_min || t > t_max {
            return None;
        }

        let x = t.mul_add(ray.direction().x(), ray.origin().x());
        let y = t.mul_add(ray.direction().y(), ray.origin().y());
//        let x = ray.origin().x() + t * ray.direction().x();
//        let y = ray.origin().y() + t * ray.direction().y();
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (y - self.y0) / (self.y1 - self.y0);

        Some(HitRecord::new(t, ray.point_at_parameter(t), u, v, Vec3::new(0.0, 0.0, 1.0), self.material.as_ref()))
    }

//    fn bounding_box(&self, t0: f32, t1: f32, hit_record: &HitRecord) -> Option<AABBVolume> {
//        Some(AABBVolume::new(Vec3::new(self.x0, self.y0, self.k - 0.0001), Vec3::new(self.x1, self.y1, self.k + 0.0001)))
//    }
}

pub struct XZRect {
    x0: f32,
    x1: f32,
    z0: f32,
    z1: f32,
    k: f32,
    // TODO: Make this a ref (and store materials somewhere more efficient than random heap objects)?
//    material: &'mat Material
    material: Box<Material>
}

impl XZRect {
    pub fn new(x0: f32, x1: f32, z0: f32, z1: f32, k: f32, material: Box<Material>) -> XZRect {
        XZRect{ x0, x1, z0, z1, k, material }
    }

    pub fn new_boxed(x0: f32, x1: f32, z0: f32, z1: f32, k: f32, material: Box<Material>) -> Box<XZRect> {
        Box::new(XZRect::new(x0, x1, z0, z1, k, material))
    }
}

impl Hitable for XZRect {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - ray.origin().y()) / ray.direction().y();
        if t < t_min || t > t_max { return None; }

        let x = t.mul_add(ray.direction().x(), ray.origin().x());
        let z = t.mul_add(ray.direction().z(), ray.origin().z());
//        let x = ray.origin().x() + t * ray.direction().x();
//        let z = ray.origin().z() + t * ray.direction().z();
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 { return None; }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (z - self.z0) / (self.z1 - self.z0);

        Some(HitRecord::new(t, ray.point_at_parameter(t), u, v, Vec3::new(0.0, 1.0, 0.0), self.material.as_ref()))
    }

//    fn bounding_box(&self, t0: f32, t1: f32, hit_record: &HitRecord) -> Option<AABBVolume> {
//        Some(AABBVolume::new(Vec3::new(self.x0, self.k - 0.0001, self.z0), Vec3::new(self.x1, self.k + 0.0001, self.z1)))
//    }
}

pub struct YZRect {
    y0: f32,
    y1: f32,
    z0: f32,
    z1: f32,
    k: f32,
    // TODO: Make this a ref (and store materials somewhere more efficient than random heap objects)?
//    material: &'mat Material
    material: Box<Material>
}

impl YZRect {
    pub fn new(y0: f32, y1: f32, z0: f32, z1: f32, k: f32, material: Box<Material>) -> YZRect {
        YZRect{ y0, y1, z0, z1, k, material }
    }

    pub fn new_boxed(y0: f32, y1: f32, z0: f32, z1: f32, k: f32, material: Box<Material>) -> Box<YZRect> {
        Box::new(YZRect::new(y0, y1, z0, z1, k, material))
    }
}

impl Hitable for YZRect {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - ray.origin().x()) / ray.direction().x();
        if t < t_min || t > t_max { return None; }

        let y = t.mul_add(ray.direction().y(), ray.origin().y());
        let z = t.mul_add(ray.direction().z(), ray.origin().z());
//        let y = ray.origin().y() + t * ray.direction().y();
//        let z = ray.origin().z() + t * ray.direction().z();
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 { return None; }

        let u = (y - self.y0) / (self.y1 - self.y0);
        let v = (z - self.z0) / (self.z1 - self.z0);

        Some(HitRecord::new(t, ray.point_at_parameter(t), u, v, Vec3::new(1.0, 0.0, 0.0), self.material.as_ref()))
    }

//    fn bounding_box(&self, t0: f32, t1: f32, hit_record: &HitRecord) -> Option<AABBVolume> {
//        Some(AABBVolume::new(Vec3::new(self.k - 0.0001, self.y0, self.z0), Vec3::new(self.k + 0.0001, self.y1, self.z1)))
//    }
}