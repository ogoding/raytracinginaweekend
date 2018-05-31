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

fn lambert_scatter(albedo: &Vec3, _ray: &Ray, hit_record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
    let target = hit_record.p + hit_record.normal + random_in_unit_sphere();

    *scattered = Ray::new(hit_record.p, target - hit_record.p);
    *attenuation = *albedo;
    return true;
}

fn metal_scatter(albedo: &Vec3, ray: &Ray, hit_record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray, fuzz: f32) -> bool {
    let reflected = reflect(&ray.direction().unit(), &hit_record.normal);

    let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
    *scattered = Ray::new(hit_record.p, reflected + fuzz * random_in_unit_sphere());
    *attenuation = *albedo;

    Vec3::dot(&scattered.direction(), &hit_record.normal) > 0.0
}

#[derive(Debug, Copy, Clone)]
pub enum Material {
    Lambertian(Vec3),
    Metal(Vec3, f32)
}

impl Material {
    // TODO make helper/constructor methods
    pub fn scatter(&self, ray: &Ray, hit_record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        match self {
            Material::Lambertian(albedo) => lambert_scatter(&albedo, ray, hit_record, attenuation, scattered),
            Material::Metal(albedo, fuzz) => metal_scatter(&albedo, ray, hit_record, attenuation, scattered, *fuzz)
        }
    }
}