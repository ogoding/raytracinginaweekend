use aabb::{surrounding_box, AABBVolume};
use aarect::{XYRect, XZRect, YZRect};
use hitable::{HitRecord, Hitable};
use scene::{Entities, MaterialRef};
use ray::Ray;
use transform::FlipNormals;
use vec3::Vec3;

#[derive(Debug)]
pub struct Cube {
    front: XYRect,
    back: FlipNormals<XYRect>,
    top: XZRect,
    bottom: FlipNormals<XZRect>,
    left: YZRect,
    right: FlipNormals<YZRect>,
}

impl Cube {
    pub fn new(pmin: Vec3, pmax: Vec3, mat: MaterialRef) -> Cube {
        let front = XYRect::new(pmin.x(), pmax.x(), pmin.y(), pmax.y(), pmax.z(), mat);
        let back = FlipNormals::new(XYRect::new(
            pmin.x(),
            pmax.x(),
            pmin.y(),
            pmax.y(),
            pmin.z(),
            mat,
        ));
        let top = XZRect::new(pmin.x(), pmax.x(), pmin.z(), pmax.z(), pmax.y(), mat);
        let bottom = FlipNormals::new(XZRect::new(
            pmin.x(),
            pmax.x(),
            pmin.z(),
            pmax.z(),
            pmin.y(),
            mat,
        ));
        let left = YZRect::new(pmin.y(), pmax.y(), pmin.z(), pmax.z(), pmax.x(), mat);
        let right = FlipNormals::new(YZRect::new(
            pmin.y(),
            pmax.y(),
            pmin.z(),
            pmax.z(),
            pmin.x(),
            mat,
        ));

        Cube {
            front,
            back,
            top,
            bottom,
            left,
            right,
        }
    }
}

impl Hitable for Cube {
    fn hit_ptr(&self, entities: &Entities, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::zero();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        // TODO: Make a macro or fn to clean this up a bit
        if self
            .front
            .hit_ptr(entities, ray, t_min, closest_so_far, &mut temp_rec)
        {
            hit_anything = true;
            closest_so_far = temp_rec.t;
            *hit_record = temp_rec;
        }
        if self.back.hit_ptr(entities, ray, t_min, closest_so_far, &mut temp_rec) {
            hit_anything = true;
            closest_so_far = temp_rec.t;
            *hit_record = temp_rec;
        }
        if self.top.hit_ptr(entities, ray, t_min, closest_so_far, &mut temp_rec) {
            hit_anything = true;
            closest_so_far = temp_rec.t;
            *hit_record = temp_rec;
        }
        if self
            .bottom
            .hit_ptr(entities, ray, t_min, closest_so_far, &mut temp_rec)
        {
            hit_anything = true;
            closest_so_far = temp_rec.t;
            *hit_record = temp_rec;
        }
        if self.left.hit_ptr(entities, ray, t_min, closest_so_far, &mut temp_rec) {
            hit_anything = true;
            closest_so_far = temp_rec.t;
            *hit_record = temp_rec;
        }
        if self
            .right
            .hit_ptr(entities, ray, t_min, closest_so_far, &mut temp_rec)
        {
            hit_anything = true;
            //            closest_so_far = temp_rec.t;
            *hit_record = temp_rec;
        }

        hit_anything
    }

    fn bounding_box(&self, t_min: f32, t_max: f32) -> Option<AABBVolume> {
        let mut bbox = self.front.bounding_box(t_min, t_max).unwrap();
        bbox = surrounding_box(bbox, self.back.bounding_box(t_min, t_max).unwrap());
        bbox = surrounding_box(bbox, self.top.bounding_box(t_min, t_max).unwrap());
        bbox = surrounding_box(bbox, self.bottom.bounding_box(t_min, t_max).unwrap());
        bbox = surrounding_box(bbox, self.left.bounding_box(t_min, t_max).unwrap());
        bbox = surrounding_box(bbox, self.right.bounding_box(t_min, t_max).unwrap());
        Some(bbox)
    }
}
