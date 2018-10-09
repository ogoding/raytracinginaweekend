use vec3::Vec3;
use ray::Ray;
use hitable::{Hitable, HitRecord, HitableList, FlipNormals};
use aarect::{XYRect, XZRect, YZRect};
use material::Material;

pub struct Cube {
    pmin: Vec3,
    pmax: Vec3,
    top: Box<Hitable>,
    bottom: Box<Hitable>,
    front: Box<Hitable>,
    back: Box<Hitable>,
    left: Box<Hitable>,
    right: Box<Hitable>
}

impl Cube {

    // FIXME: Compiler suggests using &'static (dyn Material + 'static) but then fails saying that materials[0].as_ref() doesn't live long enough
    // FIXME: Work out what the hell &'static (dyn Material + 'static) means and how to fix all of this
//    pub fn new(pmin: Vec3, pmax: Vec3, mat: &'static (dyn Material + 'static)) -> Cube {
    pub fn new<M: Material + 'static>(pmin: Vec3, pmax: Vec3, mat: Box<M>) -> Cube {
//        let mut list: Vec<Box<Hitable>> = Vec::new();
        let front = XYRect::new_boxed(pmin.x(), pmax.x(), pmin.y(), pmax.y(), pmax.z(), mat);
        let back = FlipNormals::new_boxed(XYRect::new(pmin.x(), pmax.x(), pmin.y(), pmax.y(), pmin.z(), mat));
        let bottom = XZRect::new_boxed(pmin.x(), pmax.x(), pmin.z(), pmax.z(), pmax.y(), mat);
        let top = FlipNormals::new_boxed(XZRect::new(pmin.x(), pmax.x(), pmin.z(), pmax.z(), pmin.y(), mat));
        let right = YZRect::new_boxed(pmin.y(), pmax.y(), pmin.z(), pmax.z(), pmax.x(), mat);
        let left = FlipNormals::new_boxed(YZRect::new(pmin.y(), pmax.y(), pmin.z(), pmax.z(), pmin.x(), mat));

//        FlipNormals::new_boxed(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, materials[1].as_ref())),        // Left plane
//        YZRect::new_boxed(0.0, 555.0, 0.0, 555.0, 0.0, materials[0].as_ref()),                                   // Right plane
//        XZRect::new_boxed(213.0, 343.0, 227.0, 322.0, 554.0, materials[3].as_ref()),                           // Top light
//        XZRect::new_boxed(0.0, 555.0, 0.0, 555.0, 0.0, materials[2].as_ref()),                                 // Bottom plane
//        FlipNormals::new_boxed(XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, materials[2].as_ref())),       // Top plane
//        FlipNormals::new_boxed(XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, materials[2].as_ref())),       // Back plane
        Cube{ pmin, pmax, top, bottom, front, back, left, right }
    }

//    pub fn new_boxed(pmin: Vec3, pmax: Vec3, mat: &'static (dyn Material + 'static)) -> Box<Cube> {
    pub fn new_boxed<M: Material + 'static>(pmin: Vec3, pmax: Vec3, mat: Box<M>) -> Box<Cube> {
        Box::new(Cube::new(pmin, pmax, mat))
    }
}

impl Hitable for Cube {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
//        self.list.hit(ray, t_min, t_max)
        let mut hit = self.top.hit(ray, t_min, t_max);
        if hit.is_some() { return hit; }
        hit = self.bottom.hit(ray, t_min, t_max);
        if hit.is_some() { return hit; }
        hit = self.front.hit(ray, t_min, t_max);
        if hit.is_some() { return hit; }
        hit =  self.back.hit(ray, t_min, t_max);
        if hit.is_some() { return hit; }
        hit = self.left.hit(ray, t_min, t_max);
        if hit.is_some() { return hit; }

        return self.right.hit(ray, t_min, t_max);
    }
}


//pub struct Cube {
//    pmin: Vec3,
//    pmax: Vec3,
//    // TODO: Could split this up into fields and avoid lifetime problems
//    list: HitableList
//}
//
//impl Cube {
//
//    // FIXME: Compiler suggests using &'static (dyn Material + 'static) but then fails saying that materials[0].as_ref() doesn't live long enough
//    // FIXME: Work out what the hell &'static (dyn Material + 'static) means and how to fix all of this
//    pub fn new(pmin: Vec3, pmax: Vec3, mat: &'static (dyn Material + 'static)) -> Cube {
////    pub fn new<M: Material + 'static>(pmin: Vec3, pmax: Vec3, mat: &'static M) -> Cube {
//        let mut list: Vec<Box<Hitable>> = Vec::new();
//        list.push(XYRect::new_boxed(pmin.x(), pmax.x(), pmin.y(), pmax.y(), pmax.z(), mat));
//        list.push(FlipNormals::new_boxed(XYRect::new(pmin.x(), pmax.x(), pmin.y(), pmax.y(), pmin.z(), mat)));
//        list.push(XZRect::new_boxed(pmin.x(), pmax.x(), pmin.z(), pmax.z(), pmax.y(), mat));
//        list.push(FlipNormals::new_boxed(XZRect::new(pmin.x(), pmax.x(), pmin.z(), pmax.z(), pmin.y(), mat)));
//        list.push(YZRect::new_boxed(pmin.y(), pmax.y(), pmin.z(), pmax.z(), pmax.x(), mat));
//        list.push(FlipNormals::new_boxed(YZRect::new(pmin.y(), pmax.y(), pmin.z(), pmax.z(), pmin.x(), mat)));
////        let list = HitableList::new(vec![
////            XYRect::new_boxed(pmin.x(), pmax.x(), pmin.y(), pmax.y(), pmax.z(), mat),
////            FlipNormals::new_boxed(XYRect::new(pmin.x(), pmax.x(), pmin.y(), pmax.y(), pmin.z(), mat)),
////            XZRect::new_boxed(pmin.x(), pmax.x(), pmin.z(), pmax.z(), pmax.y(), mat),
////            FlipNormals::new_boxed(XZRect::new(pmin.x(), pmax.x(), pmin.z(), pmax.z(), pmin.y(), mat)),
////            YZRect::new_boxed(pmin.y(), pmax.y(), pmin.z(), pmax.z(), pmax.x(), mat),
////            FlipNormals::new_boxed(YZRect::new(pmin.y(), pmax.y(), pmin.z(), pmax.z(), pmin.x(), mat))
////        ]);
//
//        Cube{ pmin, pmax, list: HitableList::new(list) }
//    }
//
//    pub fn new_boxed(pmin: Vec3, pmax: Vec3, mat: &'static (dyn Material + 'static)) -> Box<Cube> {
////    pub fn new_boxed<M: Material + 'static>(pmin: Vec3, pmax: Vec3, mat: &'static M) -> Box<Cube> {
//        Box::new(Cube::new(pmin, pmax, mat))
//    }
//}
//
//impl Hitable for Cube {
//    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
//        self.list.hit(ray, t_min, t_max)
//    }
//}
