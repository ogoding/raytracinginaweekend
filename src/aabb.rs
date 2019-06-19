use vec3::Vec3;

#[inline]
fn ffmin(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}

#[inline]
fn ffmax(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}

pub fn surrounding_box(box0: AABBVolume, box1: AABBVolume) -> AABBVolume {
    AABBVolume {
        min: Vec3::new(
            ffmin(box0.min.x(), box1.min.x()),
            ffmin(box0.min.y(), box1.min.y()),
            ffmin(box0.min.z(), box1.min.z()),
        ),
        max: Vec3::new(
            ffmax(box0.max.x(), box1.max.x()),
            ffmax(box0.max.y(), box1.max.y()),
            ffmax(box0.max.z(), box1.max.z()),
        ),
    }
}

// FIXME: Why does this result in fewer rays than previous approach?
// Credit to Majercik et al. - http://jcgt.org/published/0007/03/04/
#[inline(always)]
fn slabs(aabb_min: Vec3, aabb_max: Vec3, ray_origin: Vec3, ray_inv_dir: Vec3) -> bool {
    let t0 = (aabb_min - ray_origin) * ray_inv_dir;
    let t1 = (aabb_max - ray_origin) * ray_inv_dir;
    let tmin = t0.min(&t1); // vector of element wise min
    let tmax = t0.max(&t1); // vector of element wise max

    // max element in tmin <= min element in tmax
    tmin.max_component() <= tmax.min_component()
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AABBVolume {
    min: Vec3,
    max: Vec3,
}

impl AABBVolume {
    pub fn zero() -> AABBVolume {
        AABBVolume {
            min: Vec3::zero(),
            max: Vec3::zero(),
        }
    }

    pub fn new(min: Vec3, max: Vec3) -> AABBVolume {
        AABBVolume { min, max }
    }

    pub fn min(&self) -> Vec3 {
        self.min
    }

    pub fn max(&self) -> Vec3 {
        self.max
    }

    pub fn hit(&self, ray_origin: Vec3, ray_inv_dir: Vec3, _t_min: f32, _t_max: f32) -> bool {
        slabs(self.min, self.max, ray_origin, ray_inv_dir)
    }
}
