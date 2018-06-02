use vec3::Vec3;
use ray::Ray;
use hitable::HitRecord;
use super::rand;


// TODO Implement Material as a trait
//pub trait Material {
//    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, attenuation: &Vec3, scattered: &Ray) -> bool;
//}

fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = 2.0 * Vec3::new(rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>()) - Vec3::uniform(1.0);

        if p.squared_length() >= 1.0 {
            break;
        }
    }

    Vec3::zero()
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
    *attenuation = Vec3::new(1.0, 1.0, 1.0);

    let mut outward_normal = Vec3::zero();
    let mut ni_over_nt = 0.0;
    let mut cosine = 0.0;

    if Vec3::dot(&ray_in.direction(), &hit_record.normal) > 0.0 {
        outward_normal = -1.0 * hit_record.normal;
        ni_over_nt = ref_idx;
        cosine = ref_idx * Vec3::dot(&ray_in.direction(), &hit_record.normal) / ray_in.direction().length();
    } else {
        outward_normal = hit_record.normal;
        ni_over_nt = 1.0 / ref_idx;
        cosine = -Vec3::dot(&ray_in.direction(), &hit_record.normal) / ray_in.direction().length();
    }


    let mut reflect_prob = 0.0;
    let mut refracted = Vec3::zero();
    let reflected = reflect(&ray_in.direction(), &hit_record.normal);

    if let Some(refracted_ray) = refract(&ray_in.direction(), &outward_normal, ni_over_nt) {
        refracted = refracted_ray;
        reflect_prob = schlick(cosine, ref_idx);
    } else {
        reflect_prob = 1.0
    }

    if rand::random::<f32>() < reflect_prob {
        *scattered = Ray::new(hit_record.p, reflected);
    } else {
        *scattered = Ray::new(hit_record.p, refracted);
    }

    true
}

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