#![allow(unused_imports)]

use vec3::Vec3;
use hitable::{Hitable, HitableList, FlipNormals};
use sphere::{Sphere, MovingSphere};
use camera::Camera;
use aarect::{XYRect, XZRect, YZRect};
use material::{Material, Lambertian, LambertianTextured, Metal, Dieletric, DiffuseLight};
use texture::{ConstantTexture, CheckerTexture, PerlinTexture, ScaledPerlinTexture, ScaledTurbulencePerlinTexture, ImageTexture};
use transform::{Translate, RotateY};
//use cube::Cube;
use volume::ConstantMedium;
use random::drand48;

use scene::{Scene, Window};

fn make_camera(lookfrom: Vec3, lookat: Vec3, vfov: f32, nx: u32, ny: u32, aperture: f32, dist_to_focus: f32) -> Camera {
    Camera::new(lookfrom, lookat, Vec3::new(0.0, 1.0, 0.0), vfov, nx as f32 / ny as f32, aperture, dist_to_focus, 0.0, 1.0)
}

fn make_default_camera(nx: u32, ny: u32) -> Camera {
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    Camera::new(lookfrom, lookat, Vec3::new(0.0, 1.0, 0.0), 20.0, nx as f32 / ny as f32, aperture, dist_to_focus, 0.0, 1.0)
}

#[allow(dead_code)]
//pub fn make_scene() -> HitableList<Sphere> {
pub fn make_scene(nx: u32, ny: u32) -> (HitableList, Camera) {
    let world = HitableList::new(vec![
        Sphere::new_boxed(Vec3::new(0.0, 0.0, -1.0), 0.5, Box::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5)))),
        Sphere::new_boxed(Vec3::new(0.0, -100.5, -1.0), 100.0, Box::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)))),
        Sphere::new_boxed(Vec3::new(1.0, 0.0, -1.0), 0.5, Box::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.3))),
        Sphere::new_boxed(Vec3::new(-1.0, 0.0, -1.0), 0.5, Box::new(Dieletric::new(1.5))),
        Sphere::new_boxed(Vec3::new(-1.0, 0.0, -1.0), -0.45, Box::new(Dieletric::new(1.5)))
    ]);

    (world, make_default_camera(nx, ny))
}

#[allow(dead_code)]
//pub fn make_random_scene() -> HitableList<Sphere> {
pub fn make_random_scene(nx: u32, ny: u32) -> (HitableList, Camera) {
//    let mut spheres: Vec<Box<Hitable>> = vec![Sphere::new_boxed(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Box::new(Lambertian::new(Vec3::uniform(0.5))))];
//    let mut spheres: Vec<Box<Hitable>> = vec![Sphere::new_boxed(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Box::new(LambertianTextured::new(ConstantTexture::new(Vec3::uniform(0.5)))))];
    let mut spheres: Vec<Box<Hitable>> = vec![Sphere::new_boxed(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Box::new(LambertianTextured::new(CheckerTexture::new_solid(Vec3::new(0.2, 0.3, 0.1), Vec3::uniform(0.9)))))];

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = drand48();
            let center = Vec3::new(a as f32 + 0.9 * drand48(), 0.2, b as f32 + 0.9 * drand48());
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    spheres.push(Sphere::new_boxed(center, 0.2,
                                                   Box::new(Lambertian::new(Vec3::new(drand48() * drand48(),
                                                                                      drand48() * drand48(),
                                                                                      drand48() * drand48())))));
                } else if choose_mat < 0.95 {
                    spheres.push(Sphere::new_boxed(center, 0.2,
                                                   Box::new(Metal::new(Vec3::new(0.5 * (1.0 + drand48()),
                                                                                 0.5 * (1.0 + drand48()),
                                                                                 0.5 * (1.0 + drand48())),
                                                                       0.5 * drand48()))));
                } else {
                    spheres.push(Sphere::new_boxed(center, 0.2, Box::new(Dieletric::new(1.5))));
                }
            }
        }
    }

    spheres.push(Sphere::new_boxed(Vec3::new(0.0, 1.0, 0.0), 1.0, Box::new(Dieletric::new(1.5))));
    spheres.push(Sphere::new_boxed(Vec3::new(-4.0, 1.0, 0.0), 1.0, Box::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)))));
    spheres.push(Sphere::new_boxed(Vec3::new(4.0, 1.0, 0.0), 1.0, Box::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0))));

    (HitableList::new(spheres), make_default_camera(nx, ny))
}

