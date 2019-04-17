// TODO: Use a proper arena implementation for the Bvh Tree
// TODO: Implement in a way that lets Bvh trees contain sub-trees
// TODO: Try using Z-Order curves to sort instead of random axis

use aabb::{surrounding_box, AABBVolume};
use hitable::{HitRecord, Hitable, HitableList};
use random::drand48;
use ray::Ray;

use std::cmp::Ordering;
use std::u32;
use std::usize;

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

    fn create_aggregate_node(
        low_idx: BvhNodeIndex,
        high_idx: BvhNodeIndex,
        data: &mut Vec<Box<Hitable>>,
        nodes: &mut Vec<BvhNode>,
        t_min: f32,
        t_max: f32,
    ) -> BvhNodeIndex {
        let current_index = nodes.len() as BvhNodeIndex;

        // sort data
        sort_data(&mut data[low_idx as usize..high_idx as usize]);

        nodes.push(BvhNode {
            bbox: AABBVolume::zero(),
            left: 0,
            right: 0,
        });

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

    fn create_leaf_node(
        data_index: BvhNodeIndex,
        data: &mut Vec<Box<Hitable>>,
        nodes: &mut Vec<BvhNode>,
        t_min: f32,
        t_max: f32,
    ) -> BvhNodeIndex {
        let current_index = nodes.len() as BvhNodeIndex;

        nodes.push(BvhNode {
            bbox: data[data_index as usize]
                .bounding_box(t_min, t_max)
                .unwrap(),
            left: data_index,
            right: GEOMETRY_INDEX_SENTINEL,
        });

        current_index
    }

    fn create_node(
        low_index: BvhNodeIndex,
        high_index: BvhNodeIndex,
        data: &mut Vec<Box<Hitable>>,
        nodes: &mut Vec<BvhNode>,
        t_min: f32,
        t_max: f32,
    ) -> BvhNodeIndex {
        // FIXME: Need to be able to do this in a loop instead of recursively because of stack overflowing
        if low_index == high_index || low_index + 1 == high_index {
            BvhNode::create_leaf_node(low_index, data, nodes, t_min, t_max)
        } else {
            BvhNode::create_aggregate_node(low_index, high_index, data, nodes, t_min, t_max)
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

fn box_x_compare(a1: &Box<dyn Hitable>, a2: &Box<dyn Hitable>) -> Ordering {
    let box_1 = a1.bounding_box(0.0, 0.0).unwrap_or_else(AABBVolume::zero);
    let box_2 = a2.bounding_box(0.0, 0.0).unwrap_or_else(AABBVolume::zero);

    box_1.min().x().partial_cmp(&box_2.min().x()).unwrap()
}

fn box_y_compare(a1: &Box<dyn Hitable>, a2: &Box<dyn Hitable>) -> Ordering {
    let box_1 = a1.bounding_box(0.0, 0.0).unwrap_or_else(AABBVolume::zero);
    let box_2 = a2.bounding_box(0.0, 0.0).unwrap_or_else(AABBVolume::zero);

    box_1.min().y().partial_cmp(&box_2.min().y()).unwrap()
}

fn box_z_compare(a1: &Box<dyn Hitable>, a2: &Box<dyn Hitable>) -> Ordering {
    let box_1 = a1.bounding_box(0.0, 0.0).unwrap_or_else(AABBVolume::zero);
    let box_2 = a2.bounding_box(0.0, 0.0).unwrap_or_else(AABBVolume::zero);

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
    // TODO: Test using a proper arena type instead of writing own version
    // TODO: Store the height of the tree in order to build a non recursive version of hit_internal_ptr - could leave create_node logic as recursive
    nodes: Vec<BvhNode>,
}

impl Bvh {
    pub fn new(hitables: &mut HitableList, t_min: f32, t_max: f32) -> Bvh {
        let mut nodes = Vec::with_capacity(hitables.len() * 2);

        {
            BvhNode::create_node(
                0 as BvhNodeIndex,
                hitables.len() as BvhNodeIndex,
                hitables.list_as_mut(),
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
        world: &HitableList,
        ray: &Ray,
        t_min: f32,
        t_max: f32,
        hit_record: &mut HitRecord,
    ) -> bool {
        let node = &self.nodes[node_idx as usize];

        if node.bounding_box().hit(ray, t_min, t_max) {
            // TODO: Re order to handling the aggregate node case first?
            if node.is_geometry_node() {
                return world[node.geom_index() as usize].hit_ptr(ray, t_min, t_max, hit_record);
            } else if self.hit_internal_ptr(node.left(), world, ray, t_min, t_max, hit_record) {
                self.hit_internal_ptr(
                    node.right(),
                    world,
                    ray,
                    t_min,
                    hit_record.t,
                    hit_record,
                );
                return true;
            } else {
                return self.hit_internal_ptr(
                    node.right(),
                    world,
                    ray,
                    t_min,
                    t_max,
                    hit_record,
                );
            }
        }

        false
    }

//    pub fn hit_loop(
//        &self,
//        world: &Vec<Box<Hitable,
//        ray: &Ray,
//        t_min: f32,
//        t_max: f32,
//        hit_record: &mut HitRecord,
//    ) -> bool {
//        // TODO: Estimate the required size of this stack
//        let mut node_stack: Vec<BvhNodeIndex> = Vec::with_capacity(10);
//        let mut current_node = &self.nodes[0];
//        let mut latest_hit = NodeHitTest {
//            record: HitRecord::zero(),
//            was_geometry: false,
//        };
//
//        if current_node.bounding_box().hit(ray, t_min, t_max) {
//            if current_node.is_geometry_node() {
//                return world[current_node.geom_index() as usize]
//                    .hit_ptr(ray, t_min, t_max, hit_record);
//            } else {
//                node_stack.push(current_node.right);
//                node_stack.push(current_node.left);
//            }
//        } else {
//            return false;
//        }
//
//        while !node_stack.is_empty() {
//            //            let node_to_test = node_stack.pop().unwrap();
//            //            current_node = &self.nodes[node_to_test as usize];
//            current_node = &self.nodes[node_stack.pop().unwrap() as usize];
//
//            if current_node.bounding_box().hit(ray, t_min, t_max) {
//                if current_node.is_geometry_node() {
//                    let mut record = HitRecord::zero();
//                    if world[current_node.geom_index() as usize].hit_ptr(
//                        ray,
//                        t_min,
//                        t_max,
//                        &mut record,
//                    ) && latest_hit.record.t < record.t
//                    {
//                        latest_hit.record = record;
//                        latest_hit.was_geometry = true;
//                    }
//                } else {
//                    node_stack.push(current_node.right);
//                    node_stack.push(current_node.left);
//                }
//            }
//        }
//
//        // TODO: Determine whether to return whether there was a hit here?
//        if latest_hit.was_geometry {
//            *hit_record = latest_hit.record;
//            true
//        } else {
//            false
//        }
//    }

    // FIXME: Fix this so that a bvh can contain a sub bvh - unsure how this will work with rust ownership
    pub fn hit(&self, world: &HitableList, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_record = HitRecord::zero();
        // TODO: Profile iterative hit (hit_loop) and see if it can be as fast or faster than recursive version
        if self.hit_internal_ptr(0, world, ray, t_min, t_max, &mut hit_record) {
            //        if self.hit_loop(world, ray, t_min, t_max, &mut hit_record) {
            Some(hit_record)
        } else {
            None
        }
    }
}

//#[derive(Debug, Clone, Copy)]
//struct NodeHitTest {
//    record: HitRecord,
//    was_geometry: bool,
//}
