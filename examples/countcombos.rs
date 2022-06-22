use std::sync::atomic::{AtomicU64, Ordering};

use pcf::PIECES;

fn main() {
    let pieces = PIECES.repeat(4).into_iter().collect();
    let count = AtomicU64::new(0);
    let t = std::time::Instant::now();

    let mut incr = DelayedIncrement::new(&count);
    pcf::find_combinations_mt(pieces, pcf::BitBoard(0), &Default::default(), 4, move |_| {
        incr.inc();
    });

    println!(
        "Found {} combinations in {:?}",
        count.into_inner(),
        t.elapsed()
    );
}

#[derive(Clone)]
struct DelayedIncrement<'a> {
    local: u64,
    target: &'a AtomicU64,
}

impl DelayedIncrement<'_> {
    pub fn new(target: &AtomicU64) -> DelayedIncrement {
        DelayedIncrement { local: 0, target }
    }

    pub fn inc(&mut self) {
        self.local += 1;
    }
}

impl Drop for DelayedIncrement<'_> {
    fn drop(&mut self) {
        self.target.fetch_add(self.local, Ordering::Relaxed);
    }
}
