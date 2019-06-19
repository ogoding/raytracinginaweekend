// TODO: Use a proper arena implementation for the Bvh Tree
// TODO: Implement in a way that lets Bvh trees contain sub-trees
// TODO: Try using Z-Order curves to sort instead of random axis

use aabb::{surrounding_box, AABBVolume};
use hitable::HitRecord;
use random::drand48;
use ray::Ray;

use std::cmp::Ordering;
use std::usize;
use scene::{Entities, HitableRef};
use vec3::Vec3;

type BvhNodeIndex = usize;
// TODO: Get rid of this somehow
const GEOMETRY_INDEX_SENTINEL: BvhNodeIndex = usize::MAX;

// TODO: Make an arena based tree that stores the node's index + child indexes - makes nodes bigger but can be more easily created and ordered
// Read this: https://rcoh.me/posts/cache-oblivious-datastructures/

pub trait Accelerator {
    fn hit(&self, entities: &Entities, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool;
}

#[derive(Debug, Clone, Copy)]
enum NodeRef {
    Geometry(HitableRef),
    Aggregate(BvhNodeIndex),
    None
}

impl NodeRef {
    fn is_geom(&self) -> bool {
        match self {
            NodeRef::Geometry(_) => true,
            _ => false
        }
    }

    fn is_aggregate(&self) -> bool {
        match self {
            NodeRef::Aggregate(_) => true,
            _ => false
        }
    }

    fn is_none(&self) -> bool {
        match self {
            NodeRef::None => true,
            _ => false
        }
    }
}

#[derive(Debug)]
struct CompactBvhNode {
    // TODO: Try giving this an id so that the Vec<CompactBvhNode> can be sorted as needed
    bbox: AABBVolume,
    left: NodeRef,
    right: NodeRef
}

impl CompactBvhNode {
    fn create_ref(hitables: &mut [(usize, AABBVolume)], nodes: &mut Vec<CompactBvhNode>, t_min: f32, t_max: f32) -> (NodeRef, AABBVolume) {
        if hitables.is_empty() {
            (NodeRef::None, AABBVolume::zero())
        } else if hitables.len() == 1 {
            (NodeRef::Geometry(hitables[0].0 as HitableRef), hitables[0].1)
        } else {
            let index = CompactBvhNode::create_node(hitables, nodes, t_min, t_max);
            (NodeRef::Aggregate(index), nodes[index as usize].bbox)
        }
    }

