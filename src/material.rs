use vec3::Vec3;
use ray::Ray;
use hitable::HitRecord;
use random::drand48;
//use texture::Texture;

fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = 2.0 * Vec3::random() - Vec3::uniform(1.0);

        if p.squared_length() < 1.0 {
            return p;
        }
    }
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2.0 * Vec3::dot(v, n) * *n
}

fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let v_unit = v.unit();
    let dt = Vec3::dot(&v_unit, n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);

    if discriminant > 0.0 {
        Some(ni_over_nt * (v_unit - *n * dt) - *n * discriminant.sqrt())
    } else {
        None
    }
}

// Polynomial approximation of acute reflection by Christphe Schlick
fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
}

pub struct Lambertian {
    albedo: Vec3
}

impl Lambertian {
    // TODO: Make this return an Arc wrapped value to make code simpler
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian{ albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let target = hit_record.p + hit_record.normal + random_in_unit_sphere();

        *scattered = Ray::new(hit_record.p, target - hit_record.p, ray.time());
        *attenuation = self.albedo;
        true
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f32
}

impl Metal {
    // TODO: Make this return an Arc wrapped value to make code simpler
    pub fn new(albedo: Vec3, fuzz: f32) -> Metal {
        Metal{ albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let reflected = reflect(&ray.direction().unit(), &hit_record.normal);

        let fuzz = if self.fuzz < 1.0 { self.fuzz } else { 1.0 };
        *scattered = Ray::new(hit_record.p, reflected + fuzz * random_in_unit_sphere(), ray.time());
        *attenuation = self.albedo;

        Vec3::dot(&scattered.direction(), &hit_record.normal) > 0.0
    }
}

pub struct Dieletric {
    ref_idx: f32
}

impl Dieletric {
    // TODO: Make this return an Arc wrapped value to make code simpler
    pub fn new(ref_idx: f32) -> Dieletric {
        Dieletric{ ref_idx }
    }
}

impl Material for Dieletric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        *attenuation = Vec3::uniform(1.0);

        let (outward_normal, ni_over_nt, cosine) = if Vec3::dot(&ray.direction(), &hit_record.normal) > 0.0 {
            (-hit_record.normal,
             self.ref_idx,
             self.ref_idx * Vec3::dot(&ray.direction(), &hit_record.normal) / ray.direction().length())
        } else {
            (hit_record.normal,
             1.0 / self.ref_idx,
             -Vec3::dot(&ray.direction(), &hit_record.normal) / ray.direction().length())
        };

        let reflected = reflect(&ray.direction(), &hit_record.normal);
        let (reflect_prob, refracted) = if let Some(refracted_ray) = refract(&ray.direction(), &outward_normal, ni_over_nt) {
            (schlick(cosine, self.ref_idx), refracted_ray)
        } else { (1.0, Vec3::zero()) };

        if drand48() < reflect_prob {
            *scattered = Ray::new(hit_record.p, reflected, ray.time());
        } else {
            *scattered = Ray::new(hit_record.p, refracted, ray.time());
        }

        true
    }
}
