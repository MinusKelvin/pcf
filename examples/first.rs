use fumen::{ Fumen, Page };
use pcf::{ BitBoard, Piece, SearchStatus };
use rand::prelude::*;

mod common;

fn main() {
    let mut queue = [
        Piece::I, Piece::T, Piece::O, Piece::L, Piece::J, Piece::S, Piece::Z,
        Piece::I, Piece::T, Piece::O, Piece::L, Piece::J, Piece::S, Piece::Z
    ];
    queue[..7].shuffle(&mut thread_rng());
    queue[7..].shuffle(&mut thread_rng());
    println!("Solving PC for queue {:?}", &queue[..11]);

    let mut fumen = Fumen::default();
    fumen.pages.pop();
    pcf::solve_pc(&queue, BitBoard(0), true, true, pcf::placeability::tucks, |soln| {
        common::add_placement_pages(&mut fumen, BitBoard(0), soln);
        SearchStatus::Continue
    });

    use std::io::Write;
    writeln!(std::fs::File::create("solutions.txt").unwrap(), "{}", fumen.encode()).unwrap();
    println!("{} Tuck solutions have been saved to solutions.txt", fumen.pages.len());
    println!("(these fumens can be so large that you can't view it by clicking a URL)");
}