use crate::*;

pub fn always(_: BitBoard, _: Placement) -> bool {
    true
}

pub fn hard_drop_only(board: BitBoard, placement: Placement) -> bool {
    !board.overlaps(placement.harddrop_mask())
}