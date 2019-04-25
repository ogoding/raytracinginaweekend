use camera::Camera;
use hitable::Hitable;
use material::Material;
use texture::Texture;

// TODO: These world and material collections should be more generic (a slice) to allow for array usage instead of always Vec
// TODO: Should also make them use a series of typed arrays/vecs instead - e.g. Map<T, [T]>
pub struct Scene {
    pub resources: Resources
}

impl Scene {
    pub fn new(resources: Resources) -> Scene {
        Scene { resources }
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

pub type EntityRef = usize;
pub type HitableRef = usize;
pub type MaterialRef = usize;
pub type TextureRef = usize;

#[derive(Debug)]
pub struct Entity {
    pub hitable_id: HitableRef,
    ptr: Box<Hitable>
}

#[derive(Debug)]
pub struct Entities {
    pub entities: Vec<Entity>,
}

impl Entities {
    pub fn new() -> Entities {
        Entities {
            entities: vec![]
        }
    }

    pub fn new_entity<T: 'static + Hitable>(&mut self, hitable: T) -> HitableRef {
        let id = self.entities.len();
        self.entities.push(Entity { hitable_id: id, ptr: Box::new(hitable) });
        id
    }

    pub fn get_entity(&self, id: EntityRef) -> &Entity {
        &self.entities[id]
    }

    pub fn get_hitable(&self, id: HitableRef) -> &Box<Hitable> {
        let entity = self.entities.iter()
            .find(|&entity| entity.hitable_id == id)
            .unwrap();
        &entity.ptr
    }

    pub fn len(&self) -> usize {
        self.entities.len()
    }
}

#[derive(Debug)]
pub struct Resources {
    // TODO: Bench/try using an arena or slotmap
    // TODO: Bench/try using a map of vecs/arenas/etc where the key == data type - e.g. Map<Hitable<T>::id, Vec<Hitable<T>> map; map.get::<Hitable<T>>(id); or map.get(id); where id includes hitable type's id
    pub entities: Entities,
    pub materials: Vec<Material>,
    pub textures: Vec<Texture>
}

unsafe impl Send for Resources {}
unsafe impl Sync for Resources {}

impl Resources {
    pub fn new() -> Resources {
        Resources {
            entities: Entities::new(),
            materials: vec![],
            textures: vec![]
        }
    }

    pub fn new_entity<T: 'static + Hitable>(&mut self, hitable: T) -> HitableRef {
        self.entities.new_entity(hitable)
    }

    pub fn new_material(&mut self, material: Material) -> MaterialRef {
        // TODO: assert that textures exist
        self.materials.push(material);
        self.materials.len() - 1
    }

    pub fn new_texture(&mut self, texture: Texture) -> TextureRef {
        self.textures.push(texture);
        self.textures.len() - 1
    }

    pub fn get_entity(&self, id: HitableRef) -> &Box<Hitable> {
        self.entities.get_hitable(id)
    }

    pub fn get_material(&self, id: MaterialRef) -> &Material {
        &self.materials[id]
    }

    pub fn get_texture(&self, id: TextureRef) -> &Texture {
        &self.textures[id]
    }
}
