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
        entities: &Entities,
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
        let left_node_index = BvhNode::create_node(low_idx, pivot_idx, hitables, entities, nodes, t_min, t_max);
        let right_node_index = BvhNode::create_node(pivot_idx, high_idx, hitables, entities, nodes, t_min, t_max);

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
        entities: &Entities,
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
        entities: &Entities,
        nodes: &mut Vec<BvhNode>,
        t_min: f32,
        t_max: f32,
    ) -> BvhNodeIndex {
        // FIXME: Need to be able to do this in a loop instead of recursively because of stack overflowing
        if low_index == high_index || low_index + 1 == high_index {
            BvhNode::create_leaf_node(low_index, hitables, entities, nodes, t_min, t_max)
        } else {
            BvhNode::create_aggregate_node(low_index, high_index, hitables, entities, nodes, t_min, t_max)
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
    // TODO: Test using a proper arena type instead of writing own version
    // TODO: Store the height of the tree in order to build a non recursive version of hit_internal_ptr - could leave create_node logic as recursive
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
                entities,
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

        // FIXME: Somehow I broke lighting by doing the Resources/bvh changes

        if node.bounding_box().hit(ray, t_min, t_max) {
            if node.is_geometry_node() {
                return entities.get_hitable(node.geom_index() as usize).hit_ptr(entities, ray, t_min, t_max, hit_record);
            } else if self.hit_internal_ptr(node.left(), entities, ray, t_min, t_max, hit_record) {
                self.hit_internal_ptr(
                    node.right(),
                    entities,
                    ray,
                    t_min,
                    hit_record.t,
                    hit_record,
                );
                return true;
            } else {
                return self.hit_internal_ptr(
                    node.right(),
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
