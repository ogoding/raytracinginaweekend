// TODO: Use a proper arena implementation for the Bvh Tree
// TODO: Implement in a way that lets Bvh trees contain sub-trees
// TODO: Try using Z-Order curves to sort instead of random axis

use aabb::{surrounding_box, AABBVolume};
use hitable::{HitRecord, Hitable};
use random::drand48;
use ray::Ray;

use std::cmp::Ordering;
use std::u32;
use std::usize;
use scene::Entities;

type BvhNodeIndex = u32;
const GEOMETRY_INDEX_SENTINEL: BvhNodeIndex = u32::MAX;

// TODO: Make an arena based tree that stores the node's index + child indexes - makes nodes bigger but can be more easily created and ordered
// Read this: https://rcoh.me/posts/cache-oblivious-datastructures/

//enum NodeRef {
//    Geometry(HitableRef),
//    Node(BvhNodeIndex),
//    None
//}
//
//impl NodeRef {
//    fn is_geom(&self) -> bool {
//        match self {
//            NodeRef::Geometry(_) => true,
//            _ => false
//        }
//    }
//
//    fn is_sentinel(&self) -> bool {
//        match self {
//            NodeRef::None => true,
//            _ => false
//        }
//    }
//}

#[derive(Debug, Clone, Copy)]
struct BvhNodeRef {
    // TODO: Use an enum to indicate if it's a geometry index or a node index
    index: BvhNodeIndex,
    // Start with this and then make do something smart like a geometry ref counter and check if == to number of references or something and always put geometry references on the left
    is_geometry: bool,
}

impl BvhNodeRef {
    fn new() -> BvhNodeRef {
        BvhNodeRef {
            index: GEOMETRY_INDEX_SENTINEL,
            is_geometry: false
        }
    }

    fn is_sentinel(&self) -> bool {
        self.index == GEOMETRY_INDEX_SENTINEL
    }
}

#[derive(Debug)]
struct CompactBvhNode {
    // TODO: Try giving this an id so that the Vec<CompactBvhNode> can be sorted as needed
    bbox: AABBVolume,
    left: BvhNodeRef,
    right: BvhNodeRef
}

impl CompactBvhNode {
    fn set_ref(node_ref: &mut BvhNodeRef, hitables: &mut [(usize, AABBVolume)], nodes: &mut Vec<CompactBvhNode>, t_min: f32, t_max: f32) -> AABBVolume {
        // TODO: handle/panic if hitables is empty
        if hitables.len() == 1 {
            node_ref.index = hitables[0].0 as BvhNodeIndex;
            node_ref.is_geometry = true;
            hitables[0].1
        } else {
            let index = CompactBvhNode::create_node(hitables, nodes, t_min, t_max);
            node_ref.index = index;
            node_ref.is_geometry = false;
            nodes[index as usize].bbox
        }
    }

    fn create_node(
        hitables: &mut [(usize, AABBVolume)],
        nodes: &mut Vec<CompactBvhNode>,
        t_min: f32,
        t_max: f32,
    ) -> BvhNodeIndex {
        let current_index = nodes.len() as BvhNodeIndex;
        let mut hitables = hitables;

        // sort data
        sort_entities(&mut hitables);

        nodes.push(CompactBvhNode {
            bbox: AABBVolume::zero(),
            left: BvhNodeRef::new(),
            right: BvhNodeRef::new(),
        });

        // TODO: Think up a less horrible way to do the correct rounding
        let pivot_idx = (hitables.len() as f32 / 2.0).ceil() as usize;
//        let pivot_idx = hitables.len()/ 2;
        let (left_hitables, right_hitables) = hitables.split_at_mut(pivot_idx);

        let mut left_ref = BvhNodeRef::new();
        let mut node_bbox = CompactBvhNode::set_ref(&mut left_ref, left_hitables, nodes, t_min, t_max);

        let mut right_ref = BvhNodeRef::new();
        if !right_hitables.is_empty() {
            let box_right = CompactBvhNode::set_ref(&mut right_ref, right_hitables, nodes, t_min, t_max);
            node_bbox = surrounding_box(node_bbox, box_right);
        }
        let node = &mut nodes[current_index as usize];
        node.left = left_ref;
        node.right = right_ref;
        node.bbox = node_bbox;

        current_index
    }
}

