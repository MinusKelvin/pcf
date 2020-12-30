use fumen::{ Fumen, Page };
use pcf::{ BitBoard, Piece, placeability };
use rand::prelude::*;
use std::sync::atomic::AtomicBool;

mod common;

fn main() {
    let mut queue = [
        Piece::I, Piece::T, Piece::O, Piece::L, Piece::J, Piece::S, Piece::Z,
        Piece::I, Piece::T, Piece::O, Piece::L, Piece::J, Piece::S, Piece::Z
    ];
    queue[..7].shuffle(&mut thread_rng());
    queue[7..].shuffle(&mut thread_rng());
    println!("Solving PC for queue {:?}", &queue[..14]);

    let mut fumen = Fumen::default();
    fumen.pages.pop();

    let (send, recv) = std::sync::mpsc::channel();
    let mut fumen = SendOnDrop::new(send, fumen);
    let t = std::time::Instant::now();
    let b = BitBoard(0b1000000000_1000000000_1000000000_1000000000_1100000000_1100000000);
    pcf::solve_pc_mt(
        &queue, b, true, true, &AtomicBool::new(false), placeability::hard_drop_only,
        move |soln| common::add_placement_pages(&mut fumen, b, soln)
    );
    println!("Done in {:?}.", t.elapsed());

    let mut fumen = Fumen::default();
    fumen.pages.pop();
    for mut subfumen in recv {
        fumen.pages.append(&mut subfumen.pages);
    }

    use std::io::Write;
    writeln!(std::fs::File::create("solutions.txt").unwrap(), "{}", fumen.encode()).unwrap();
    println!("{} solutions have been saved to solutions.txt", fumen.pages.len() / 10);
    println!("(these fumens can be so large that you can't view it by clicking a URL)");
}

#[derive(Clone)]
struct SendOnDrop<T>(std::sync::mpsc::Sender<T>, Option<T>);

impl<T> SendOnDrop<T> {
    pub fn new(sender: std::sync::mpsc::Sender<T>, v: T) -> Self {
        SendOnDrop(sender, Some(v))
    }
}

impl<T> std::ops::Deref for SendOnDrop<T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.1.as_ref().unwrap()
    }
}

impl<T> std::ops::DerefMut for SendOnDrop<T> {
    fn deref_mut(&mut self) -> &mut T {
        self.1.as_mut().unwrap()
    }
}

impl<T> Drop for SendOnDrop<T> {
    fn drop(&mut self) {
        self.0.send(self.1.take().unwrap()).ok();
    }
}