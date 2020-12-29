use std::fs::File;
use std::path::Path;
use std::io::Write;
use std::env;

use arrayvec::ArrayVec;

fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=build.rs");

    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("data.rs");
    let mut file = File::create(&path)?;

    writeln!(file, "#[allow(unused)]")?;
    writeln!(file, "mod data {{")?;
    writeln!(file, "    use crate::*;")?;

    let mut states = vec![];

    gen_piece_data(
        &mut states, [(0, 0), (1, 0), (1, 1), (2, 1)], "SHorizontal",
        &[("North", 1, 0), ("South", 1, 1)]
    );
    gen_piece_data(
        &mut states, [(1, 0), (1, 1), (0, 1), (0, 2)], "SVertical",
        &[("West", 1, 1), ("East", 0, 1)]
    );
    gen_piece_data(
        &mut states, [(0, 1), (1, 1), (1, 0), (2, 0)], "ZHorizontal",
        &[("North", 1, 0), ("South", 1, 1)]
    );
    gen_piece_data(
        &mut states, [(0, 0), (0, 1), (1, 1), (1, 2)], "ZVertical",
        &[("West", 1, 1), ("East", 0, 1)]
    );
    gen_piece_data(
        &mut states, [(0, 0), (1, 0), (2, 0), (3, 0)], "IHorizontal",
        &[("North", 1, 0), ("South", 2, 0)]
    );
    gen_piece_data(
        &mut states, [(0, 0), (0, 1), (0, 2), (0, 3)], "IVertical",
        &[("West", 0, 1), ("East", 0, 2)]
    );
    gen_piece_data(
        &mut states, [(0, 0), (1, 0), (2, 0), (0, 1)], "JNorth",
        &[("North", 1, 0)]
    );
    gen_piece_data(
        &mut states, [(0, 0), (1, 0), (2, 0), (1, 1)], "TNorth",
        &[("North", 1, 0)]
    );
    gen_piece_data(
        &mut states, [(0, 0), (1, 0), (2, 0), (2, 1)], "LNorth",
        &[("North", 1, 0)]
    );
    gen_piece_data(
        &mut states, [(0, 1), (1, 1), (2, 1), (2, 0)], "JSouth",
        &[("South", 1, 1)]
    );
    gen_piece_data(
        &mut states, [(0, 1), (1, 1), (2, 1), (1, 0)], "TSouth",
        &[("South", 1, 1)]
    );
    gen_piece_data(
        &mut states, [(0, 1), (1, 1), (2, 1), (0, 0)], "LSouth",
        &[("South", 1, 1)]
    );
    gen_piece_data(
        &mut states, [(0, 0), (0, 1), (0, 2), (1, 2)], "JEast",
        &[("East", 0, 1)]
    );
    gen_piece_data(
        &mut states, [(0, 0), (0, 1), (0, 2), (1, 1)], "TEast",
        &[("East", 0, 1)]
    );
    gen_piece_data(
        &mut states, [(0, 0), (0, 1), (0, 2), (1, 0)], "LEast",
        &[("East", 0, 1)]
    );
    gen_piece_data(
        &mut states, [(1, 0), (1, 1), (1, 2), (0, 0)], "JWest",
        &[("West", 1, 1)]
    );
    gen_piece_data(
        &mut states, [(1, 0), (1, 1), (1, 2), (0, 1)], "TWest",
        &[("West", 1, 1)]
    );
    gen_piece_data(
        &mut states, [(1, 0), (1, 1), (1, 2), (0, 2)], "LWest",
        &[("West", 1, 1)]
    );
    gen_piece_data(
        &mut states, [(0, 0), (1, 0), (0, 1), (1, 1)], "O",
        &[("North", 0, 0), ("East", 0, 1), ("South", 1, 1), ("West", 1, 0)]
    );

    let mut piece_state_enum =
        "#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)] pub enum PieceState {".to_owned();
    let mut piece_bits = format!("const PIECE_BITS: &'static [u64; {}] = &[", states.len());
    let mut piece_widths = format!("const PIECE_WIDTHS: &'static [u8; {}] = &[", states.len());
    let mut piece_hurdles = format!("const PIECE_HURDLES: &'static [u8; {}] = &[", states.len());
    let mut piece_below = format!("const PIECE_BELOW: &'static [u64; {}] = &[", states.len());
    let mut piece_harddrop = format!("const PIECE_HARDDROP: &'static [u64; {}] = &[", states.len());
    let mut piece_y = format!(
        "const PIECE_Y: &'static [u8; {}] = &[", states.len()
    );
    let mut piece_kinds = format!("const PIECE_KINDS: &'static [Piece; {}] = &[", states.len());
    let mut piece_srs = format!(
        "const PIECE_SRS: &'static [&'static [SrsPiece]; {}] = &[", states.len()
    );

    let mut heights_and_piece: [[String; 7]; 6] = {
        let mut a1 = ArrayVec::new();
        a1.extend(std::iter::repeat(String::new()));
        let mut a = ArrayVec::new();
        a.extend(std::iter::repeat(a1.into_inner().unwrap()));
        a.into_inner().unwrap()
    };

    for data in &states {
        piece_state_enum.push_str(&data.name);
        piece_state_enum.push(',');
        piece_kinds.push_str("Piece::");
        piece_kinds.push(data.name.chars().next().unwrap());
        piece_kinds.push(',');
        piece_bits.push_str(&format!("{},", data.bitboard));
        piece_widths.push_str(&format!("{},", data.width));
        piece_hurdles.push_str(&format!("{},", data.hurdles));
        piece_below.push_str(&format!("{},", data.below));
        piece_harddrop.push_str(&format!("{},", data.harddrop));
        piece_y.push_str(&format!("{},", data.y));
        piece_srs.push_str("&[");
        for (rot, x, y) in &data.srs {
            piece_srs.push_str(&format!("SrsPiece {{
                piece: Piece::{},
                rotation: Rotation::{},
                x: {}, y: {}
            }},", data.name.chars().next().unwrap(), rot, x, y));
        }
        piece_srs.push_str("],");

        for i in data.height as usize - 1 .. 6 {
            let s = &mut heights_and_piece[i][piece_index(data.name.chars().next().unwrap())];
            s.push_str("PieceState::");
            s.push_str(&data.name);
            s.push(',');
        }
    }

    writeln!(file, "pub const PIECE_STATES_FOR_HEIGHT_AND_PIECE: &'static [[&'static [PieceState]; 7]; 6] = &[")?;
    for h in &heights_and_piece {
        writeln!(file, "[")?;
        for v in h {
            writeln!(file, "&[{}],", v)?;
        }
        writeln!(file, "],")?;
    }
    writeln!(file, "];")?;

    writeln!(file,
        "{}}}\n{}];\n{}];\n{}];\n{}];\n{}];\n{}];\n{}];\n{}];",
        piece_state_enum, piece_bits, piece_hurdles, piece_widths, piece_below, piece_harddrop,
        piece_y, piece_kinds, piece_srs
    )?;

    writeln!(file, "{}", stringify! {
        impl PieceState {
            #[inline]
            pub fn board(self) -> crate::BitBoard {
                BitBoard(PIECE_BITS[self as usize])
            }

            #[inline]
            pub fn width(self) -> u8 {
                PIECE_WIDTHS[self as usize]
            }

            #[inline]
            pub fn hurdles(self) -> u8 {
                PIECE_HURDLES[self as usize]
            }

            #[inline]
            pub fn below_mask(self) -> BitBoard {
                BitBoard(PIECE_BELOW[self as usize])
            }

            #[inline]
            pub fn harddrop_mask(self) -> BitBoard {
                BitBoard(PIECE_HARDDROP[self as usize])
            }

            #[inline]
            pub fn y(self) -> u8 {
                PIECE_Y[self as usize]
            }

            #[inline]
            pub fn piece(self) -> Piece {
                PIECE_KINDS[self as usize]
            }

            #[inline]
            pub fn piece_srs(self) -> &'static [SrsPiece] {
                PIECE_SRS[self as usize]
            }
        }
    })?;

    writeln!(file, "}}")
}

