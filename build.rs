use std::fs::File;
use std::path::Path;
use std::io::Write;
use std::env;

fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=build.rs");

    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("data.rs");
    let mut file = File::create(&path)?;

    writeln!(file, "#[allow(unused)]")?;
    writeln!(file, "mod data {{")?;
    writeln!(file, "    use crate::*;")?;

    let mut states = vec![];

    gen_piece_data(&mut states, [(0, 0), (1, 0), (1, 1), (2, 1)], "SHorizontal");
    gen_piece_data(&mut states, [(1, 0), (1, 1), (0, 1), (0, 2)], "SVertical");
    gen_piece_data(&mut states, [(0, 1), (1, 1), (1, 0), (2, 0)], "ZHorizontal");
    gen_piece_data(&mut states, [(0, 0), (0, 1), (1, 1), (1, 2)], "ZVertical");
    gen_piece_data(&mut states, [(0, 0), (1, 0), (2, 0), (3, 0)], "IHorizontal");
    gen_piece_data(&mut states, [(0, 0), (0, 1), (0, 2), (0, 3)], "IVertical");
    gen_piece_data(&mut states, [(0, 0), (1, 0), (2, 0), (0, 1)], "JNorth");
    gen_piece_data(&mut states, [(0, 0), (1, 0), (2, 0), (1, 1)], "TNorth");
    gen_piece_data(&mut states, [(0, 0), (1, 0), (2, 0), (2, 1)], "LNorth");
    gen_piece_data(&mut states, [(0, 1), (1, 1), (2, 1), (2, 0)], "JSouth");
    gen_piece_data(&mut states, [(0, 1), (1, 1), (2, 1), (1, 0)], "TSouth");
    gen_piece_data(&mut states, [(0, 1), (1, 1), (2, 1), (0, 0)], "LSouth");
    gen_piece_data(&mut states, [(0, 0), (0, 1), (0, 2), (1, 2)], "JEast");
    gen_piece_data(&mut states, [(0, 0), (0, 1), (0, 2), (1, 1)], "TEast");
    gen_piece_data(&mut states, [(0, 0), (0, 1), (0, 2), (1, 0)], "LEast");
    gen_piece_data(&mut states, [(1, 0), (1, 1), (1, 2), (0, 0)], "JWest");
    gen_piece_data(&mut states, [(1, 0), (1, 1), (1, 2), (0, 1)], "TWest");
    gen_piece_data(&mut states, [(1, 0), (1, 1), (1, 2), (0, 2)], "LWest");
    gen_piece_data(&mut states, [(0, 0), (1, 0), (0, 1), (1, 1)], "O");

    let mut piece_state_enum =
        "#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)] pub enum PieceState {".to_owned();
    let mut piece_bits = format!("const PIECE_BITS: &'static [u64; {}] = &[", states.len());
    let mut piece_widths = format!("const PIECE_WIDTHS: &'static [u8; {}] = &[", states.len());
    let mut piece_hurdles = format!("const PIECE_HURDLES: &'static [u8; {}] = &[", states.len());
    let mut piece_below = format!("const PIECE_BELOW: &'static [u64; {}] = &[", states.len());
    let mut piece_harddrop = format!("const PIECE_HARDDROP: &'static [u64; {}] = &[", states.len());
    let mut piece_grounded = format!("const PIECE_GROUNDED: &'static [bool; {}] = &[", states.len());
    let mut piece_kinds = format!("const PIECE_KINDS: &'static [Piece; {}] = &[", states.len());

    let mut heights = [
        String::new(), String::new(), String::new(),
        String::new(), String::new(), String::new(),
    ];

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
        piece_grounded.push_str(&format!("{},", data.grounded));

        for i in data.height as usize - 1 .. 6 {
            heights[i].push_str("PieceState::");
            heights[i].push_str(&data.name);
            heights[i].push(',');
        }
    }

    writeln!(file, "pub const PIECE_STATES_FOR_HEIGHT: &'static [&'static [PieceState]; 6] = &[")?;
    for h in &heights {
        writeln!(file, "&[{}],", h)?;
    }
    writeln!(file, "];")?;

    writeln!(file,
        "{}}} {}]; {}]; {}]; {}]; {}]; {}]; {}];",
        piece_state_enum, piece_bits, piece_hurdles, piece_widths, piece_below, piece_harddrop,
        piece_grounded, piece_kinds
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
            pub fn grounded(self) -> bool {
                PIECE_GROUNDED[self as usize]
            }

            #[inline]
            pub fn piece(self) -> Piece {
                PIECE_KINDS[self as usize]
            }
        }
    })?;

    writeln!(file, "}}")
}

fn gen_piece_data(data: &mut Vec<PieceData>, cells: [(u32, u32); 4], name: &str) {
    let mut h = 0;
    let mut w = 0;
    let mut bits = [0; 4];
    for &(x, y) in &cells {
        h = h.max(y+1);
        w = w.max(x+1);
        bits[y as usize] |= 1 << x;
    }

    rec_gen_piece_data(data, bits, w, h, name, &mut vec![]);
}

fn rec_gen_piece_data(
    data: &mut Vec<PieceData>, bits: [u16; 4], w: u32, h: u32, name: &str, offsets: &mut Vec<u32>
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
            grounded: offsets[0] == 0,
            hurdles,
            height: row,
            bitboard,
            harddrop: hdrop,
            below,
            width: w
        });
        return;
    }
    let remaining = 6 - offsets.iter().copied().sum::<u32>() - h;
    for offset in 0..=remaining {
        offsets.push(offset);
        rec_gen_piece_data(data, bits, w, h, name, offsets);
        offsets.pop();
    }
}

#[derive(Debug)]
struct PieceData {
    name: String,
    hurdles: u32,
    grounded: bool,
    width: u32,
    height: u32,
    bitboard: u64,
    below: u64,
    harddrop: u64
}