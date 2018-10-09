use hitable::HitableList;
use material::Material;
use camera::Camera;

pub struct Scene {
    pub world: HitableList,
    pub materials: Vec<Box<Material>>
}

impl Scene {
    pub fn new(world: HitableList, materials: Vec<Box<dyn Material>>) -> Scene {
        Scene{ world, materials }
    }
}

// TODO: Remove?
unsafe impl Send for Scene {}
unsafe impl Sync for Scene {}

pub struct Window {
    pub width: u32,
    pub height: u32,
    pub samples: u32,
    pub camera: Camera
}

impl Window {
    pub fn new(width: u32, height: u32, samples: u32, camera: Camera) -> Window {
        Window{ width, height, samples, camera }
    }
}

// TODO: Remove?
unsafe impl Send for Window {}
unsafe impl Sync for Window {}