fn piece_index(name: char) -> usize {
    match name {
        'S' => 0,
        'Z' => 1,
        'J' => 2,
        'L' => 3,
        'T' => 4,
        'O' => 5,
        'I' => 6,
        _ => unreachable!("invalid piece: {:?}", name)
    }
}

fn gen_piece_data(
    data: &mut Vec<PieceData>,
    cells: [(u32, u32); 4],
    name: &str,
    rots: &[(&'static str, i32, u32)]
) {
    let mut h = 0;
    let mut w = 0;
    let mut bits = [0; 4];
    for &(x, y) in &cells {
        h = h.max(y+1);
        w = w.max(x+1);
        bits[y as usize] |= 1 << x;
    }

    rec_gen_piece_data(data, bits, w, h, name, &mut vec![], rots);
}

fn rec_gen_piece_data(
    data: &mut Vec<PieceData>, bits: [u16; 4], w: u32, h: u32, name: &str, offsets: &mut Vec<u32>,
    rots: &[(&'static str, i32, u32)]
) {
    if offsets.len() as u32 == h {
        let mut name = name.to_owned();
        let mut hurdles = 0;
        let mut row = 0u32;
        let mut bitboard = 0;
        let mut below = 0;
        let mut hdrow = 0;
        let mut hdrop = 0;
        for (i, &o) in offsets.iter().enumerate() {
            name.push_str(&o.to_string());
            bitboard |= (bits[i] as u64) << 10 * (row+o);
            hdrow |= bits[i] as u64;
            hdrop |= hdrow << 10*(row+o);
            if row != 0 {
                below |= (bits[i] as u64) << 10 * (row-1);
            }
            if i == 0 && o != 0 {
                below |= (bits[i] as u64) << 10 * (o-1);
            }
            if i != 0 {
                for j in 0..o {
                    hurdles |= 1 << row + j;
                }
            }
            row += o + 1;
        }
        for i in row..6 {
            hdrop |= hdrow << 10*i;
        }
        data.push(PieceData {
            name,
            y: offsets[0],
            hurdles,
            height: row,
            bitboard,
            harddrop: hdrop,
            below,
            width: w,
            srs: rots.iter().map(|&(r, x, y)| (r, x, y+offsets[0])).collect()
        });
        return;
    }
    let remaining = 6 - offsets.iter().copied().sum::<u32>() - h;
    for offset in 0..=remaining {
        offsets.push(offset);
        rec_gen_piece_data(data, bits, w, h, name, offsets, rots);
        offsets.pop();
    }
}

#[derive(Debug)]
struct PieceData {
    name: String,
    hurdles: u32,
    y: u32,
    width: u32,
    height: u32,
    bitboard: u64,
    below: u64,
    harddrop: u64,
    srs: Vec<(&'static str, i32, u32)>
}