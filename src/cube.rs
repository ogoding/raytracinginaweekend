use vec3::Vec3;
use ray::Ray;
use hitable::{Hitable, HitRecord, HitableList};
use aarect::{XYRect, XZRect, YZRect};
use transform::FlipNormals;
use material::MaterialIndex;
use aabb::AABBVolume;

pub struct Cube {
    list: HitableList,
//    top: FlipNormals<XZRect>,
//    bottom: XZRect,
//    front: XYRect,
//    back: FlipNormals<XYRect>,
//    left: FlipNormals<YZRect>,
//    right: YZRect
}

impl Cube {
    pub fn new(pmin: Vec3, pmax: Vec3, mat: MaterialIndex) -> Cube {
        let front = XYRect::new(pmin.x(), pmax.x(), pmin.y(), pmax.y(), pmax.z(), mat);
        let back = FlipNormals::new(XYRect::new(pmin.x(), pmax.x(), pmin.y(), pmax.y(), pmin.z(), mat));
        let bottom = XZRect::new(pmin.x(), pmax.x(), pmin.z(), pmax.z(), pmax.y(), mat);
        let top = FlipNormals::new(XZRect::new(pmin.x(), pmax.x(), pmin.z(), pmax.z(), pmin.y(), mat));
        let right = YZRect::new(pmin.y(), pmax.y(), pmin.z(), pmax.z(), pmax.x(), mat);
        let left = FlipNormals::new(YZRect::new(pmin.y(), pmax.y(), pmin.z(), pmax.z(), pmin.x(), mat));

//        Cube{ top, bottom, front, back, left, right }
        Cube{ list: HitableList::new(vec![Box::new(front), Box::new(back), Box::new(bottom), Box::new(top), Box::new(right), Box::new(left)]) }
    }

    pub fn new_boxed(pmin: Vec3, pmax: Vec3, mat: MaterialIndex) -> Box<Cube> {
        Box::new(Cube::new(pmin, pmax, mat))
    }
}

impl Hitable for Cube {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.list.hit(ray, t_min, t_max)
//        let mut hit = self.top.hit(ray, t_min, t_max);
//        if hit.is_some() { return hit; }
//        hit = self.bottom.hit(ray, t_min, t_max);
//        if hit.is_some() { return hit; }
//        hit = self.front.hit(ray, t_min, t_max);
//        if hit.is_some() { return hit; }
//        hit =  self.back.hit(ray, t_min, t_max);
//        if hit.is_some() { return hit; }
//        hit = self.left.hit(ray, t_min, t_max);
//        if hit.is_some() { return hit; }
//
//        return self.right.hit(ray, t_min, t_max);
    }

    fn bounding_box(&self, t_min: f32, t_max: f32) -> Option<AABBVolume> {
        self.list.bounding_box(t_min, t_max)
//        let mut bbox = self.top.bounding_box(t_min, t_max).unwrap();
//        bbox = surrounding_box(bbox, self.bottom.bounding_box(t_min, t_max).unwrap());
//        bbox = surrounding_box(bbox, self.left.bounding_box(t_min, t_max).unwrap());
//        bbox = surrounding_box(bbox, self.right.bounding_box(t_min, t_max).unwrap());
//        bbox = surrounding_box(bbox, self.front.bounding_box(t_min, t_max).unwrap());
//        bbox = surrounding_box(bbox, self.back.bounding_box(t_min, t_max).unwrap());
//        Some(bbox)
    }
}
