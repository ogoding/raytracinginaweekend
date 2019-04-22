use aarect::{XYRect, XZRect, YZRect};
use camera::Camera;
use cube::Cube;
use transform::{FlipNormals, RotateY, Translate};
use vec3::Vec3;

use scene::{Scene, Window};
use scene::{Resources, MaterialRef};
use material::MaterialEnum;
use texture::TextureEnum;
use sphere::{Sphere, MovingSphere};
use volume::ConstantMedium;
use random::drand48;


pub fn load_scene(name: &str, width: u32, height: u32, samples: u32) -> Result<(Scene, Window), String> {
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
        _ => Err("Unknown scene!".to_owned())
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
pub fn make_scene(nx: u32, ny: u32, samples: u32) -> (Scene, Window) {
    let mut resources = Resources::new();
    resources.new_material(MaterialEnum::Lambertian(Vec3::new(0.1, 0.2, 0.5)));
    resources.new_material(MaterialEnum::Lambertian(Vec3::new(0.8, 0.8, 0.0)));
    resources.new_material(MaterialEnum::Metal(Vec3::new(0.8, 0.6, 0.2), 0.3));
    resources.new_material(MaterialEnum::Dieletric(1.5));

    resources.new_entity(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, 0));
    resources.new_entity(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, 1));
    resources.new_entity(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, 2));
    resources.new_entity(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, 3));
    resources.new_entity(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.45, 3));

    (
        Scene::new(resources),
        Window::new(nx, ny, samples, make_default_camera(nx, ny)),
    )
}

#[allow(dead_code)]
pub fn make_random_scene(nx: u32, ny: u32, samples: u32) -> (Scene, Window) {
    let mut resources = Resources::new();
    resources.new_texture(TextureEnum::Constant(Vec3::new(0.2, 0.3, 0.1)));
    resources.new_texture(TextureEnum::Constant(Vec3::uniform(0.9)));
    let tex = resources.new_texture(TextureEnum::Checker(0, 1));
    resources.new_material(MaterialEnum::LambertianTextured(tex));
    resources.new_material(MaterialEnum::Dieletric(1.5));
    resources.new_material(MaterialEnum::Lambertian(Vec3::new(0.4, 0.2, 0.1)));
    resources.new_material(MaterialEnum::Metal(Vec3::new(0.7, 0.6, 0.5), 0.0));

    resources.new_entity(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, 0));
    resources.new_entity(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, 1));
    resources.new_entity(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, 2));
    resources.new_entity(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, 3));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = drand48();
            let center = Vec3::new(a as f32 + 0.9 * drand48(), 0.2, b as f32 + 0.9 * drand48());
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let material = if choose_mat < 0.8 {
                    resources.new_material(MaterialEnum::Lambertian(Vec3::random() * Vec3::random()))
                } else if choose_mat < 0.95 {
                    resources.new_material(MaterialEnum::Metal(0.5 * (1.0 + Vec3::random()), 0.5 * drand48()))
                } else {
                    resources.new_material(MaterialEnum::Dieletric(1.5))
                };

                resources.new_entity(Sphere::new(center, 0.2, material as MaterialRef));
            }
        }
    }

    (
        Scene::new(resources),
        Window::new(nx, ny, samples, make_default_camera(nx, ny)),
    )
}

