use aarect::{XYRect, XZRect, YZRect};
use camera::Camera;
use cube::Cube;
use hitable::{Hitable, HitableList};
use material::{Dieletric, DiffuseLight, Isotropic, Lambertian, LambertianTextured, Material, Metal, MaterialIndex, MaterialEnum};
use random::drand48;
use sphere::{MovingSphere, Sphere};
use texture::{CheckerTexture, ConstantTexture, ImageTexture, PerlinTexture, ScaledPerlinTexture, ScaledTurbulencePerlinTexture, TextureEnum};
use transform::{FlipNormals, RotateY, Translate};
use vec3::Vec3;
use volume::ConstantMedium;

use scene::{Scene, Window};
use scene::Resources;

pub fn load_scene(name: &str, width: u32, height: u32, samples: u32) -> Result<(Scene, Window), &str> {
    match name {
        "default_scene" => Ok(make_scene(width, height, samples)),
        "random_scene" => Ok(make_random_scene(width, height, samples)),
        "random_moving_scene" => Ok(make_random_moving_scene(width, height, samples)),
        "two_spheres" => Ok(make_two_spheres_scene(width, height, samples)),
        "earth" => Ok(make_earth_scene(width, height, samples)),
        "two_perlin_spheres" => Ok(make_two_perlin_spheres_scene(width, height, samples)),
        "simple_light" => Ok(make_simple_light_scene(width, height, samples)),
        "cornell_box" => Ok(make_cornell_box(width, height, samples)),
        "cornell_smoke" => Ok(make_cornell_smoke(width, height, samples)),
        "final_scene" => Ok(make_final_scene(width, height, samples)),
        _ => Err("Unknown scene!")
    }
}

fn make_camera(
    lookfrom: Vec3,
    lookat: Vec3,
    vfov: f32,
    nx: u32,
    ny: u32,
    aperture: f32,
    dist_to_focus: f32,
) -> Camera {
    Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        vfov,
        nx as f32 / ny as f32,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    )
}

fn make_default_camera(nx: u32, ny: u32) -> Camera {
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        nx as f32 / ny as f32,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    )
}

#[allow(dead_code)]
//pub fn make_scene() -> HitableList<Sphere> {
pub fn make_scene(nx: u32, ny: u32, samples: u32) -> (Scene, Window) {
    let materials: Vec<Box<Material>> = vec![
        Box::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5))),
        Box::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))),
        Box::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.3)),
        Box::new(Dieletric::new(1.5)),
    ];

    let world = HitableList::new(vec![
        Sphere::new_boxed(Vec3::new(0.0, 0.0, -1.0), 0.5, 0),
        Sphere::new_boxed(Vec3::new(0.0, -100.5, -1.0), 100.0, 1),
        Sphere::new_boxed(Vec3::new(1.0, 0.0, -1.0), 0.5, 2),
        Sphere::new_boxed(Vec3::new(-1.0, 0.0, -1.0), 0.5, 3),
        Sphere::new_boxed(Vec3::new(-1.0, 0.0, -1.0), -0.45, 3),
    ]);

    (
        Scene::new(world, materials),
        Window::new(nx, ny, samples, make_default_camera(nx, ny)),
    )
}

#[allow(dead_code)]
pub fn make_random_scene(nx: u32, ny: u32, samples: u32) -> (Scene, Window) {
    let mut materials: Vec<Box<Material>> = vec![
        Box::new(LambertianTextured::new(CheckerTexture::new_solid(
            Vec3::new(0.2, 0.3, 0.1),
            Vec3::uniform(0.9),
        ))),
        Box::new(Dieletric::new(1.5)),
        Box::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))),
        Box::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)),
    ];

    let mut spheres: Vec<Box<Hitable>> = vec![
        Sphere::new_boxed(Vec3::new(0.0, -1000.0, 0.0), 1000.0, 0),
        Sphere::new_boxed(Vec3::new(0.0, 1.0, 0.0), 1.0, 1),
        Sphere::new_boxed(Vec3::new(-4.0, 1.0, 0.0), 1.0, 2),
        Sphere::new_boxed(Vec3::new(4.0, 1.0, 0.0), 1.0, 3),
    ];

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = drand48();
            let center = Vec3::new(a as f32 + 0.9 * drand48(), 0.2, b as f32 + 0.9 * drand48());
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    materials.push(Box::new(Lambertian::new(Vec3::new(
                        drand48() * drand48(),
                        drand48() * drand48(),
                        drand48() * drand48(),
                    ))));
                } else if choose_mat < 0.95 {
                    materials.push(Box::new(Metal::new(
                        Vec3::new(
                            0.5 * (1.0 + drand48()),
                            0.5 * (1.0 + drand48()),
                            0.5 * (1.0 + drand48()),
                        ),
                        0.5 * drand48(),
                    )));
                } else {
                    materials.push(Box::new(Dieletric::new(1.5)));
                }
                spheres.push(Sphere::new_boxed(center, 0.2, materials.len() as MaterialIndex - 1));
            }
        }
    }

    (
        Scene::new(HitableList::new(spheres), materials),
        Window::new(nx, ny, samples, make_default_camera(nx, ny)),
    )
}

