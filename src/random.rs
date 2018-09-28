//use super::rand::random;
use std::cell::RefCell;
use time::precise_time_ns;
use xorshift::{Rng, SeedableRng, Xorshift128};

thread_local! {
    static XOR_SHIFT: RefCell<Xorshift128> = {
        let now = precise_time_ns();
        let states = [now, now];
        RefCell::new(SeedableRng::from_seed(&states[..]))
    };
}

pub fn drand48() -> f32 {
//    random::<f32>()
    XOR_SHIFT.with(|rng| rng.borrow_mut().next_f32())
}
