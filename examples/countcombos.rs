use std::sync::atomic::{AtomicU64, Ordering};

use pcf::PIECES;

fn main() {
    let pieces = PIECES.repeat(4).into_iter().collect();
    let count = AtomicU64::new(0);
    let t = std::time::Instant::now();

    pcf::find_combinations_mt(pieces, pcf::BitBoard(0), &Default::default(), 4, |_| {
        count.fetch_add(1, Ordering::Relaxed);
    });

    println!(
        "Found {} combinations in {:?}",
        count.into_inner(),
        t.elapsed()
    );
}
