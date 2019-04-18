use camera::Camera;
use hitable::HitableList;
use material::Material;

// TODO: Come up with a struct for "owning" both entity list/arena and the BVH/acceleration structure
// TODO: Find out whether the crates.io arenas contain sync/send

// TODO: These world and material collections should be more generic (a slice) to allow for array usage instead of always Vec
// TODO: Should also make them use a series of typed arrays/vecs instead - e.g. Map<T, [T]>
pub struct Scene {
    pub world: HitableList,
    pub materials: Vec<Box<Material>>,
//    pub resources: Resources
    // TODO: Make 3 collections, entities/hitables, materials, textures - use arenas(?) and work out a more efficient way to access (e.g. list of typed lists)
}

impl Scene {
    // TODO: Change the fn signature(s) to take in a Vec<Box<Hitable>> and use these functions to determine the hitable container
//    pub fn new(world: HitableList, resources: Resources) -> Scene {
//        Scene { world, resources }
//    }
    pub fn new(world: HitableList, materials: Vec<Box<dyn Material>>) -> Scene {
        Scene { world, materials }
    }
}

// This will be properly safe when Scene fields are no longer public and have methods for gaining immutable access are created
unsafe impl Send for Scene {}
unsafe impl Sync for Scene {}

#[derive(Debug)]
pub struct Window {
    pub width: u32,
    pub height: u32,
    pub samples: u32,
    pub camera: Camera,
}

impl Window {
    pub fn new(width: u32, height: u32, samples: u32, camera: Camera) -> Window {
        Window {
            width,
            height,
            samples,
            camera,
        }
    }
}

unsafe impl Send for Window {}
unsafe impl Sync for Window {}


//use hitable::Hitable;
use material::MaterialEnum;
use texture::TextureEnum;

//type HitableRef = usize;
type MaterialRef = usize;
type TextureRef = usize;

#[derive(Debug)]
pub struct Resources {
    // TODO: Bench/try using an arena or slotmap
    // TODO: Bench/try using a map of vecs/arenas/etc where the key == data type - e.g. Map<Hitable<T>::id, Vec<Hitable<T>> map; map.get::<Hitable<T>>(id); or map.get(id); where id includes hitable type's id
//    pub entities: Vec<Box<Hitable>>,
    pub materials: Vec<MaterialEnum>,
    pub textures: Vec<TextureEnum>
}

unsafe impl Send for Resources {}
unsafe impl Sync for Resources {}

impl Resources {
    pub fn new() -> Resources {
        Resources {
            materials: vec![],
            textures: vec![]
        }
    }
//    pub fn new_entity(&mut self, entity: Box<Hitable>) -> HitableRef {
//        // TODO: assert that material exists and relevant textures?
//        self.entities.push(entity);
//        self.entities.len() - 1
//    }

    pub fn new_material(&mut self, material: MaterialEnum) -> MaterialRef {
        // TODO: assert that textures exist
        self.materials.push(material);
        self.materials.len() - 1
    }

    pub fn new_texture(&mut self, texture: TextureEnum) -> TextureRef {
        self.textures.push(texture);
        self.textures.len() - 1
    }

//    pub fn get_entity(&self, id: HitableRef) -> &Hitable {
//        &self.entities[id]
//    }

    pub fn get_material(&self, id: MaterialRef) -> &MaterialEnum {
        &self.materials[id]
    }

    pub fn get_texture(&self, id: TextureRef) -> &TextureEnum {
        &self.textures[id]
    }

    // TODO: Implement an exists fn? or just make the get return Option<&T>
}
