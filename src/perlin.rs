#![allow(dead_code, unused_variables)]

use random::drand48;
use vec3::Vec3;

//  TODO: Test this module. Eventually replace with SIMD Noise crate?

lazy_static! {
    static ref RAND_VEC3: [Vec3; 256] = generate_vec3();
    static ref RAND_FLOATS: [f32; 256] = generate_floats();
    static ref PERM_X: [i32; 256] = perlin_generate_perm();
    static ref PERM_Y: [i32; 256] = perlin_generate_perm();
    static ref PERM_Z: [i32; 256] = perlin_generate_perm();
}

fn generate_vec3() -> [Vec3; 256] {
    let mut vecs = [Vec3::zero(); 256];

    for vec in vecs.iter_mut() {
        *vec = (Vec3::uniform(-1.0) + Vec3::uniform(2.0) * Vec3::random()).unit();
    }

    vecs
}

fn generate_floats() -> [f32; 256] {
    let mut floats = [0.0; 256];
    for float in floats.iter_mut() {
        *float = drand48();
    }
    floats
}

fn permute<T: Copy>(values: &mut [T; 256]) {
    for i in (0..256).rev() {
        let target = (drand48() as i32 * (i + 1)) as usize;
        let tmp = values[i as usize];
        values[i as usize] = values[target];
        values[target] = tmp;
    }
}

fn perlin_generate_perm() -> [i32; 256] {
    let mut vals = [0; 256];
    for i in 0..256 {
        vals[i as usize] = i as i32;
    }
    vals
}

pub fn turb(p: &Vec3, depth: i32) -> f32 {
    let mut accum = 0.0;
    let mut weight = 1.0;
    let mut temp_p = *p;

    for i in 0..depth {
        accum += weight * trilinear_noise_vecs(&temp_p);
        weight *= 0.5;
        temp_p = temp_p * 2.0;
    }

    accum.abs()
}

#[inline]
fn trilinear_interp_vecs(c: [[[Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
    // Apply Hermite cubic
    let u = u * u * (3.0 - 2.0 * u);
    let v = v * v * (3.0 - 2.0 * v);
    let w = w * w * (3.0 - 2.0 * w);

    let mut accum = 0.0;

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let weight_v = Vec3::new(u - i as f32, v - j as f32, w - k as f32);
                accum += (i as f32 * u + (1 - i) as f32 * (1.0 - u)) *
                    (j as f32 * v + (1 - j) as f32 * (1.0 - v)) *
                    (k as f32 * w + (1 - k) as f32 * (1.0 - w)) *
                    Vec3::dot(&c[i as usize][j as usize][k as usize], &weight_v);
            }
        }
    }
    accum
}

#[inline]
fn trilinear_interp(c: [[[f32; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
    // Apply Hermite cubic
    let u = u * u * (3.0 - 2.0 * u);
    let v = v * v * (3.0 - 2.0 * v);
    let w = w * w * (3.0 - 2.0 * w);

    let mut accum = 0.0;

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                accum += (i as f32 * u + (1 - i) as f32 * (1.0 - u)) *
                    (j as f32 * v + (1 - j) as f32 * (1.0 - v)) *
                    (k as f32 * w + (1 - k) as f32 * (1.0 - w)) *
                    c[i as usize][j as usize][k as usize];
            }
        }
    }
    accum
}

pub fn trilinear_noise_vecs(p: &Vec3) -> f32 {
    let u = p.x() - p.x().floor();
    let v = p.y() - p.y().floor();
    let w = p.z() - p.z().floor();

    let i = p.x().floor();
    let j = p.y().floor();
    let k = p.z().floor();
    let mut c = [[[Vec3::zero(); 2]; 2]; 2];

    for di in 0..2 {
        for dj in 0..2 {
            for dk in 0..2 {
                c[di][dj][dk] = RAND_VEC3[(PERM_X[(i as usize + di) & 255] ^ PERM_Y[(j as usize + dj) & 255] ^ PERM_Z[(k as usize + dk) & 255]) as usize]
            }
        }
    }

    trilinear_interp_vecs(c, u, v, w)
}

pub fn trilinear_noise(p: &Vec3) -> f32 {
    let u = p.x() - p.x().floor();
    let v = p.y() - p.y().floor();
    let w = p.z() - p.z().floor();

    let i = p.x().floor();
    let j = p.y().floor();
    let k = p.z().floor();
    let mut c = [[[0.0; 2]; 2]; 2];

    for di in 0..2 {
        for dj in 0..2 {
            for dk in 0..2 {
                c[di][dj][dk] = RAND_FLOATS[(PERM_X[(i as usize + di) & 255] ^ PERM_Y[(j as usize + dj) & 255] ^ PERM_Z[(k as usize + dk) & 255]) as usize]
            }
        }
    }

    trilinear_interp(c, u, v, w)
}

pub fn noise(p: &Vec3) -> f32 {
    let u = p.x() - p.x().floor();
    let v = p.y() - p.y().floor();
    let w = p.z() - p.z().floor();

    let i = (4.0 * p.x()) as usize & 255;
    let j = (4.0 * p.y()) as usize & 255;
    let k = (4.0 * p.z()) as usize & 255;

    let rand_index = (PERM_X[i] ^ PERM_Y[j] ^ PERM_Z[k]) as usize;
    RAND_FLOATS[rand_index]
}

// TODO: Implement a vec3 based perlin noise