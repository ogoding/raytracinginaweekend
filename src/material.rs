use hitable::HitRecord;
use random::drand48;
use ray::Ray;
use texture::{TextureRef, Texture};
use vec3::Vec3;

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

fn lambert(ray: &Ray, hit_record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray, albedo: Vec3) -> bool {
    let target = hit_record.p + hit_record.normal + random_in_unit_sphere();

    *scattered = Ray::new(hit_record.p, target - hit_record.p, ray.time());
    *attenuation = albedo;
    true
}

fn metal(ray: &Ray, hit_record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray, albedo: &Vec3, fuzz: f32) -> bool {
    let reflected = reflect(&ray.direction().unit(), &hit_record.normal);

    let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
    *scattered = Ray::new(
        hit_record.p,
        reflected + fuzz * random_in_unit_sphere(),
        ray.time(),
    );
    *attenuation = *albedo;

    Vec3::dot(&scattered.direction(), &hit_record.normal) > 0.0
}

fn dieletric(ray: &Ray, hit_record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray, ref_idx: f32) -> bool {
    *attenuation = Vec3::uniform(1.0);

    let (outward_normal, ni_over_nt, cosine) =
        if Vec3::dot(&ray.direction(), &hit_record.normal) > 0.0 {
            (
                -hit_record.normal,
                ref_idx,
                ref_idx * Vec3::dot(&ray.direction(), &hit_record.normal)
                    / ray.direction().length(),
            )
        } else {
            (
                hit_record.normal,
                1.0 / ref_idx,
                -Vec3::dot(&ray.direction(), &hit_record.normal) / ray.direction().length(),
            )
        };

    let reflected = reflect(&ray.direction(), &hit_record.normal);
    let (reflect_prob, refracted) =
        if let Some(refracted_ray) = refract(&ray.direction(), &outward_normal, ni_over_nt) {
            (schlick(cosine, ref_idx), refracted_ray)
        } else {
            (1.0, Vec3::zero())
        };

    if drand48() < reflect_prob {
        *scattered = Ray::new(hit_record.p, reflected, ray.time());
    } else {
        *scattered = Ray::new(hit_record.p, refracted, ray.time());
    }

    true
}

fn isotropic(ray: &Ray, hit_record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray, albedo: Vec3) -> bool {
    *attenuation = albedo;
    *scattered = Ray::new(hit_record.p, random_in_unit_sphere(), ray.time());
    true
}

#[derive(Debug)]
pub enum Material {
    Lambertian(Vec3),
    LambertianTextured(TextureRef),
    Metal(Vec3, f32),
    Dieletric(f32),
    DiffuseLight(TextureRef),
    Isotropic(TextureRef)
}

impl Material {
    pub fn scatter(&self, textures: &[Texture], ray: &Ray, hit_record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        match self {
            Material::Lambertian(albedo) => lambert(ray, hit_record, attenuation, scattered, *albedo),
            Material::LambertianTextured(tex_ref) => {
                let albedo = textures[*tex_ref].value(textures, hit_record.u, hit_record.v, &hit_record.p);
                lambert(ray, hit_record, attenuation, scattered, albedo)
            },
            Material::Metal(albedo, fuzz) => metal(ray, hit_record, attenuation, scattered, albedo, *fuzz),
            Material::Dieletric(ref_idx) => dieletric(ray, hit_record, attenuation, scattered, *ref_idx),
            Material::DiffuseLight(_) => false,
            Material::Isotropic(tex_ref) => {
                let albedo = textures[*tex_ref].value(textures, hit_record.u, hit_record.v, &hit_record.p);
                isotropic(ray, hit_record, attenuation, scattered, albedo)
            }
        }
    }
    pub fn emitted(&self, textures: &[Texture], u: f32, v: f32, p: &Vec3) -> Vec3 {
        match self {
            Material::DiffuseLight(tex_ref) => textures[*tex_ref].value(textures, u, v, p),
            _ => Vec3::zero()
        }
    }
}