// TODO: Add a flag to indicate whether it is a geometry node or not so that geometry nodes can contain 2 geometry indexes
// Thereby reducing the number of nodes and the depth of the tree
/*
    Handle this case better (x = aggregate nodes, o = geometry nodes)
        x
       / \
      o   x
         / \
        o   o
    Should only require 2 nodes, first is geom + agg, second is geom + geom. This will reduce bvh work by only needing 2 instead of 5 nodes

         x
        / \
       /   \
      x     x
     / \   / \
    o   o o   o
    This example should only require 3 nodes (agg + agg, geom + geom, geom + geom) instead of 7 nodes

    split at ceil(len() / 2)
    [1, 2, 3]
    left: [1, 2] right: [3]
    left: (left: [1] right: [2]) right: [3]


    [1, 2, 3, 4]
    left: [1, 2] right: [3, 4]
    left: (left: [1] right: [2]) right: (left: [3] right: [4])

    New version
              x
             / \
            /   \
           /     \
          /       \
         /         \
        x           x
       / \         / \
      /   \       /   \
     x     x     x     x
    / \   / \   / \   / \
   o   o o   o o   o o   o


   Old version
              x
             / \
            /   \
           /     \
          /       \
         /         \
        x           x
       / \         / \
      /   \       /   \
     x     x     x     x
    / \   / \   / \   / \
   x   x x   x x   x x   x
   |   | |   | |   | |   |
   o   o o   o o   o o   o
*/

#[derive(Debug)]
pub struct CompactBvh {
    // TODO: Make the Bvh structure own the vec of entities within it - ???
    // This might mean that it is more dangerous/difficult to make things like Transform using arena indexes
    // as they will have to know which entity bundle contains the referenced object
    // Does this actually improve things? e.g. traversing a single bvh vs bvh of bvh trees

    // TODO: Test using a proper arena type instead of writing own version
    nodes: Vec<CompactBvhNode>,
}

impl CompactBvh {
    pub fn new(entities: &Entities, t_min: f32, t_max: f32) -> CompactBvh {
        use scene::HitableRef;
        let mut nodes = Vec::with_capacity(entities.len() * 2);

        // FIXME: Instead of effectively making a copy of the hitables list, sort the original Vec<Entity> (for mem/cache access/locality reasons)
        let mut hitables: Vec<(HitableRef, AABBVolume)> = entities.entities.iter().map(|entity| {
            let id = entity.hitable_id;
            (id, entities.get_hitable(id).bounding_box(t_min, t_max).unwrap())
        }).collect();

        {
            CompactBvhNode::create_node(
                &mut hitables,
                &mut nodes,
                t_min,
                t_max,
            );
        }

        println!("created BVH of {} nodes using list of {}", nodes.len(), hitables.len());

        CompactBvh { nodes }
    }

    fn hit_ref(&self, node_ref: BvhNodeRef, entities: &Entities, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        // TODO: Panic if the node_ref is a sentinel and not either aggregate or geometry?
        if node_ref.is_geometry {
            entities.get_hitable(node_ref.index as usize).hit_ptr(entities, ray, t_min, t_max, hit_record)
        } else {
            // Continue searching recursively
            self.hit_internal_ptr(node_ref.index, entities, ray, t_min, t_max, hit_record)
        }
    }