#[allow(dead_code)]
pub fn make_random_moving_scene(nx: u32, ny: u32, samples: u32) -> (Scene, Window) {
    let mut materials: Vec<Box<Material>> = vec![
        Box::new(LambertianTextured::new(CheckerTexture::new_solid(
            Vec3::new(0.2, 0.3, 0.1),
            Vec3::uniform(0.9),
        ))),
        Box::new(Dieletric::new(1.5)),
        Box::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))),
        Box::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)),
    ];

    let mut spheres: Vec<Box<Hitable>> = vec![
        Sphere::new_boxed(Vec3::new(0.0, -1000.0, 0.0), 1000.0, 0),
        Sphere::new_boxed(Vec3::new(0.0, 1.0, 0.0), 1.0, 1),
        Sphere::new_boxed(Vec3::new(-4.0, 1.0, 0.0), 1.0, 2),
        Sphere::new_boxed(Vec3::new(4.0, 1.0, 0.0), 1.0, 3),
    ];

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = drand48();
            let center = Vec3::new(a as f32 + 0.9 * drand48(), 0.2, b as f32 + 0.9 * drand48());
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    materials.push(Box::new(Lambertian::new(Vec3::new(
                        drand48() * drand48(),
                        drand48() * drand48(),
                        drand48() * drand48(),
                    ))));
                    spheres.push(MovingSphere::new_boxed(
                        center,
                        center + Vec3::new(0.0, 0.5 * drand48(), 0.0),
                        0.0,
                        1.0,
                        0.2,
                        materials.len() as MaterialIndex - 1,
                    ));
                } else if choose_mat < 0.95 {
                    materials.push(Box::new(Metal::new(
                        Vec3::new(
                            0.5 * (1.0 + drand48()),
                            0.5 * (1.0 + drand48()),
                            0.5 * (1.0 + drand48()),
                        ),
                        0.5 * drand48(),
                    )));
                    spheres.push(Sphere::new_boxed(center, 0.2, materials.len() as MaterialIndex - 1));
                } else {
                    materials.push(Box::new(Dieletric::new(1.5)));
                    spheres.push(Sphere::new_boxed(center, 0.2, materials.len() as MaterialIndex - 1));
                }
            }
        }
    }

    (
        Scene::new(HitableList::new(spheres), materials),
        Window::new(nx, ny, samples, make_default_camera(nx, ny)),
    )
}

#[allow(dead_code)]
pub fn make_two_spheres_scene(nx: u32, ny: u32, samples: u32) -> (Scene, Window) {
    let materials: Vec<Box<Material>> = vec![Box::new(LambertianTextured::new(
        CheckerTexture::new_solid(Vec3::new(0.2, 0.3, 0.1), Vec3::uniform(0.9)),
    ))];

    let world = HitableList::new(vec![
        Sphere::new_boxed(Vec3::new(0.0, -10.0, 0.0), 10.0, 0),
        Sphere::new_boxed(Vec3::new(0.0, 10.0, 0.0), 10.0, 0),
    ]);

    (
        Scene::new(world, materials),
        Window::new(nx, ny, samples, make_default_camera(nx, ny)),
    )
}