#[allow(dead_code)]
pub fn make_random_moving_scene(nx: u32, ny: u32, samples: u32) -> (Scene, Window) {
    let mut resources = Resources::new();
    resources.new_texture(TextureEnum::Constant(Vec3::new(0.2, 0.3, 0.1)));
    resources.new_texture(TextureEnum::Constant(Vec3::uniform(0.9)));
    let tex = resources.new_texture(TextureEnum::Checker(0, 1));
    resources.new_material(MaterialEnum::LambertianTextured(tex));
    resources.new_material(MaterialEnum::Dieletric(1.5));
    resources.new_material(MaterialEnum::Lambertian(Vec3::new(0.4, 0.2, 0.1)));
    resources.new_material(MaterialEnum::Metal(Vec3::new(0.7, 0.6, 0.5), 0.0));

    resources.new_entity(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, 0));
    resources.new_entity(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, 1));
    resources.new_entity(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, 2));
    resources.new_entity(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, 3));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = drand48();
            let center = Vec3::new(a as f32 + 0.9 * drand48(), 0.2, b as f32 + 0.9 * drand48());
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let material = resources.new_material(MaterialEnum::Lambertian(Vec3::random() * Vec3::random()));
                    resources.new_entity(MovingSphere::new(center,
                                                           center + Vec3::new(0.0, 0.5 * drand48(), 0.0),
                                                           0.0,
                                                           1.0,
                                                           0.2,
                                                           material as MaterialRef));
                } else if choose_mat < 0.95 {
                    let material = resources.new_material(MaterialEnum::Metal(0.5 * (1.0 + Vec3::random()), 0.5 * drand48()));
                    resources.new_entity(Sphere::new(center, 0.2, material as MaterialRef));
                } else {
                    let material = resources.new_material(MaterialEnum::Dieletric(1.5));
                    resources.new_entity(Sphere::new(center, 0.2, material as MaterialRef));
                }
            }
        }
    }

    (
        Scene::new(resources),
        Window::new(nx, ny, samples, make_default_camera(nx, ny)),
    )
}

#[allow(dead_code)]
pub fn make_two_spheres_scene(nx: u32, ny: u32, samples: u32) -> (Scene, Window) {
    let mut resources = Resources::new();
    resources.new_texture(TextureEnum::Constant(Vec3::new(0.2, 0.3, 0.1)));
    resources.new_texture(TextureEnum::Constant(Vec3::uniform(0.9)));
    resources.new_texture(TextureEnum::Checker(0, 1));
    resources.new_material(MaterialEnum::LambertianTextured(0));

    resources.new_entity(Sphere::new(Vec3::new(0.0, -10.0, 0.0), 10.0, 0));
    resources.new_entity(Sphere::new(Vec3::new(0.0, 10.0, 0.0), 10.0, 0));

    (
        Scene::new(resources),
        Window::new(nx, ny, samples, make_default_camera(nx, ny)),
    )
}

#[allow(dead_code)]
pub fn make_two_perlin_spheres_scene(nx: u32, ny: u32, samples: u32) -> (Scene, Window) {
    let mut resources = Resources::new();
//    let tex = resources.new_texture(TextureEnum::Perlin);
//    let tex = resources.new_texture(TextureEnum::ScaledPerlin(1.0));
    let tex = resources.new_texture(TextureEnum::ScaledTurbulencePerlin(4.0));
    let perlin_mat = resources.new_material(MaterialEnum::LambertianTextured(tex));

    resources.new_entity(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, perlin_mat));
    resources.new_entity(Sphere::new(Vec3::new(0.0, 2.0, 0.0), 2.0, perlin_mat));
    (
        Scene::new(resources),
        Window::new(nx, ny, samples, make_default_camera(nx, ny)),
    )
}

#[allow(dead_code)]
pub fn make_earth_scene(nx: u32, ny: u32, samples: u32) -> (Scene, Window) {
    let mut resources = Resources::new();
    let tex = resources.new_texture(TextureEnum::Image(::imagers::open("earthmap.jpg").unwrap().to_rgb()));
    resources.new_material(MaterialEnum::LambertianTextured(tex));
    resources.new_entity(Sphere::new(Vec3::zero(), 2.0, 0));

    (
        Scene::new(resources),
        Window::new(nx, ny, samples, make_default_camera(nx, ny)),
    )
}

