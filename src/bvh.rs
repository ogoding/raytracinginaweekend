// TODO: Try and make a Tree version with ARC<BvhNode> or something?

// TODO: Do the tree/node construction without sorting by Z-Curves and then add later


// TODO: Make this Bvh thing with the random axis sorting + middle splitting/binary search tree
// TODO: and remake with Z-Order curves later (and find out how to split/implement properly)

use ray::Ray;
use hitable::{Hitable, HitRecord, HitableList};
use aabb::{AABBVolume, surrounding_box};
use random::drand48;

use std::cmp::Ordering;

type BvhNodeIndex = usize;
type GeometryIndex = usize;

#[derive(Debug)]
enum BvhNode {
    Aggregate(AABBVolume, BvhNodeIndex, BvhNodeIndex),
    Geometry(AABBVolume, GeometryIndex)
}

// TODO: Work out whether BvhNode can be made to implement Hitable - Technically could scrap this flat Bvh and make a tree with Arcs or something
impl BvhNode {
    fn bounding_box(&self) -> Option<AABBVolume> {
        match &self {
            BvhNode::Aggregate(bbox, _, _) => Some(*bbox),
            BvhNode::Geometry(bbox, _) => Some(*bbox)
        }
    }

    fn left(&self) -> Option<BvhNodeIndex> {
        match &self {
            BvhNode::Aggregate(_, left, _) => Some(*left),
            BvhNode::Geometry(_, _) => None
        }
    }

    fn right(&self) -> Option<BvhNodeIndex> {
        match &self {
            BvhNode::Aggregate(_, _, right) => Some(*right),
            BvhNode::Geometry(_, _) => None
        }
    }

    fn geom_index(&self) -> Option<GeometryIndex> {
        match &self {
            BvhNode::Aggregate(_, _, _) => None,
            BvhNode::Geometry(_, index) => Some(*index)
        }
    }
}

//fn box_x_compare(a1: &Hitable, a2: &Hitable) -> Ordering {
fn box_x_compare(a1: &Box<dyn Hitable>, a2: &Box<dyn Hitable>) -> Ordering {
    let box_1 = a1.bounding_box(0.0, 0.0).unwrap_or(AABBVolume::zero());
    let box_2 = a2.bounding_box(0.0, 0.0).unwrap_or(AABBVolume::zero());

    box_1.min().x().partial_cmp(&box_2.min().x()).unwrap()
}

//fn box_y_compare(a1: &Hitable, a2: &Hitable) -> Ordering {
fn box_y_compare(a1: &Box<dyn Hitable>, a2: &Box<dyn Hitable>) -> Ordering {
    let box_1 = a1.bounding_box(0.0, 0.0).unwrap_or(AABBVolume::zero());
    let box_2 = a2.bounding_box(0.0, 0.0).unwrap_or(AABBVolume::zero());

    box_1.min().y().partial_cmp(&box_2.min().y()).unwrap()
}

//fn box_z_compare(a1: &Hitable, a2: &Hitable) -> Ordering {
fn box_z_compare(a1: &Box<dyn Hitable>, a2: &Box<dyn Hitable>) -> Ordering {
    let box_1 = a1.bounding_box(0.0, 0.0).unwrap_or(AABBVolume::zero());
    let box_2 = a2.bounding_box(0.0, 0.0).unwrap_or(AABBVolume::zero());

    box_1.min().z().partial_cmp(&box_2.min().z()).unwrap()
}

fn sort_data(hitables: &mut [Box<Hitable>]) {
    match drand48() as u32 * 3 {
        0 => hitables.sort_by(box_x_compare),
        1 => hitables.sort_by(box_y_compare),
        _ => hitables.sort_by(box_z_compare),
    };
}

fn create_aggregate_node(low_idx: usize, high_idx: usize, data: &mut Vec<Box<Hitable>>, nodes: &mut Vec<BvhNode>, t_min: f32, t_max: f32) -> usize {
    let current_index = nodes.len();

    // sort data
    sort_data(&mut data[low_idx..high_idx]);

    nodes.push(BvhNode::Aggregate(AABBVolume::zero(), 0, 0));

    let pivot_idx = low_idx + (high_idx - low_idx) / 2;
    let left_node_index = create_node(low_idx, pivot_idx, data, nodes, t_min, t_max);
    let right_node_index = create_node(pivot_idx, high_idx, data, nodes, t_min, t_max);

    let box_left = nodes[left_node_index].bounding_box().unwrap_or(AABBVolume::zero());
    let box_right = nodes[right_node_index].bounding_box().unwrap_or(AABBVolume::zero());
    let node = &mut nodes[current_index];
    if let &mut BvhNode::Aggregate(ref mut bbox, ref mut left, ref mut right) = node {
        *bbox = surrounding_box(box_left, box_right);
        *left = left_node_index;
        *right = right_node_index;
    };

    current_index
}

fn create_leaf_node(data_index: usize, data: &mut Vec<Box<Hitable>>, nodes: &mut Vec<BvhNode>, t_min: f32, t_max: f32) -> usize {
    let current_index = nodes.len();

    // TODO: handle the None case
    let node = BvhNode::Geometry(data[data_index].bounding_box(t_min, t_max).unwrap(), data_index);
    nodes.push(node);

    current_index
}

fn create_node(low_index: usize, high_index: usize, data: &mut Vec<Box<Hitable>>, nodes: &mut Vec<BvhNode>, t_min: f32, t_max: f32) -> usize {
    // FIXME: Need to be able to do this in a loop instead of recursively because of stack overflowing
    if low_index == high_index || low_index + 1 == high_index {
        create_leaf_node(low_index, data, nodes, t_min, t_max)
    } else {
        create_aggregate_node(low_index, high_index, data, nodes, t_min, t_max)
    }
}

pub struct Bvh {
    nodes: Vec<BvhNode>
}

impl Bvh {
    pub fn new(hitables: &mut HitableList, t_min: f32, t_max: f32) -> Bvh {
        // TODO: Take a guess at the size to reserve to speed up creation
        let mut nodes = Vec::new();

        {
            create_node(0, hitables.len(), hitables.list_as_mut(), &mut nodes, t_min, t_max);
        }

        Bvh{ nodes }
    }

    // TODO: Implement an iterative version that won't be able to blow up the stack
    fn hit_internal(&self, node_idx: BvhNodeIndex, world: &HitableList, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let node = &self.nodes[node_idx];
        let node_bbox = node.bounding_box();

        if node_bbox.is_some() && node_bbox.unwrap().hit(ray, t_min, t_max) {
            // TODO: Try and clean this up - maybe match on enum type instead of the option of left/right
            let hit_left = match node.left() {
                Some(index) => self.hit_internal(index, world, ray, t_min, t_max),
                None => world[node.geom_index().unwrap()].hit(ray, t_min, t_max)
            };
            let hit_right = match node.right() {
                Some(index) => self.hit_internal(index, world, ray, t_min, t_max),
                None => world[node.geom_index().unwrap()].hit(ray, t_min, t_max)
            };

            // Return the closest child Hitable if both are hit
            if hit_left.is_some() && hit_right.is_some() {
                let hit_left = hit_left.unwrap();
                let hit_right = hit_right.unwrap();

                if hit_left.t < hit_right.t {
                    return Some(hit_left);
                } else {
                    return Some(hit_right);
                }
            } else if hit_left.is_some() {
                return hit_left;
            } else if hit_right.is_some() {
                return hit_right;
            }
        }

        None
    }

    pub fn hit(&self, world: &HitableList, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.hit_internal(0, world, ray, t_min, t_max)
    }
}