    fn hit_internal_ptr(
        &self,
        node_idx: BvhNodeIndex,
        entities: &Entities,
        ray: &Ray,
        t_min: f32,
        t_max: f32,
        hit_record: &mut HitRecord,
    ) -> bool {
        let node = &self.nodes[node_idx as usize];

        if node.bbox.hit(ray, t_min, t_max) {
            // FIXME: Various Bvh node/tree hits still seem to be around half the runtime of Bvh tests. Try and work out why
            // FIXME: Does not seem to work correctly. e.g. shadow is missing on sphere in simple_light scene
            if self.hit_ref(node.left, entities, ray, t_min, t_max, hit_record) {
                self.hit_ref(node.right, entities, ray, t_min, t_max, hit_record);

                return true;
            } else {
                return self.hit_ref(node.right, entities, ray, t_min, t_max, hit_record);
            }
        }

        false
    }

    pub fn hit(&self, entities: &Entities, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        self.hit_internal_ptr(0, entities, ray, t_min, t_max, hit_record)
    }
}


#[derive(Debug)]
struct BvhNode {
    bbox: AABBVolume,
    left: BvhNodeIndex,
    right: BvhNodeIndex,
}

impl BvhNode {
    fn bounding_box(&self) -> AABBVolume {
        self.bbox
    }

    fn left(&self) -> BvhNodeIndex {
        self.left
    }

    fn right(&self) -> BvhNodeIndex {
        self.right
    }

    fn geom_index(&self) -> BvhNodeIndex {
        self.left
    }

    fn is_geometry_node(&self) -> bool {
        self.right == GEOMETRY_INDEX_SENTINEL
    }

    fn create_aggregate_node(
        low_idx: BvhNodeIndex,
        high_idx: BvhNodeIndex,
        hitables: &mut [(usize, AABBVolume)],
        nodes: &mut Vec<BvhNode>,
        t_min: f32,
        t_max: f32,
    ) -> BvhNodeIndex {
        let current_index = nodes.len() as BvhNodeIndex;

        // sort data
        sort_entities(&mut hitables[low_idx as usize..high_idx as usize]);

        nodes.push(BvhNode {
            bbox: AABBVolume::zero(),
            left: 0,
            right: 0,
        });

        let pivot_idx = low_idx + (high_idx - low_idx) / 2;
        let left_node_index = BvhNode::create_node(low_idx, pivot_idx, hitables, nodes, t_min, t_max);
        let right_node_index = BvhNode::create_node(pivot_idx, high_idx, hitables, nodes, t_min, t_max);

        let box_left = nodes[left_node_index as usize].bounding_box();
        let box_right = nodes[right_node_index as usize].bounding_box();

        let node = &mut nodes[current_index as usize];
        node.bbox = surrounding_box(box_left, box_right);
        node.left = left_node_index;
        node.right = right_node_index;

        current_index
    }

    fn create_leaf_node(
        data_index: BvhNodeIndex,
        hitables: &mut [(usize, AABBVolume)],
        nodes: &mut Vec<BvhNode>,
        t_min: f32,
        t_max: f32,
    ) -> BvhNodeIndex {
        let current_index = nodes.len() as BvhNodeIndex;

        nodes.push(BvhNode {
            bbox: hitables[data_index as usize].1,
            left: hitables[data_index as usize].0 as u32,
            right: GEOMETRY_INDEX_SENTINEL,
        });

        current_index
    }

    fn create_node(
        low_index: BvhNodeIndex,
        high_index: BvhNodeIndex,
        hitables: &mut [(usize, AABBVolume)],
        nodes: &mut Vec<BvhNode>,
        t_min: f32,
        t_max: f32,
    ) -> BvhNodeIndex {
        // FIXME: Need to be able to do this in a loop instead of recursively because of stack overflowing
        if low_index == high_index || low_index + 1 == high_index {
            BvhNode::create_leaf_node(low_index, hitables, nodes, t_min, t_max)
        } else {
            BvhNode::create_aggregate_node(low_index, high_index, hitables, nodes, t_min, t_max)
        }
    }
}

fn arena_bbox_x_compare(box_1: &(usize, AABBVolume), box_2: &(usize, AABBVolume)) -> Ordering {
    box_1.1.min().x().partial_cmp(&box_2.1.min().x()).unwrap()
}

