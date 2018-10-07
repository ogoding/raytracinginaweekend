use vec3::Vec3;
use ray::Ray;
use material::Material;
use hitable::{Hitable, HitRecord};
use std::f32::consts::{PI, FRAC_PI_2};

fn get_sphere_uv(p: &Vec3) -> (f32, f32) {
    let phi = p.z().atan2(p.x());
    let theta = p.y().asin();
    (1.0 - (phi + PI) / (2.0 * PI), (theta + FRAC_PI_2) / PI)
}

pub struct Sphere {
    center: Vec3,
    radius: f32,
    // What if this was a MaterialIndex? like what I'm planning on doing with SphereIndex/PrimativeIndex?
    // TODO: Make this a ref (and store materials somewhere more efficient than random heap objects)?
    material: Box<Material>
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Box<Material>) -> Sphere {
        Sphere{ center, radius, material }
    }

    pub fn new_boxed(center: Vec3, radius: f32, material: Box<Material>) -> Box<Sphere> {
        Box::new(Sphere::new(center, radius, material))
    }

    #[inline(always)]
    pub fn create_hit_record(&self, ray: &Ray, t: f32) -> HitRecord {
        let p = ray.point_at_parameter(t);
        let (u, v) = get_sphere_uv(&((p - self.center) / self.radius));
        HitRecord::new(t, p, u, v, (p - self.center) / self.radius, self.material.as_ref())
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let ray_direction = ray.direction();
        let a = ray_direction.squared_length();
        let b = Vec3::dot(&oc, &ray_direction);
        let c = oc.squared_length() - self.radius.powi(2);
        let discriminant = b.powi(2) - a * c;

        if discriminant > 0.0 {
            let discrim_sqrt = discriminant.sqrt();
            let mut temp = (-b - discrim_sqrt) / a;
            if t_max > temp && temp > t_min {
                return Some(self.create_hit_record(ray, temp));
            }

            temp = (-b + discrim_sqrt) / a;
            if t_max > temp && temp > t_min {
                return Some(self.create_hit_record(ray, temp));
            }
        }

        None
    }
}

pub struct MovingSphere {
    center0: Vec3,
    center1: Vec3,
    radius: f32,
    // TODO: Make this a ref (and store materials somewhere more efficient than random heap objects)?
    material: Box<Material>,
    time0: f32,
    time1: f32
}

impl MovingSphere {
    pub fn new(center0: Vec3, center1: Vec3, t0: f32, t1: f32, radius: f32, material: Box<Material>) -> MovingSphere {
        MovingSphere{ center0, center1, radius, material, time0: t0, time1: t1 }
    }

    pub fn new_boxed(center0: Vec3, center1: Vec3, t0: f32, t1: f32, radius: f32, material: Box<Material>) -> Box<MovingSphere> {
        Box::new(MovingSphere::new(center0, center1, t0, t1, radius, material))
    }

    #[inline(always)]
    pub fn create_hit_record(&self, ray: &Ray, t: f32) -> HitRecord {
        let p = ray.point_at_parameter(t);
        // FIXME: Work out whether uv coords are needed for movingsphere
        let (u, v) = get_sphere_uv(&((p - self.center(t)) / self.radius));
//        let (u, v) = (0.0, 0.0);
        HitRecord::new(t, p, u, v, (p - self.center(ray.time())) / self.radius, self.material.as_ref())
    }

    #[inline]
    fn center(&self, time: f32) -> Vec3 {
        self.center0 + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl Hitable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin() - self.center(ray.time());
        let ray_direction = ray.direction();
        let a = ray_direction.squared_length();
        let b = Vec3::dot(&oc, &ray_direction);
        let c = oc.squared_length() - self.radius.powi(2);
        let discriminant = b.powi(2) - a * c;

        if discriminant > 0.0 {
            let discrim_sqrt = discriminant.sqrt();
            let mut temp = (-b - discrim_sqrt) / a;
            if t_max > temp && temp > t_min {
                return Some(self.create_hit_record(ray, temp));
            }

            temp = (-b + discrim_sqrt) / a;
            if t_max > temp && temp > t_min {
                return Some(self.create_hit_record(ray, temp));
            }
        }

        None
    }
}