    fn create_node(
        hitables: &mut [(usize, AABBVolume)],
        nodes: &mut Vec<CompactBvhNode>,
        t_min: f32,
        t_max: f32,
    ) -> BvhNodeIndex {
        let current_index = nodes.len() as BvhNodeIndex;
        let hitables = hitables;

        // sort data
        let axis = drand48() as u8 * 3;
//        let axis = (drand48() * 3.0) as u8;
        match axis {
            0 => hitables.sort_by(arena_bbox_x_compare),
            1 => hitables.sort_by(arena_bbox_y_compare),
            _ => hitables.sort_by(arena_bbox_z_compare),
        };

        nodes.push(CompactBvhNode {
            bbox: AABBVolume::zero(),
            left: NodeRef::None,
            right: NodeRef::None,
        });

        // TODO: Think up a less horrible way to do the correct rounding
        let pivot_idx = (hitables.len() as f32 / 2.0).ceil() as usize;
//        let pivot_idx = hitables.len()/ 2;
        let (left_hitables, right_hitables) = hitables.split_at_mut(pivot_idx);

        let (left_ref, left_bbox) = CompactBvhNode::create_ref(left_hitables, nodes, t_min, t_max);
        let (right_ref, right_bbox) = CompactBvhNode::create_ref(right_hitables, nodes, t_min, t_max);

        let node: &mut CompactBvhNode = &mut nodes[current_index as usize];
        node.left = left_ref;
        node.right = right_ref;
        // Must check whether right is valid otherwise it will attempt to make a box from 0, 0, 0 to
        // left_box instead of just around left_box. Although this generally shouldn't be a problem
        node.bbox = if right_ref.is_none() {
            left_bbox
        } else {
            surrounding_box(left_bbox, right_bbox)
        };

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

    fn hit_ref(&self, node_ref: &NodeRef, entities: &Entities, ray: &Ray, inv_ray_dir: &Vec3, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        match node_ref {
            NodeRef::Geometry(index) => entities.get_hitable(*index).hit_ptr(entities, ray, t_min, t_max, hit_record),
            NodeRef::Aggregate(index) => self.hit_internal_ptr(*index, entities, ray, inv_ray_dir, t_min, t_max, hit_record),
            NodeRef::None => unreachable!("Shouldn't be possible since the tree structure should prevent these")
        }
        // TODO: Panic if the node_ref is a sentinel and not either aggregate or geometry?
//        if node_ref.is_geometry {
//            entities.get_hitable(node_ref.index as usize).hit_ptr(entities, ray, t_min, t_max, hit_record)
//        } else {
//            // Continue searching recursively
//            self.hit_internal_ptr(node_ref.index, entities, ray, t_min, t_max, hit_record)
//        }
    }

    fn hit_internal_ptr(
        &self,
        node_idx: BvhNodeIndex,
        entities: &Entities,
        ray: &Ray,
        inv_ray_dir: &Vec3,
        t_min: f32,
        t_max: f32,
        hit_record: &mut HitRecord,
    ) -> bool {
        let node = &self.nodes[node_idx as usize];

        // FIXME: Bug is somewhere in this logic. Old commented out version works correctly, however reduced version does not
        if node.bbox.hit(ray._origin, *inv_ray_dir, t_min, t_max) {
            let mut left_rec = HitRecord::zero();
            let mut right_rec = HitRecord::zero();
            let hit_left = self.hit_ref(&node.left, entities, ray, inv_ray_dir, t_min, t_max, &mut left_rec);
            let hit_right = self.hit_ref(&node.right, entities, ray, inv_ray_dir, t_min, t_max, &mut right_rec);

            if hit_left && hit_right {
                if left_rec.t < right_rec.t {
                    *hit_record = left_rec;
                } else {
                    *hit_record = right_rec;
                }
                return true;
            } else if hit_left {
                *hit_record = left_rec;
                return true;
            } else if hit_right {
                *hit_record = right_rec;
                return true;
            }

            // FIXME: Various Bvh node/tree hits still seem to be around half the runtime of Bvh tests. Try and work out why

            // FIXME: There is something broken suspect about this code/approach (might be in hit_ref())
            // FIXME: Old version worked as there was an initial check whether it was a geometry node (and could only hold one geometry index)
//            if self.hit_ref(&node.left, entities, ray, t_min, t_max, hit_record) {
//                self.hit_ref(&node.right, entities, ray, t_min, t_max, hit_record);
//
//                return true;
//            } else {
//                return self.hit_ref(&node.right, entities, ray, t_min, t_max, hit_record);
//            }
        }

        false
    }
}

impl Accelerator for CompactBvh {
    fn hit(&self, entities: &Entities, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        self.hit_internal_ptr(0, entities, ray, &ray.inverse_direction(), t_min, t_max, hit_record)
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
        let axis = drand48() as u8 * 3;
//        let axis = (drand48() * 3.0) as u8;
        match axis {
            0 => hitables.sort_by(arena_bbox_x_compare),
            1 => hitables.sort_by(arena_bbox_y_compare),
            _ => hitables.sort_by(arena_bbox_z_compare),
        };

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

        let node: &mut BvhNode = &mut nodes[current_index as usize];
        node.bbox = surrounding_box(box_left, box_right);
        node.left = left_node_index;
        node.right = right_node_index;

        current_index
    }

    fn create_leaf_node(
        data_index: BvhNodeIndex,
        hitables: &mut [(usize, AABBVolume)],
        nodes: &mut Vec<BvhNode>,
    ) -> BvhNodeIndex {
        let current_index = nodes.len() as BvhNodeIndex;

        nodes.push(BvhNode {
            bbox: hitables[data_index as usize].1,
            left: hitables[data_index as usize].0,
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
        // FIXME: Need to be able to do this in a loop instead of recursively because of potential stack overflowing
        if low_index == high_index || low_index + 1 == high_index {
            BvhNode::create_leaf_node(low_index, hitables, nodes)
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
    fn hit_internal_ptr( &self,
                         node_idx: BvhNodeIndex,
                         entities: &Entities,
                         ray: &Ray,
                         inv_ray_dir: &Vec3,
                         t_min: f32,
                         t_max: f32,
                         hit_record: &mut HitRecord,
    ) -> bool {
        let node: &BvhNode = &self.nodes[node_idx as usize];

        if node.bbox.hit(ray._origin, *inv_ray_dir, t_min, t_max) {
            if node.is_geometry_node() {
                return entities.get_hitable(node.geom_index() as usize).hit_ptr(entities, ray, t_min, t_max, hit_record);
            } else if self.hit_internal_ptr(node.left, entities, ray, inv_ray_dir, t_min, t_max, hit_record) {
                self.hit_internal_ptr(
                    node.right,
                    entities,
                    ray,
                    inv_ray_dir,
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
                    inv_ray_dir,
                    t_min,
                    t_max,
                    hit_record,
                );
            }
        }

        false
    }
}

impl Accelerator for Bvh {
    fn hit(&self, entities: &Entities, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        self.hit_internal_ptr(0, entities, ray, &ray.inverse_direction(), t_min, t_max, hit_record)
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