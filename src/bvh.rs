// TODO: Try and make a Tree version with ARC<BvhNode> or something?

// TODO: Do the tree/node construction without sorting by Z-Curves and then add later


// TODO: Make this Bvh thing with the random axis sorting + middle splitting/binary search tree
// TODO: and remake with Z-Order curves later (and find out how to split/implement properly)

use ray::Ray;
use hitable::{Hitable, HitRecord, HitableList};
use aabb::{AABBVolume, surrounding_box};
use random::drand48;

use std::cmp::Ordering;
use std::usize;
use std::u32;

// NOTE: If it looks like I need to run this for really big BVH trees,
// might have to either make collections of bvh or turn the left/right
// fields into index offsets or even just up it back to u64
type BvhNodeIndex = u32;
const GEOMETRY_INDEX_SENTINEL: BvhNodeIndex = u32::MAX;

#[derive(Debug)]
struct BvhNode {
    bbox: AABBVolume,
    left: BvhNodeIndex,
    right: BvhNodeIndex
}

// TODO: Work out whether BvhNode can be made to implement Hitable - Technically could scrap this flat Bvh and make a tree with Arcs or something
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

    fn create_aggregate_node(low_idx: BvhNodeIndex, high_idx: BvhNodeIndex, data: &mut Vec<Box<Hitable>>, nodes: &mut Vec<BvhNode>, t_min: f32, t_max: f32) -> BvhNodeIndex {
        let current_index = nodes.len() as BvhNodeIndex;

        // sort data
        sort_data(&mut data[low_idx as usize..high_idx as usize]);

        nodes.push(BvhNode { bbox: AABBVolume::zero(), left: 0, right: 0 });

        let pivot_idx = low_idx + (high_idx - low_idx) / 2;
        let left_node_index = BvhNode::create_node(low_idx, pivot_idx, data, nodes, t_min, t_max);
        let right_node_index = BvhNode::create_node(pivot_idx, high_idx, data, nodes, t_min, t_max);

        let box_left = nodes[left_node_index as usize].bounding_box();
        let box_right = nodes[right_node_index as usize].bounding_box();

        let node = &mut nodes[current_index as usize];
        node.bbox = surrounding_box(box_left, box_right);
        node.left = left_node_index;
        node.right = right_node_index;

        current_index
    }

    fn create_leaf_node(data_index: BvhNodeIndex, data: &mut Vec<Box<Hitable>>, nodes: &mut Vec<BvhNode>, t_min: f32, t_max: f32) -> BvhNodeIndex {
        let current_index = nodes.len() as BvhNodeIndex;

        // TODO: handle the None case
        nodes.push(BvhNode { bbox: data[data_index as usize].bounding_box(t_min, t_max).unwrap(), left: data_index, right: GEOMETRY_INDEX_SENTINEL });

        current_index
    }

    fn create_node(low_index: BvhNodeIndex, high_index: BvhNodeIndex, data: &mut Vec<Box<Hitable>>, nodes: &mut Vec<BvhNode>, t_min: f32, t_max: f32) -> BvhNodeIndex {
        // FIXME: Need to be able to do this in a loop instead of recursively because of stack overflowing
        if low_index == high_index || low_index + 1 == high_index {
            BvhNode::create_leaf_node(low_index, data, nodes, t_min, t_max)
        } else {
            BvhNode::create_aggregate_node(low_index, high_index, data, nodes, t_min, t_max)
        }
    }
}

fn box_x_compare(a1: &Box<dyn Hitable>, a2: &Box<dyn Hitable>) -> Ordering {
    let box_1 = a1.bounding_box(0.0, 0.0).unwrap_or(AABBVolume::zero());
    let box_2 = a2.bounding_box(0.0, 0.0).unwrap_or(AABBVolume::zero());

    box_1.min().x().partial_cmp(&box_2.min().x()).unwrap()
}

fn box_y_compare(a1: &Box<dyn Hitable>, a2: &Box<dyn Hitable>) -> Ordering {
    let box_1 = a1.bounding_box(0.0, 0.0).unwrap_or(AABBVolume::zero());
    let box_2 = a2.bounding_box(0.0, 0.0).unwrap_or(AABBVolume::zero());

    box_1.min().y().partial_cmp(&box_2.min().y()).unwrap()
}

fn box_z_compare(a1: &Box<dyn Hitable>, a2: &Box<dyn Hitable>) -> Ordering {
    let box_1 = a1.bounding_box(0.0, 0.0).unwrap_or(AABBVolume::zero());
    let box_2 = a2.bounding_box(0.0, 0.0).unwrap_or(AABBVolume::zero());

    box_1.min().z().partial_cmp(&box_2.min().z()).unwrap()
}

fn sort_data(hitables: &mut [Box<Hitable>]) {
    match drand48() as u8 * 3 {
        0 => hitables.sort_by(box_x_compare),
        1 => hitables.sort_by(box_y_compare),
        _ => hitables.sort_by(box_z_compare),
    };
}

pub struct Bvh {
    nodes: Vec<BvhNode>
}

impl Bvh {
    pub fn new(hitables: &mut HitableList, t_min: f32, t_max: f32) -> Bvh {
        let mut nodes = Vec::with_capacity(hitables.len() * 2);

        {
            BvhNode::create_node(0 as BvhNodeIndex, hitables.len() as BvhNodeIndex, hitables.list_as_mut(), &mut nodes, t_min, t_max);
        }

        println!("created BVH of {} nodes using list of {}", nodes.len(), hitables.len());

        Bvh{ nodes }
    }

    // TODO: Implement an iterative version that won't be able to blow up the stack
    fn hit_internal_ptr(&self, node_idx: BvhNodeIndex, world: &HitableList, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let node = &self.nodes[node_idx as usize];

        if node.bounding_box().hit(ray, t_min, t_max) {
            let mut hit_rec_left = HitRecord::zero();
            let mut hit_rec_right = HitRecord::zero();

            let hit_left = if node.is_geometry_node() {
                world[node.geom_index() as usize].hit_ptr(ray, t_min, t_max, &mut hit_rec_left)
            } else {
                self.hit_internal_ptr(node.left(), world, ray, t_min, t_max, &mut hit_rec_left)
            };
            let hit_right = if node.is_geometry_node() {
                false
            } else {
                self.hit_internal_ptr(node.right(), world, ray, t_min, t_max, &mut hit_rec_right)
            };

            // Return the closest child Hitable if both are hit
            if hit_left && hit_right {
                if hit_rec_left.t < hit_rec_right.t {
                    *hit_record = hit_rec_left;
                } else {
                    *hit_record = hit_rec_right;
                }
                return true;
            } else if hit_left {
                *hit_record = hit_rec_left;
                return true;
            } else if hit_right {
                *hit_record = hit_rec_right;
                return true;
            }
        }

        false
    }

    pub fn hit(&self, world: &HitableList, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_record = HitRecord::zero();
        if self.hit_internal_ptr(0, world, ray, t_min, t_max, &mut hit_record) {
            Some(hit_record)
        } else {
            None
        }
    }
}