#[allow(dead_code)]
pub fn make_two_perlin_spheres_scene(nx: u32, ny: u32, samples: u32) -> (Scene, Window) {
    let materials: Vec<Box<Material>> = vec![
        Box::new(LambertianTextured::new(PerlinTexture {})),
        Box::new(LambertianTextured::new(ScaledPerlinTexture::new(1.0))),
        Box::new(LambertianTextured::new(ScaledTurbulencePerlinTexture::new(
            4.0,
        ))),
    ];

    let world = HitableList::new(vec![
        Sphere::new_boxed(Vec3::new(0.0, -1000.0, 0.0), 1000.0, 2),
        Sphere::new_boxed(Vec3::new(0.0, 2.0, 0.0), 2.0, 2),
    ]);

    (
        Scene::new(world, materials),
        Window::new(nx, ny, samples, make_default_camera(nx, ny)),
    )
}

#[allow(dead_code)]
pub fn make_earth_scene(nx: u32, ny: u32, samples: u32) -> (Scene, Window) {
    let materials: Vec<Box<Material>> = vec![Box::new(LambertianTextured::new(ImageTexture::new(
        "earthmap.jpg",
    )))];
    let world = HitableList::new(vec![Sphere::new_boxed(Vec3::zero(), 2.0, 0)]);

    (
        Scene::new(world, materials),
        Window::new(nx, ny, samples, make_default_camera(nx, ny)),
    )
}

#[allow(dead_code)]
pub fn make_simple_light_scene(nx: u32, ny: u32, samples: u32) -> (Scene, Window) {
    let materials: Vec<Box<Material>> = vec![
        Box::new(LambertianTextured::new(ScaledTurbulencePerlinTexture::new(
            4.0,
        ))),
        Box::new(DiffuseLight::new(ConstantTexture::new(Vec3::uniform(4.0)))),
    ];

    let world = HitableList::new(vec![
        Sphere::new_boxed(Vec3::new(0.0, -1000.0, 0.0), 1000.0, 0),
        Sphere::new_boxed(Vec3::new(0.0, 2.0, 0.0), 2.0, 0),
        Sphere::new_boxed(Vec3::new(0.0, 7.0, 0.0), 2.0, 1),
        XYRect::new_boxed(3.0, 5.0, 1.0, 3.0, -2.0, 1),
    ]);

    (
        Scene::new(world, materials),
        Window::new(
            nx,
            ny,
            samples,
            make_camera(
                Vec3::new(18.0, 5.0, 3.0),
                Vec3::new(0.0, 0.0, 0.0),
                40.0,
                nx as u32,
                ny as u32,
                0.1,
                10.0,
            ),
        ),
    )
}

#[allow(dead_code)]
pub fn make_cornell_box(nx: u32, ny: u32, samples: u32) -> (Scene, Window) {
//    let mut resources = Resources::new();
//    resources.new_texture(TextureEnum::Constant(Vec3::new(0.65, 0.05, 0.05)));
//    resources.new_material(MaterialEnum::LambertianTextured(0));
//    resources.new_texture(TextureEnum::Constant(Vec3::new(0.12, 0.45, 0.15)));
//    resources.new_material(MaterialEnum::LambertianTextured(1));
//    resources.new_texture(TextureEnum::Constant(Vec3::uniform(0.73)));
//    resources.new_material(MaterialEnum::LambertianTextured(2));
//    resources.new_texture(TextureEnum::Constant(Vec3::uniform(15.0)));
//    resources.new_material(MaterialEnum::DiffuseLight(2));

    let materials: Vec<Box<Material>> = vec![
        Box::new(LambertianTextured::new(ConstantTexture::new(Vec3::new(
            0.65, 0.05, 0.05,
        )))),
        Box::new(LambertianTextured::new(ConstantTexture::new(Vec3::new(
            0.12, 0.45, 0.15,
        )))),
        Box::new(LambertianTextured::new(ConstantTexture::new(
            Vec3::uniform(0.73),
        ))),
        Box::new(DiffuseLight::new(ConstantTexture::new(Vec3::uniform(15.0)))),
    ];

    let cube1 = Translate::new_boxed(
        RotateY::new(Cube::new(Vec3::zero(), Vec3::uniform(165.0), 2), -18.0),
        Vec3::new(130.0, 0.0, 65.0),
    );
    let cube2 = Translate::new_boxed(
        RotateY::new(
            Cube::new(Vec3::zero(), Vec3::new(165.0, 330.0, 165.0), 2),
            15.0,
        ),
        Vec3::new(265.0, 0.0, 295.0),
    );

    let world = HitableList::new(vec![
        FlipNormals::new_boxed(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, 1)), // Left plane
        YZRect::new_boxed(0.0, 555.0, 0.0, 555.0, 0.0, 0),                     // Right plane
        XZRect::new_boxed(213.0, 343.0, 227.0, 322.0, 554.0, 3),               // Top light
        XZRect::new_boxed(0.0, 555.0, 0.0, 555.0, 0.0, 2),                     // Bottom plane
        FlipNormals::new_boxed(XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, 2)), // Top plane
        FlipNormals::new_boxed(XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, 2)), // Back plane
        cube1,
        cube2,
    ]);

    (
        Scene::new(world, materials),
//        Scene::new(world, resources),
        Window::new(
            nx,
            ny,
            samples,
            make_camera(
                Vec3::new(278.0, 278.0, -800.0),
                Vec3::new(278.0, 278.0, 0.0),
                40.0,
                nx as u32,
                ny as u32,
                0.0,
                1.0,
            ),
        ),
    )
}

