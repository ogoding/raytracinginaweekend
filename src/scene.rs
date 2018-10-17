use hitable::HitableList;
use material::Material;
use camera::Camera;

// TODO: These world and material collections should be more generic (a slice) to allow for array usage instead of always Vec
// TODO: Should also make them use a series of typed arrays/vecs instead - e.g. Map<T, [T]>
pub struct Scene {
    pub world: HitableList,
    pub materials: Vec<Box<Material>>
}

impl Scene {
    // TODO: Change the fn signature(s) to take in a Vec<Box<Hitable>> and use these functions to determine the hitable container
    pub fn new(world: HitableList, materials: Vec<Box<dyn Material>>) -> Scene {
        Scene{ world, materials }
    }
}

// This will be properly safe when Scene fields are no longer public and have methods for gaining immutable access are created
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

unsafe impl Send for Window {}
unsafe impl Sync for Window {}
