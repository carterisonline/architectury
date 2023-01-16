use std::time::Instant;

use architectury::gt::{await_threads, go, num_threads};
use architectury::prelude::*;

const NUM_GREEN_THREADS: usize = 10000;

fn allocate_a_lot(n: i32) -> Vec<u128> {
    let mut prev = 0;
    let mut curr = 1u128;
    let mut out = vec![];

    for _ in 0..n {
        let lprev = prev;
        prev = curr;
        curr = curr.wrapping_add(lprev);

        out.push(curr);
    }

    out
}

fn main() {
    architectury::init();

    let start_time = Instant::now();

    for _ in 0..NUM_GREEN_THREADS {
        go(|| {
            allocate_a_lot(1000);
        });
    }

    await_threads();

    info!(
        "[Green Threads] Took {}ms",
        start_time.elapsed().as_millis()
    );
    info!("... on {} physical threads", num_threads());
    info!("... spawned {} green threads", NUM_GREEN_THREADS);

    let start_time = Instant::now();

    for _ in 0..NUM_GREEN_THREADS {
        allocate_a_lot(1000);
    }

    info!("[Normal] Took {}ms", start_time.elapsed().as_millis());
}
