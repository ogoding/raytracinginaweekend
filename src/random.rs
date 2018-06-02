use super::rand;

pub fn drand48() -> f32 {
    rand::random::<f32>()
}
