use crate::piece::{Piece, PieceColor};

const WHITE_DEFAULT_START: u64 =
    0b00000000_00000000_00000000_00000000_00000000_10101010_01010101_10101010;
const BLACK_DEFAULT_START: u64 =
    0b01010101_10101010_01010101_10101010_00000000_00000000_00000000_00000000;

pub struct Move {
    from_x: usize,
    from_y: usize,
    to_x: usize,
    to_y: usize,
}

impl Move {
    pub fn new(from_x: usize, from_y: usize, to_x: usize, to_y: usize) -> Self {
        Self {
            from_x,
            from_y,
            to_x,
            to_y,
        }
    }
}

struct BitBoard {
    white: u64,
    black: u64,
    kings: u64,
}

impl BitBoard {
    fn new() -> Self {
        Self {
            white: WHITE_DEFAULT_START,
            black: BLACK_DEFAULT_START,
            kings: 0,
        }
    }

    #[inline]
    fn coord_to_index(x: usize, y: usize) -> usize {
        x + y * 8
    }

    #[inline]
    fn mask(x: usize, y: usize) -> u64 {
        1 << Self::coord_to_index(x, y)
    }

    pub fn get_piece(&self, x: usize, y: usize) -> Option<Piece> {
        let mask = Self::mask(x, y);

        let mut piece = if self.white & mask != 0 {
            Piece::white()
        } else if self.black & mask != 0 {
            Piece::black()
        } else {
            return None;
        };

        if self.kings & mask != 0 {
            piece.promote();
        }

        Some(piece)
    }

    pub fn set_piece(&mut self, x: usize, y: usize, piece: Piece) {
        let mask = Self::mask(x, y);

        match piece.color() {
            PieceColor::White => {
                self.white |= mask;
                self.black &= !mask;
            }
            PieceColor::Black => {
                self.black |= mask;
                self.white &= !mask;
            }
        }

        if piece.is_king() {
            self.kings |= mask;
        } else {
            self.kings &= !mask;
        }
    }

    pub fn remove_piece(&mut self, x: usize, y: usize) {
        let mask = Self::mask(x, y);

        self.white &= !mask;
        self.black &= !mask;
        self.kings &= !mask;
    }

    pub fn is_free(&self, x: usize, y: usize) -> bool {
        let mask = Self::mask(x, y);

        self.white & mask == 0 && self.black & mask == 0
    }
}

pub struct Board {
    bit_board: BitBoard,
}

impl Board {
    pub fn new() -> Self {
        Self {
            bit_board: BitBoard::new(),
        }
    }

    pub fn get_valid_moves(&self, color: PieceColor) -> Vec<Move> {
        unimplemented!("Board::get_valid_moves")
    }

    pub fn apply_move(&mut self, mv: Move) {
        unimplemented!("Board::apply_move")
    }
}