#[allow(dead_code)]
pub fn make_simple_light_scene(nx: u32, ny: u32, samples: u32) -> (Scene, Window) {
    let mut resources = Resources::new();
    resources.new_texture(TextureEnum::ScaledTurbulencePerlin(4.0));
    resources.new_material(MaterialEnum::LambertianTextured(0));
    resources.new_texture(TextureEnum::Constant(Vec3::uniform(4.0)));
    resources.new_material(MaterialEnum::DiffuseLight(1));

    resources.new_entity(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, 0));
    resources.new_entity(Sphere::new(Vec3::new(0.0, 2.0, 0.0), 2.0, 0));
    resources.new_entity(Sphere::new(Vec3::new(0.0, 7.0, 0.0), 2.0, 1));
    resources.new_entity(XYRect::new(3.0, 5.0, 1.0, 3.0, -2.0, 1));

    (
        Scene::new(resources),
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
    let mut resources = Resources::new();
    let red_t = resources.new_texture(TextureEnum::Constant(Vec3::new(0.65, 0.05, 0.05)));
    let red = resources.new_material(MaterialEnum::LambertianTextured(red_t));

    let green_t = resources.new_texture(TextureEnum::Constant(Vec3::new(0.12, 0.45, 0.15)));
    let green = resources.new_material(MaterialEnum::LambertianTextured(green_t));

    let white_t = resources.new_texture(TextureEnum::Constant(Vec3::uniform(0.73)));
    let white = resources.new_material(MaterialEnum::LambertianTextured(white_t));

    let light_t = resources.new_texture(TextureEnum::Constant(Vec3::uniform(7.0)));
    let light = resources.new_material(MaterialEnum::DiffuseLight(light_t));


    // TODO: Flip this, so that the type is in charge of pushing any required entities - Maybe not needed?
    resources.new_entity(Translate::new(RotateY::new(Cube::new(Vec3::zero(),
                                                               Vec3::uniform(165.0), white), -18.0),
                                        Vec3::new(130.0, 0.0, 65.0)));
    resources.new_entity(Translate::new(RotateY::new(Cube::new(Vec3::zero(),
                                                               Vec3::new(165.0, 330.0, 165.0), white), 15.0, ),
                                        Vec3::new(265.0, 0.0, 295.0)));

    resources.new_entity(FlipNormals::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green))); // Left plane
    resources.new_entity(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red));                     // Right plane
    resources.new_entity(XZRect::new(113.0, 443.0, 127.0, 432.0, 554.0, light));               // Top light
    resources.new_entity(XZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white));                     // Bottom plane
    resources.new_entity(FlipNormals::new(XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white))); // Top plane
    resources.new_entity(FlipNormals::new(XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white))); // Back plane

    (
        Scene::new(resources),
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
    let mut resources = Resources::new();
    let red_t = resources.new_texture(TextureEnum::Constant(Vec3::new(0.65, 0.05, 0.05)));
    let red = resources.new_material(MaterialEnum::LambertianTextured(red_t));

    let green_t = resources.new_texture(TextureEnum::Constant(Vec3::new(0.12, 0.45, 0.15)));
    let green = resources.new_material(MaterialEnum::LambertianTextured(green_t));

    let white_t = resources.new_texture(TextureEnum::Constant(Vec3::uniform(0.73)));
    let white = resources.new_material(MaterialEnum::LambertianTextured(white_t));

    let light_t = resources.new_texture(TextureEnum::Constant(Vec3::uniform(7.0)));
    let light = resources.new_material(MaterialEnum::DiffuseLight(light_t));

    let smoke_box_t_0 = resources.new_texture(TextureEnum::Constant(Vec3::uniform(1.0)));
    let smoke_box_m_0 = resources.new_material(MaterialEnum::Isotropic(smoke_box_t_0));
    let smoke_box_t_1 = resources.new_texture(TextureEnum::Constant(Vec3::uniform(0.0)));
    let smoke_box_m_1 = resources.new_material(MaterialEnum::Isotropic(smoke_box_t_1));


    resources.new_entity(FlipNormals::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green))); // Left plane
    resources.new_entity(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red));                     // Right plane
    resources.new_entity(XZRect::new(113.0, 443.0, 127.0, 432.0, 554.0, light));               // Top light
    resources.new_entity(XZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white));                     // Bottom plane
    resources.new_entity(FlipNormals::new(XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white))); // Top plane
    resources.new_entity(FlipNormals::new(XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white))); // Back plane

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

    resources.new_entity(ConstantMedium::new(b1, 0.01, smoke_box_m_0));
    resources.new_entity(ConstantMedium::new(b2, 0.01, smoke_box_m_1));

    (
        Scene::new(resources),
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
    let mut resources = Resources::new();
    let tex = resources.new_texture(TextureEnum::Constant(Vec3::uniform(0.73)));
    let white = resources.new_material(MaterialEnum::LambertianTextured(tex));
    let tex = resources.new_texture(TextureEnum::Constant(Vec3::new(0.48, 0.83, 0.53)));
    let ground = resources.new_material(MaterialEnum::LambertianTextured(tex));

    let tex = resources.new_texture(TextureEnum::Constant(Vec3::uniform(7.0)));
    let light = resources.new_material(MaterialEnum::DiffuseLight(tex));
    let tex = resources.new_texture(TextureEnum::Constant(Vec3::new(0.7, 0.3, 0.1)));
    let brown = resources.new_material(MaterialEnum::LambertianTextured(tex));

    let glass = resources.new_material(MaterialEnum::Dieletric(1.5));
    let metal = resources.new_material(MaterialEnum::Metal(Vec3::new(0.8, 0.8, 0.9), 10.0));

    let tex = resources.new_texture(TextureEnum::Constant(Vec3::new(0.2, 0.4, 0.9)));
    let blue = resources.new_material(MaterialEnum::Isotropic(tex));
    let tex = resources.new_texture(TextureEnum::Constant(Vec3::uniform(1.0)));
    let black = resources.new_material(MaterialEnum::Isotropic(tex));

    let tex = resources.new_texture(TextureEnum::Image(::imagers::open("earthmap.jpg").unwrap().to_rgb()));
    let earthmap = resources.new_material(MaterialEnum::LambertianTextured(tex));
    let tex = resources.new_texture(TextureEnum::ScaledTurbulencePerlin(0.1));
    let perlin = resources.new_material(MaterialEnum::LambertianTextured(tex));

    for i in 0..20 {
        for j in 0..20 {
            let w = 100.0;
            let x0 = -1000.0 + i as f32 * w;
            let z0 = -1000.0 + j as f32 * w;
            let y0 = 0.0;
            resources.new_entity(Cube::new(Vec3::new(x0, y0, z0),
                                           Vec3::new(x0 + w, 100.0 * drand48() + 0.01, z0 + w),
                                           ground));
        }
    }

    resources.new_entity(XZRect::new(123.0, 423.0, 147.0, 412.0, 554.0, light));

    let center = Vec3::new(400.0, 400.0, 200.0);
    resources.new_entity(MovingSphere::new(center, center + Vec3::new(30.0, 0.0, 0.0), 0.0, 1.0, 50.0, brown));
    resources.new_entity(Sphere::new(Vec3::new(260.0, 150.0, 45.0), 50.0, glass));
    resources.new_entity(Sphere::new(Vec3::new(0.0, 150.0, 145.0), 50.0, metal));

    let boundary = Sphere::new(Vec3::new(360.0, 150.0, 145.0), 70.0, glass);
    resources.new_entity(boundary.clone());
    resources.new_entity(ConstantMedium::new(boundary, 0.2, blue));

    let mut boundary = Sphere::new(Vec3::new(360.0, 150.0, 145.0), 5000.0, glass);
    resources.new_entity(ConstantMedium::new(boundary, 0.0001, black));

    resources.new_entity(Sphere::new(Vec3::new(400.0, 200.0, 400.0), 100.0, earthmap));
    resources.new_entity(Sphere::new(Vec3::new(220.0, 280.0, 300.0), 80.0, perlin));

    for _i in 0..1000 {
        let particle = Sphere::new(Vec3::random() * 165.0, 10.0, white);
        resources.new_entity(Translate::new(RotateY::new(particle, 15.0), Vec3::new(-100.0, 270.0, 395.0)));
    }

    (
        Scene::new(resources),
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