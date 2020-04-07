use fumen::{ Fumen, Page };
use pcf::{ BitBoard, Piece, solve::solve_pc, placeability };
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
    for soln in solve_pc(&queue, BitBoard(0), true, true, placeability::hard_drop_only) {
        let mut page = Page::default();
        common::draw_placements(&mut page, &soln);
        fumen.pages.push(page);
    }

    use std::io::Write;
    writeln!(std::fs::File::create("solutions.txt").unwrap(), "{}", fumen.encode()).unwrap();
    println!("{} Hard-drop-only solutions have been saved to solutions.txt", fumen.pages.len());
    println!("(these fumens can be so large that you can't view it by clicking a URL)");
}