fn arena_bbox_y_compare(box_1: &(usize, AABBVolume), box_2: &(usize, AABBVolume)) -> Ordering {
    box_1.1.min().y().partial_cmp(&box_2.1.min().y()).unwrap()
}

fn arena_bbox_z_compare(box_1: &(usize, AABBVolume), box_2: &(usize, AABBVolume)) -> Ordering {
    box_1.1.min().z().partial_cmp(&box_2.1.min().z()).unwrap()
}

fn sort_entities(hitables: &mut [(usize, AABBVolume)]) {
    match drand48() as u8 * 3 {
        0 => hitables.sort_by(arena_bbox_x_compare),
        1 => hitables.sort_by(arena_bbox_y_compare),
        _ => hitables.sort_by(arena_bbox_z_compare),
    };
}

#[derive(Debug)]
pub struct Bvh {
    // TODO: Make the Bvh structure own the vec of entities within it - ???
    // This might mean that it is more dangerous/difficult to make things like Transform using arena indexes
    // as they will have to know which entity bundle contains the referenced object
    // Does this actually improve things? e.g. traversing a single bvh vs bvh of bvh trees

    // TODO: Test using a proper arena type instead of writing own version
    nodes: Vec<BvhNode>,
}

impl Bvh {
    pub fn new(entities: &Entities, t_min: f32, t_max: f32) -> Bvh {
        use scene::HitableRef;
        let mut nodes = Vec::with_capacity(entities.len() * 2);

        // FIXME: Instead of effectively making a copy of the hitables list, sort the original Vec<Entity> (for mem/cache access/locality reasons)
        let mut hitables: Vec<(HitableRef, AABBVolume)> = entities.entities.iter().map(|entity| {
            let id = entity.hitable_id;
            (id, entities.get_hitable(id).bounding_box(t_min, t_max).unwrap())
        }).collect();

        {
            BvhNode::create_node(
                0 as BvhNodeIndex,
                hitables.len() as BvhNodeIndex,
                &mut hitables,
                &mut nodes,
                t_min,
                t_max,
            );
        }

        println!("created BVH of {} nodes using list of {}", nodes.len(), hitables.len());

        Bvh { nodes }
    }

    // TODO: Implement an iterative version that won't be able to blow up the stack
    fn hit_internal_ptr(
        &self,
        node_idx: BvhNodeIndex,
        entities: &Entities,
        ray: &Ray,
        t_min: f32,
        t_max: f32,
        hit_record: &mut HitRecord,
    ) -> bool {
        let node = &self.nodes[node_idx as usize];

        if node.bbox.hit(ray, t_min, t_max) {
//            println!("hit node: {} (is_geometry {})", node_idx, node.is_geometry_node());
            if node.is_geometry_node() {
                return entities.get_hitable(node.geom_index() as usize).hit_ptr(entities, ray, t_min, t_max, hit_record);
            } else if self.hit_internal_ptr(node.left, entities, ray, t_min, t_max, hit_record) {
                self.hit_internal_ptr(
                    node.right,
                    entities,
                    ray,
                    t_min,
                    hit_record.t,
                    hit_record,
                );
                return true;
            } else {
                return self.hit_internal_ptr(
                    node.right,
                    entities,
                    ray,
                    t_min,
                    t_max,
                    hit_record,
                );
            }
        }

        false
    }

    pub fn hit(&self, entities: &Entities, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        self.hit_internal_ptr(0, entities, ray, t_min, t_max, hit_record)
    }
}

// TODO: In order to correctly implement this, Bvh must own the Hitables/Entities collection
//impl Hitable for Bvh {
//    fn hit_ptr(&self, entities: &Entities, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
//        self.hit(entities, ray, t_min, t_max, hit_record)
//    }
//
//    fn bounding_box(&self, t_min: f32, t_max: f32) -> Option<AABBVolume> {
//        if self.nodes.is_empty() {
//            None
//        } else {
//            Some(self.nodes[0].bbox)
//        }
//    }
//}