#[allow(dead_code)]
pub fn make_random_moving_scene(nx: u32, ny: u32) -> (HitableList, Camera) {
//    let mut spheres: Vec<Box<Hitable>> = vec![Sphere::new_boxed(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Box::new(Lambertian::new(Vec3::uniform(0.5))))];
//    let mut spheres: Vec<Box<Hitable>> = vec![Sphere::new_boxed(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Box::new(LambertianTextured::new(ConstantTexture::new(Vec3::uniform(0.5)))))];
    let mut spheres: Vec<Box<Hitable>> = vec![Sphere::new_boxed(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Box::new(LambertianTextured::new(CheckerTexture::new_solid(Vec3::new(0.2, 0.3, 0.1), Vec3::uniform(0.9)))))];

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = drand48();
            let center = Vec3::new(a as f32 + 0.9 * drand48(), 0.2, b as f32 + 0.9 * drand48());
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    spheres.push(MovingSphere::new_boxed(center, center + Vec3::new(0.0, 0.5 * drand48(), 0.0),
                                                         0.0, 1.0, 0.2,
                                                         Box::new(Lambertian::new(Vec3::new(drand48() * drand48(),
                                                                                            drand48() * drand48(),
                                                                                            drand48() * drand48())))));
                } else if choose_mat < 0.95 {
                    spheres.push(Sphere::new_boxed(center, 0.2,
                                                   Box::new(Metal::new(Vec3::new(0.5 * (1.0 + drand48()),
                                                                                 0.5 * (1.0 + drand48()),
                                                                                 0.5 * (1.0 + drand48())),
                                                                       0.5 * drand48()))));
                } else {
                    spheres.push(Sphere::new_boxed(center, 0.2, Box::new(Dieletric::new(1.5))));
                }
            }
        }
    }

    spheres.push(Sphere::new_boxed(Vec3::new(0.0, 1.0, 0.0), 1.0, Box::new(Dieletric::new(1.5))));
    spheres.push(Sphere::new_boxed(Vec3::new(-4.0, 1.0, 0.0), 1.0, Box::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)))));
    spheres.push(Sphere::new_boxed(Vec3::new(4.0, 1.0, 0.0), 1.0, Box::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0))));

    (HitableList::new(spheres), make_default_camera(nx, ny))
}

#[allow(dead_code)]
pub fn make_two_spheres_scene(nx: u32, ny: u32) -> (HitableList, Camera) {
    // TODO: Remove duplication
    let checker = CheckerTexture::new_solid(Vec3::new(0.2, 0.3, 0.1), Vec3::uniform(0.9));
    let checker2 = CheckerTexture::new_solid(Vec3::new(0.2, 0.3, 0.1), Vec3::uniform(0.9));

    let world = HitableList::new(
        vec![Sphere::new_boxed(Vec3::new(0.0, -10.0, 0.0), 10.0, Box::new(LambertianTextured::new(checker))),
             Sphere::new_boxed(Vec3::new(0.0, 10.0, 0.0), 10.0, Box::new(LambertianTextured::new(checker2)))]);

    (world, make_default_camera(nx, ny))
}

#[allow(dead_code)]
pub fn make_two_perlin_spheres_scene(nx: u32, ny: u32) -> (HitableList, Camera) {
    // TODO: Remove duplication
//    let checker = PerlinTexture{};
//    let checker2 = PerlinTexture{};
//    let checker = ScaledPerlinTexture::new(1.0);
//    let checker2 = ScaledPerlinTexture::new(1.0);
    let checker = ScaledTurbulencePerlinTexture::new(4.0);
    let checker2 = ScaledTurbulencePerlinTexture::new(4.0);

    let world = HitableList::new(
        vec![Sphere::new_boxed(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Box::new(LambertianTextured::new(checker))),
             Sphere::new_boxed(Vec3::new(0.0, 2.0, 0.0), 2.0, Box::new(LambertianTextured::new(checker2)))]);
    (world, make_default_camera(nx, ny))
}

#[allow(dead_code)]
pub fn make_earth_scene(nx: u32, ny: u32) -> (HitableList, Camera) {
    let world = HitableList::new(vec![
        Sphere::new_boxed(Vec3::zero(), 2.0, Box::new(LambertianTextured::new(ImageTexture::new("earthmap.jpg"))))
    ]);

    (world, make_default_camera(nx, ny))
}

#[allow(dead_code)]
pub fn make_simple_light_scene(nx: u32, ny: u32) -> (HitableList, Camera) {
    // TODO: Remove duplication
    let checker = ScaledTurbulencePerlinTexture::new(4.0);
    let checker2 = ScaledTurbulencePerlinTexture::new(4.0);

    let world = HitableList::new(vec![
        Sphere::new_boxed(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Box::new(LambertianTextured::new(checker))),
        Sphere::new_boxed(Vec3::new(0.0, 2.0, 0.0), 2.0, Box::new(LambertianTextured::new(checker2))),
        Sphere::new_boxed(Vec3::new(0.0, 7.0, 0.0), 2.0, Box::new(DiffuseLight::new(ConstantTexture::new(Vec3::uniform(4.0))))),
        XYRect::new_boxed(3.0, 5.0, 1.0, 3.0, -2.0, Box::new(DiffuseLight::new(ConstantTexture::new(Vec3::uniform(4.0)))))
    ]);

//    (world, make_default_camera(nx, ny))
    (world, make_camera(Vec3::new(18.0, 5.0, 3.0), Vec3::new(0.0, 0.0, 0.0), 40.0, nx as u32, ny as u32, 0.1, 10.0))
}

