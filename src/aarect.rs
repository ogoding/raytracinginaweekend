use aabb::AABBVolume;
use hitable::{HitRecord, Hitable};
use material::MaterialIndex;
use ray::Ray;
use vec3::Vec3;

pub struct XYRect {
    x0: f32,
    x1: f32,
    y0: f32,
    y1: f32,
    k: f32,
    material: MaterialIndex,
}

impl XYRect {
    pub fn new(x0: f32, x1: f32, y0: f32, y1: f32, k: f32, material: MaterialIndex) -> XYRect {
        XYRect {
            x0,
            x1,
            y0,
            y1,
            k,
            material,
        }
    }

    pub fn new_boxed(
        x0: f32,
        x1: f32,
        y0: f32,
        y1: f32,
        k: f32,
        material: MaterialIndex,
    ) -> Box<XYRect> {
        Box::new(XYRect::new(x0, x1, y0, y1, k, material))
    }
}

impl Hitable for XYRect {
    fn hit_ptr(&self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let t = (self.k - ray.origin().z()) / ray.direction().z();
        if t < t_min || t > t_max {
            return false;
        }

        let x = t.mul_add(ray.direction().x(), ray.origin().x());
        let y = t.mul_add(ray.direction().y(), ray.origin().y());
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return false;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (y - self.y0) / (self.y1 - self.y0);

        hit_record.t = t;
        hit_record.p = ray.point_at_parameter(t);
        hit_record.u = u;
        hit_record.v = v;
        hit_record.normal = Vec3::new(0.0, 0.0, 1.0);
        hit_record.material = self.material;
        true
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABBVolume> {
        Some(AABBVolume::new(
            Vec3::new(self.x0, self.y0, self.k - 0.0001),
            Vec3::new(self.x1, self.y1, self.k + 0.0001),
        ))
    }
}

pub struct XZRect {
    x0: f32,
    x1: f32,
    z0: f32,
    z1: f32,
    k: f32,
    material: MaterialIndex,
}

impl XZRect {
    pub fn new(x0: f32, x1: f32, z0: f32, z1: f32, k: f32, material: MaterialIndex) -> XZRect {
        XZRect {
            x0,
            x1,
            z0,
            z1,
            k,
            material,
        }
    }

    pub fn new_boxed(
        x0: f32,
        x1: f32,
        z0: f32,
        z1: f32,
        k: f32,
        material: MaterialIndex,
    ) -> Box<XZRect> {
        Box::new(XZRect::new(x0, x1, z0, z1, k, material))
    }
}

impl Hitable for XZRect {
    fn hit_ptr(&self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let t = (self.k - ray.origin().y()) / ray.direction().y();
        if t < t_min || t > t_max {
            return false;
        }

        let x = t.mul_add(ray.direction().x(), ray.origin().x());
        let z = t.mul_add(ray.direction().z(), ray.origin().z());
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return false;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (z - self.z0) / (self.z1 - self.z0);

        hit_record.t = t;
        hit_record.p = ray.point_at_parameter(t);
        hit_record.u = u;
        hit_record.v = v;
        hit_record.normal = Vec3::new(0.0, 1.0, 0.0);
        hit_record.material = self.material;
        true
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABBVolume> {
        Some(AABBVolume::new(
            Vec3::new(self.x0, self.k - 0.0001, self.z0),
            Vec3::new(self.x1, self.k + 0.0001, self.z1),
        ))
    }
}

pub struct YZRect {
    y0: f32,
    y1: f32,
    z0: f32,
    z1: f32,
    k: f32,
    material: MaterialIndex,
}

impl YZRect {
    pub fn new(y0: f32, y1: f32, z0: f32, z1: f32, k: f32, material: MaterialIndex) -> YZRect {
        YZRect {
            y0,
            y1,
            z0,
            z1,
            k,
            material,
        }
    }

    pub fn new_boxed(
        y0: f32,
        y1: f32,
        z0: f32,
        z1: f32,
        k: f32,
        material: MaterialIndex,
    ) -> Box<YZRect> {
        Box::new(YZRect::new(y0, y1, z0, z1, k, material))
    }
}

impl Hitable for YZRect {
    fn hit_ptr(&self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let t = (self.k - ray.origin().x()) / ray.direction().x();
        if t < t_min || t > t_max {
            return false;
        }

        let y = t.mul_add(ray.direction().y(), ray.origin().y());
        let z = t.mul_add(ray.direction().z(), ray.origin().z());
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return false;
        }

        let u = (y - self.y0) / (self.y1 - self.y0);
        let v = (z - self.z0) / (self.z1 - self.z0);

        hit_record.t = t;
        hit_record.p = ray.point_at_parameter(t);
        hit_record.u = u;
        hit_record.v = v;
        hit_record.normal = Vec3::new(1.0, 0.0, 0.0);
        hit_record.material = self.material;
        true
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABBVolume> {
        Some(AABBVolume::new(
            Vec3::new(self.k - 0.0001, self.y0, self.z0),
            Vec3::new(self.k + 0.0001, self.y1, self.z1),
        ))
    }
}