#[allow(dead_code)]
pub fn make_cornell_smoke(nx: u32, ny: u32, samples: u32) -> (Scene, Window) {
    let materials: Vec<Box<Material>> = vec![
        Box::new(LambertianTextured::new(ConstantTexture::new(Vec3::new(
            0.65, 0.05, 0.05,
        )))),
        Box::new(LambertianTextured::new(ConstantTexture::new(Vec3::new(
            0.12, 0.45, 0.15,
        )))),
        Box::new(LambertianTextured::new(ConstantTexture::new(
            Vec3::uniform(0.73),
        ))),
        Box::new(DiffuseLight::new(ConstantTexture::new(Vec3::uniform(7.0)))),
        Box::new(Isotropic::new(ConstantTexture::new(Vec3::uniform(1.0)))),
        Box::new(Isotropic::new(ConstantTexture::new(Vec3::uniform(0.0)))),
    ];

    let b1 = Translate::new(
        RotateY::new(
            Cube::new(Vec3::uniform(0.0), Vec3::uniform(165.0), 2),
            -18.0,
        ),
        Vec3::new(130.0, 0.0, 65.0),
    );
    let b2 = Translate::new(
        RotateY::new(
            Cube::new(Vec3::uniform(0.0), Vec3::new(165.0, 330.0, 165.0), 2),
            15.0,
        ),
        Vec3::new(265.0, 0.0, 295.0),
    );

    let world = HitableList::new(vec![
        FlipNormals::new_boxed(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, 1)), // Left plane
        YZRect::new_boxed(0.0, 555.0, 0.0, 555.0, 0.0, 0),                     // Right plane
        XZRect::new_boxed(113.0, 443.0, 127.0, 432.0, 554.0, 3),               // Top light
        XZRect::new_boxed(0.0, 555.0, 0.0, 555.0, 0.0, 2),                     // Bottom plane
        FlipNormals::new_boxed(XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, 2)), // Top plane
        FlipNormals::new_boxed(XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, 2)), // Back plane
        ConstantMedium::new_boxed(b1, 0.01, 4),
        ConstantMedium::new_boxed(b2, 0.01, 5),
    ]);

    (
        Scene::new(world, materials),
        Window::new(
            nx,
            ny,
            samples,
            make_camera(
                Vec3::new(278.0, 278.0, -800.0),
                Vec3::new(278.0, 278.0, 0.0),
                40.0,
                nx as u32,
                ny as u32,
                0.0,
                1.0,
            ),
        ),
    )
}

#[allow(dead_code)]
pub fn make_final_scene(nx: u32, ny: u32, samples: u32) -> (Scene, Window) {
    let mut materials: Vec<Box<Material>> = vec![];
    let mut hitables = vec![];

//    let white = Box::new()

    (
        Scene::new(HitableList::new(hitables), materials),
//        Scene::new(world, resources),
        Window::new(
            nx,
            ny,
            samples,
            make_camera(
                Vec3::new(278.0, 278.0, -800.0),
                Vec3::new(278.0, 278.0, 0.0),
                40.0,
                nx as u32,
                ny as u32,
                0.0,
                1.0,
            ),
        ),
    )
}