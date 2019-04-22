use hitable::HitRecord;
use random::drand48;
use ray::Ray;
use texture::Texture;
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

pub trait Material {
    fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
    fn emitted(&self, _u: f32, _v: f32, _p: &Vec3) -> Vec3 {
        Vec3::zero()
    }
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let target = hit_record.p + hit_record.normal + random_in_unit_sphere();

        *scattered = Ray::new(hit_record.p, target - hit_record.p, ray.time());
        *attenuation = self.albedo;
        true
    }
}

pub struct LambertianTextured<T: Texture> {
    albedo: T,
}

impl<T: Texture> LambertianTextured<T> {
    pub fn new(albedo: T) -> LambertianTextured<T> {
        LambertianTextured { albedo }
    }
}

impl<T: Texture> Material for LambertianTextured<T> {
    fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let target = hit_record.p + hit_record.normal + random_in_unit_sphere();

        *scattered = Ray::new(hit_record.p, target - hit_record.p, ray.time());
        *attenuation = self.albedo.value(hit_record.u, hit_record.v, &hit_record.p);
        true
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Metal {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(&ray.direction().unit(), &hit_record.normal);

        let fuzz = if self.fuzz < 1.0 { self.fuzz } else { 1.0 };
        *scattered = Ray::new(
            hit_record.p,
            reflected + fuzz * random_in_unit_sphere(),
            ray.time(),
        );
        *attenuation = self.albedo;

        Vec3::dot(&scattered.direction(), &hit_record.normal) > 0.0
    }
}

pub struct Dieletric {
    ref_idx: f32,
}

impl Dieletric {
    pub fn new(ref_idx: f32) -> Dieletric {
        Dieletric { ref_idx }
    }
}

impl Material for Dieletric {
    fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Vec3::uniform(1.0);

        let (outward_normal, ni_over_nt, cosine) =
            if Vec3::dot(&ray.direction(), &hit_record.normal) > 0.0 {
                (
                    -hit_record.normal,
                    self.ref_idx,
                    self.ref_idx * Vec3::dot(&ray.direction(), &hit_record.normal)
                        / ray.direction().length(),
                )
            } else {
                (
                    hit_record.normal,
                    1.0 / self.ref_idx,
                    -Vec3::dot(&ray.direction(), &hit_record.normal) / ray.direction().length(),
                )
            };

        let reflected = reflect(&ray.direction(), &hit_record.normal);
        let (reflect_prob, refracted) =
            if let Some(refracted_ray) = refract(&ray.direction(), &outward_normal, ni_over_nt) {
                (schlick(cosine, self.ref_idx), refracted_ray)
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
}

// FIXME: Something seems to have broken with the perlin noise + light and cornell smoke scenes
pub struct DiffuseLight<T: Texture> {
    emit: T,
}

impl<T: Texture> DiffuseLight<T> {
    pub fn new(emit: T) -> DiffuseLight<T> {
        DiffuseLight { emit }
    }
}

impl<T: Texture> Material for DiffuseLight<T> {
    fn scatter(
        &self,
        _ray: &Ray,
        _hit_record: &HitRecord,
        _attenuation: &mut Vec3,
        _scattered: &mut Ray,
    ) -> bool {
        false
    }

    fn emitted(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
        self.emit.value(u, v, p)
    }
}

pub struct Isotropic<T: Texture> {
    albedo: T,
}

impl<T: Texture> Isotropic<T> {
    pub fn new(albedo: T) -> Isotropic<T> {
        Isotropic { albedo }
    }
}

impl<T: Texture> Material for Isotropic<T> {
    fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = self.albedo.value(hit_record.u, hit_record.v, &hit_record.p);
        *scattered = Ray::new(hit_record.p, random_in_unit_sphere(), ray.time());
        true
    }
}



use texture::{TextureRef, TextureEnum};

#[derive(Debug)]
pub enum MaterialEnum {
    Lambertian(Vec3),
    LambertianTextured(TextureRef),
    Metal(Vec3, f32),
    Dieletric(f32),
    DiffuseLight(TextureRef),
    Isotropic(TextureRef)
}

impl MaterialEnum {
    // TODO: Any benefit in splitting these out into their own functions? e.g. reduce the scatter function size
    pub fn scatter(&self, textures: &[TextureEnum], ray: &Ray, hit_record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        match self {
            MaterialEnum::Lambertian(albedo) => {
                let target = hit_record.p + hit_record.normal + random_in_unit_sphere();

                *scattered = Ray::new(hit_record.p, target - hit_record.p, ray.time());
                *attenuation = *albedo;
                true
            },
            MaterialEnum::LambertianTextured(tex_ref) => {
                let target = hit_record.p + hit_record.normal + random_in_unit_sphere();

                *scattered = Ray::new(hit_record.p, target - hit_record.p, ray.time());
                *attenuation = textures[*tex_ref].value(textures, hit_record.u, hit_record.v, &hit_record.p);
                true
            },
            MaterialEnum::Metal(albedo, fuzz) => {
                let reflected = reflect(&ray.direction().unit(), &hit_record.normal);

                let fuzz = if *fuzz < 1.0 { *fuzz } else { 1.0 };
                *scattered = Ray::new(
                    hit_record.p,
                    reflected + fuzz * random_in_unit_sphere(),
                    ray.time(),
                );
                *attenuation = *albedo;

                Vec3::dot(&scattered.direction(), &hit_record.normal) > 0.0
            },
            MaterialEnum::Dieletric(ref_idx) => {
                *attenuation = Vec3::uniform(1.0);

                let (outward_normal, ni_over_nt, cosine) =
                    if Vec3::dot(&ray.direction(), &hit_record.normal) > 0.0 {
                        (
                            -hit_record.normal,
                            *ref_idx,
                            *ref_idx * Vec3::dot(&ray.direction(), &hit_record.normal)
                                / ray.direction().length(),
                        )
                    } else {
                        (
                            hit_record.normal,
                            1.0 / *ref_idx,
                            -Vec3::dot(&ray.direction(), &hit_record.normal) / ray.direction().length(),
                        )
                    };

                let reflected = reflect(&ray.direction(), &hit_record.normal);
                let (reflect_prob, refracted) =
                    if let Some(refracted_ray) = refract(&ray.direction(), &outward_normal, ni_over_nt) {
                        (schlick(cosine, *ref_idx), refracted_ray)
                    } else {
                        (1.0, Vec3::zero())
                    };

                if drand48() < reflect_prob {
                    *scattered = Ray::new(hit_record.p, reflected, ray.time());
                } else {
                    *scattered = Ray::new(hit_record.p, refracted, ray.time());
                }

                true
            },
            MaterialEnum::DiffuseLight(_) => false,
            MaterialEnum::Isotropic(tex_ref) => {
                *attenuation = textures[*tex_ref].value(textures, hit_record.u, hit_record.v, &hit_record.p);
                *scattered = Ray::new(hit_record.p, random_in_unit_sphere(), ray.time());
                true
            }
        }
    }
    pub fn emitted(&self, textures: &[TextureEnum], u: f32, v: f32, p: &Vec3) -> Vec3 {
        match self {
            MaterialEnum::DiffuseLight(tex_ref) => textures[*tex_ref].value(textures, u, v, p),
            _ => Vec3::zero()
        }
    }
}