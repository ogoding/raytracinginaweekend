use vec3::Vec3;
use ray::Ray;
use hitable::HitRecord;
use random::drand48;


// TODO Implement Material as a trait
//pub trait Material {
//    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, attenuation: &Vec3, scattered: &Ray) -> bool;
//}

fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = 2.0 * Vec3::random() - Vec3::uniform(1.0);

        if p.squared_length() >= 1.0 {
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

fn lambert_scatter(albedo: &Vec3, _ray: &Ray, hit_record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
    let target = hit_record.p + hit_record.normal + random_in_unit_sphere();

    *scattered = Ray::new(hit_record.p, target - hit_record.p);
    *attenuation = *albedo;
    return true;
}

fn metal_scatter(albedo: &Vec3, fuzz: f32, ray: &Ray, hit_record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
    let reflected = reflect(&ray.direction().unit(), &hit_record.normal);

    let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
    *scattered = Ray::new(hit_record.p, reflected + fuzz * random_in_unit_sphere());
    *attenuation = *albedo;

    Vec3::dot(&scattered.direction(), &hit_record.normal) > 0.0
}

fn dieletric_scatter(ref_idx: f32, ray_in: &Ray, hit_record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
    *attenuation = Vec3::uniform(1.0);

    let (outward_normal, ni_over_nt, cosine) = if Vec3::dot(&ray_in.direction(), &hit_record.normal) > 0.0 {
        (-hit_record.normal,
            ref_idx,
            ref_idx * Vec3::dot(&ray_in.direction(), &hit_record.normal) / ray_in.direction().length())
    } else {
        (hit_record.normal,
            1.0 / ref_idx,
            -Vec3::dot(&ray_in.direction(), &hit_record.normal) / ray_in.direction().length())
    };

    let reflected = reflect(&ray_in.direction(), &hit_record.normal);
    let (reflect_prob, refracted) = if let Some(refracted_ray) = refract(&ray_in.direction(), &outward_normal, ni_over_nt) {
        (schlick(cosine, ref_idx), refracted_ray)
    } else { (1.0, Vec3::zero()) };

    if drand48() < reflect_prob {
        *scattered = Ray::new(hit_record.p, reflected);
    } else {
        *scattered = Ray::new(hit_record.p, refracted);
    }

    true
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum Material {
    Lambertian(Vec3),
    Metal(Vec3, f32),
    Dieletric(f32)
}

impl Material {
    // TODO make helper/constructor methods
    pub fn scatter(&self, ray: &Ray, hit_record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        match self {
            Material::Lambertian(albedo) => lambert_scatter(&albedo, ray, hit_record, attenuation, scattered),
            Material::Metal(albedo, fuzz) => metal_scatter(&albedo, *fuzz, ray, hit_record, attenuation, scattered),
            Material::Dieletric(ref_idx) => dieletric_scatter(*ref_idx, ray, hit_record, attenuation, scattered)
        }
    }
}