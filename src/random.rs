use std::cell::Cell;
use time::precise_time_ns;
use xorshift::{Rng, SeedableRng};
use xorshift::xoroshiro128::Xoroshiro128;

thread_local! {
    static XOR_SHIFT: Cell<Xoroshiro128> = {
        let now = precise_time_ns();
        let states = [now, now];
        Cell::new(SeedableRng::from_seed(&states[..]))
    };
}

pub fn drand48() -> f32 {
    XOR_SHIFT.with(|rng| {
        let mut new_rng = rng.get();
        let result = new_rng.next_f32();
        rng.set(new_rng);
        result
    })
}

pub fn drand48_2() -> [f32; 2] {
    XOR_SHIFT.with(|rng| {
        let mut new_rng = rng.get();
        let result = [new_rng.next_f32(), new_rng.next_f32()];
        rng.set(new_rng);
        result
    })
}

pub fn drand48_3() -> [f32; 3] {
    XOR_SHIFT.with(|rng| {
        let mut new_rng = rng.get();
        let result = [new_rng.next_f32(), new_rng.next_f32(), new_rng.next_f32()];
        rng.set(new_rng);
        result
    })
}
