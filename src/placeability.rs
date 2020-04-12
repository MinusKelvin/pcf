use crate::*;

pub fn always(_: BitBoard, _: Placement) -> bool {
    true
}

pub fn hard_drop_only(board: BitBoard, placement: Placement) -> bool {
    !board.overlaps(placement.harddrop_mask())
}

pub fn tucks(board: BitBoard, placement: Placement) -> bool {
    for x in placement.x ..= 10-placement.kind.width() {
        let placement = Placement { x, ..placement };
        if board.overlaps(placement.board()) {
            break
        }
        if hard_drop_only(board, placement) {
            return true;
        }
    }
    for x in (0..placement.x).rev() {
        let placement = Placement { x, ..placement };
        if board.overlaps(placement.board()) {
            break
        }
        if hard_drop_only(board, placement) {
            return true;
        }
    }
    return false;
}