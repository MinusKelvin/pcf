use crate::*;

pub fn always(_: BitBoard, _: Placement) -> bool {
    true
}

pub fn hard_drop_only(mut board: BitBoard, placement: Placement) -> bool {
    for y in 0..6 {
        if board.line_filled(y) {
            board.0 &= !((1 << 10) - 1 << 10 * y);
        }
    }
    !board.overlaps(placement.harddrop_mask())
}

pub fn tucks(board: BitBoard, placement: Placement) -> bool {
    for x in placement.x..=10 - placement.kind.width() {
        let placement = Placement { x, ..placement };
        if board.overlaps(placement.board()) {
            break;
        }
        if hard_drop_only(board, placement) {
            return true;
        }
    }
    for x in (0..placement.x).rev() {
        let placement = Placement { x, ..placement };
        if board.overlaps(placement.board()) {
            break;
        }
        if hard_drop_only(board, placement) {
            return true;
        }
    }
    return false;
}

pub fn simple_srs_spins(board: BitBoard, placement: Placement) -> bool {
    if tucks(board, placement) {
        return true;
    }

    let piece = placement.srs_piece(board).into_iter().next().unwrap();
    let board = board.lines_cleared();
    let x = placement.x as usize;
    let y = piece.y as usize;

    let check_empty = |mask: u64| board.0 & mask << 10 * y + x == 0;
    // vertical offset so we can check for empty cells below the placement
    let check_empty_v =
        |mask: u64, off: usize| y >= off && board.0 & mask << 10 * (y - off) + x == 0;

    // this is a visible description of all the spins we're detecting:
    // https://fumen.zui.jp/?v115@pgxhHexhIewhReA8cevEn9gwhIexhlenpfpgQaAewh?GeQaAewhGeRawhGeRaAeA8FeAAceflf+gwhIexhkenpuEBU?9UTASIB5DjB98AQWrrDTG98AXO98AwyjXEroo2AseirDFbE?cEoe0TAyE88AQzgeEFbMwDv3STASorJEvwh1DIhRaAAGeA8?beaquAAIhxhkeyufIhRaGeA8AAAeA8ZeaqfIhxhkeyuf+gR?aHeQ4QaGeAABeAAZealf+gxhIewhkeipf+gRaGeA8AAQaA8?jealf+gxhIewhkeipf/gQaHewhQakeelf/gwhIewhkempfH?hAAAeQaAAFeA8BeA8ZedqfJhwhIewhae1ufIhQaJeQaaetp?fIhwhIewhbeVvfIhQaHeAAQaAeAAZetpfIhwhlelpfIhQaJ?ewhae9pfpgwhAeQaGewhAeQaGewhAeQaGewhAeQaIeQaae9?pfIhwhlelpfIhQaHewhcetpfHhQaIeQace6pfHhxhSewhRe?ypfHhRaSeQaRe6pfHhxhSewhReipfpgQaAewhGeQaAewhGe?QaAewhGeRawhIewhHeQaReqpfIhwhJeQaHexhQeipfIhQaJ?ewhHeRaQeqpfIhwhQaIeQaHexhQeypfrgQaIeQaHeQpQaHe?QpIeAtIeQpQaQeAAe+gwSIewSHeAtAeBtGewSReAAeqgQaw?hHeQawhHeQaAtHeQaAtGeBPAeAPGeQaAtQeAAe/gwSIewSG?eBtAeAtHewSQeAAe
    // the cyan blocks are the areas check_empty calls check, the gray blocks are blocks
    // that we check to make sure are filled
    match (piece.piece, piece.rotation) {
        (Piece::S, Rotation::North) => {
            (check_empty(0b_0000000011_0000000011_0000000011_0000000011_0000000000_0000000000))
                || (check_empty(
                    0b_0000000110_0000000110_0000000110_0000000110_0000000000_0000000000,
                ) && (placement.x == 7 || board.cell_filled(x + 3, y + 2)))
        }
        (Piece::Z, Rotation::North) => {
            (check_empty(0b_0000000110_0000000110_0000000110_0000000110_0000000000_0000000000))
                || (check_empty(
                    0b_0000000011_0000000011_0000000011_0000000011_0000000000_0000000000,
                ) && (placement.x == 0 || board.cell_filled(x - 1, y + 2)))
        }
        (Piece::L, Rotation::North) => {
            (check_empty_v(
                0b_0000000110_0000000110_0000000110_0000000110_0000000110_0000000110,
                1,
            )) || (check_empty_v(
                0b_0000000011_0000000011_0000000011_0000000011_0000000010_0000000010,
                1,
            )) || (check_empty(
                0b_0000000110_0000000110_0000000110_0000000110_0000000000_0000000000,
            ) && (board.cell_filled(x + 1, y + 1)
                || board.cell_filled(x, y + 1) && (x == 7 || board.cell_filled(x + 3, y + 1))))
        }
        (Piece::J, Rotation::North) => {
            (check_empty_v(
                0b_0000000011_0000000011_0000000011_0000000011_0000000011_0000000011,
                1,
            )) || (check_empty_v(
                0b_0000000110_0000000110_0000000110_0000000110_0000000010_0000000010,
                1,
            )) || (check_empty(
                0b_0000000011_0000000011_0000000011_0000000011_0000000000_0000000000,
            ) && (board.cell_filled(x + 1, y + 1)
                || board.cell_filled(x + 2, y + 1) && (x == 0 || board.cell_filled(x - 1, y + 1))))
        }
        (Piece::L, Rotation::South) => {
            (check_empty(0b_0000000110_0000000110_0000000110_0000000110_0000000110_0000000110))
                || (check_empty(
                    0b_0000000011_0000000011_0000000011_0000000011_0000000010_0000000010,
                ))
                || (check_empty(
                    0b_0000000110_0000000110_0000000110_0000000110_0000000100_0000000000,
                ) && (board.cell_filled(x + 1, y + 1)
                    || board.cell_filled(x, y + 1) && (x == 7 || board.cell_filled(x + 3, y + 1))))
                || (check_empty(
                    0b_0000000011_0000000011_0000000011_0000000011_0000000011_0000000000,
                ) && board.cell_filled(x + 2, y + 1)
                    && (x == 0 || board.cell_filled(x - 1, y + 1)))
        }
        (Piece::J, Rotation::South) => {
            (check_empty(0b_0000000011_0000000011_0000000011_0000000011_0000000011_0000000011))
                || (check_empty(
                    0b_0000000110_0000000110_0000000110_0000000110_0000000010_0000000010,
                ))
                || (check_empty(
                    0b_0000000011_0000000011_0000000011_0000000011_0000000001_0000000000,
                ) && (board.cell_filled(x + 1, y + 1)
                    || board.cell_filled(x + 2, y + 1)
                        && (x == 0 || board.cell_filled(x - 1, y + 1))))
                || (check_empty(
                    0b_0000000110_0000000110_0000000110_0000000110_0000000110_0000000000,
                ) && board.cell_filled(x, y + 1)
                    && (x == 7 || board.cell_filled(x + 3, y + 1)))
        }
        (Piece::T, Rotation::North) => {
            (check_empty_v(
                0b_0000000011_0000000011_0000000011_0000000011_0000000011_0000000010,
                1,
            )) || (check_empty_v(
                0b_0000000110_0000000110_0000000110_0000000110_0000000110_0000000010,
                1,
            )) || (check_empty(
                0b_0000000110_0000000110_0000000110_0000000110_0000000110_0000000110,
            ) && board.cell_filled(x, y + 1)
                && (x == 7 || board.cell_filled(x + 3, y + 1)))
                || (check_empty(
                    0b_0000000011_0000000011_0000000011_0000000011_0000000011_0000000011,
                ) && board.cell_filled(x + 2, y + 1)
                    && (x == 0 || board.cell_filled(x - 1, y + 1)))
        }
        (Piece::T, Rotation::West) => {
            x != 8
                && check_empty(0b_0000000110_0000000110_0000000110_0000000110_0000000110_0000000110)
        }
        (Piece::T, Rotation::East) => {
            x != 0
                && check_empty(
                    0b_0000000011_0000000011_0000000011_0000000011_0000000011_0000000011 >> 1,
                )
                && !board.cell_filled(x - 1, y)
            // due to jank we need to check that last bit manually
        }
        (Piece::T, Rotation::South) => {
            (check_empty(0b_0000000011_0000000011_0000000011_0000000011_0000000011_0000000011))
                || (check_empty(
                    0b_0000000110_0000000110_0000000110_0000000110_0000000110_0000000110,
                ))
        }
        (Piece::I, Rotation::North) => {
            (check_empty_v(
                0b_0000000010_0000000010_0000000010_0000000010_0000000010_0000000010,
                1,
            )) || (check_empty_v(
                0b_0000000100_0000000100_0000000100_0000000100_0000000100_0000000100,
                1,
            ))
        }
        _ => false,
    }
}