#[allow(dead_code)]
pub fn make_cornell_box(nx: u32, ny: u32) -> (HitableList, Camera) {
    let red = LambertianTextured::new(ConstantTexture::new(Vec3::new(0.65, 0.05, 0.05)));
    let green= LambertianTextured::new(ConstantTexture::new(Vec3::new(0.12, 0.45, 0.15)));
    // TODO: Remove duplication
    let white = LambertianTextured::new(ConstantTexture::new(Vec3::uniform(0.73)));
    let white2 = LambertianTextured::new(ConstantTexture::new(Vec3::uniform(0.73)));
    let white3 = LambertianTextured::new(ConstantTexture::new(Vec3::uniform(0.73)));
    let white4 = LambertianTextured::new(ConstantTexture::new(Vec3::uniform(0.73)));
    let white5 = LambertianTextured::new(ConstantTexture::new(Vec3::uniform(0.73)));
    let light = DiffuseLight::new(ConstantTexture::new(Vec3::uniform(15.0)));

//    let cube1 = Translate::new_boxed(RotateY::new(Cube::new(Vec3::zero(), Vec3::uniform(165.0), Box::new(white4)), -18.0), Vec3::new(130.0, 0.0, 65.0));
//    let cube2 = Translate::new_boxed(RotateY::new(Cube::new(Vec3::zero(), Vec3::new(165.0, 330.0, 165.0), Box::new(white5)), 15.0), Vec3::new(265.0, 0.0, 295.0));

    let world = HitableList::new(vec![
        FlipNormals::new_boxed(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, Box::new(green))),        // Left plane
        YZRect::new_boxed(0.0, 555.0, 0.0, 555.0, 0.0, Box::new(red)),                                   // Right plane
        XZRect::new_boxed(213.0, 343.0, 227.0, 322.0, 554.0, Box::new(light)),                           // Top light
        XZRect::new_boxed(0.0, 555.0, 0.0, 555.0, 0.0, Box::new(white)),                                 // Bottom plane
        FlipNormals::new_boxed(XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, Box::new(white2))),       // Top plane
        FlipNormals::new_boxed(XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, Box::new(white3))),       // Back plane
//        cube1,
//        cube2
    ]);

    (world, make_camera(Vec3::new(278.0, 278.0, -800.0), Vec3::new(278.0, 278.0, 0.0), 40.0, nx as u32, ny as u32, 0.0, 1.0))
}

#[allow(dead_code)]
pub fn make_cornell_smoke(nx: u32, ny: u32) -> (HitableList, Camera) {
    let red = LambertianTextured::new(ConstantTexture::new(Vec3::new(0.65, 0.05, 0.05)));
    let green= LambertianTextured::new(ConstantTexture::new(Vec3::new(0.12, 0.45, 0.15)));
    // TODO: Remove duplication
    let white = LambertianTextured::new(ConstantTexture::new(Vec3::uniform(0.73)));
    let white2 = LambertianTextured::new(ConstantTexture::new(Vec3::uniform(0.73)));
    let white3 = LambertianTextured::new(ConstantTexture::new(Vec3::uniform(0.73)));
    let white4 = LambertianTextured::new(ConstantTexture::new(Vec3::uniform(0.73)));
    let white5 = LambertianTextured::new(ConstantTexture::new(Vec3::uniform(0.73)));
    let light = DiffuseLight::new(ConstantTexture::new(Vec3::uniform(15.0)));

//    let b1 = Translate::new(RotateY::new(Cube::new(Vec3::uniform(0.0), Vec3::uniform(165.0), Box::new(white4)), -18.0), Vec3::new(130.0, 0.0, 65.0));
//    let b2 = Translate::new(RotateY::new(Cube::new(Vec3::uniform(0.0), Vec3::new(165.0, 330.0, 165.0), Box::new(white5)), 15.0), Vec3::new(265.0, 0.0, 295.0));

    let world = HitableList::new(vec![
        FlipNormals::new_boxed(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, Box::new(green))),        // Left plane
        YZRect::new_boxed(0.0, 555.0, 0.0, 555.0, 0.0, Box::new(red)),                                   // Right plane
        XZRect::new_boxed(213.0, 343.0, 227.0, 322.0, 554.0, Box::new(light)),                           // Top light
        XZRect::new_boxed(0.0, 555.0, 0.0, 555.0, 0.0, Box::new(white)),                                 // Bottom plane
        FlipNormals::new_boxed(XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, Box::new(white2))),       // Top plane
        FlipNormals::new_boxed(XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, Box::new(white3))),       // Back plane
//        ConstantMedium::new_boxed(Box::new(b1), 0.01, ConstantTexture::new(Vec3::uniform(1.0))),
//        ConstantMedium::new_boxed(Box::new(b2), 0.01, ConstantTexture::new(Vec3::uniform(0.0)))
    ]);

    (world, make_camera(Vec3::new(278.0, 278.0, -800.0), Vec3::new(278.0, 278.0, 0.0), 40.0, nx as u32, ny as u32, 0.0, 1.0))
}
