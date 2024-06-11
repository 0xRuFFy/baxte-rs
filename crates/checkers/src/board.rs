use crate::piece::{Piece, PieceColor};
use std::collections::HashMap;

const WHITE_DEFAULT_START: u64 =
    0b00000000_00000000_00000000_00000000_00000000_10101010_01010101_10101010;
const BLACK_DEFAULT_START: u64 =
    0b01010101_10101010_01010101_10101010_00000000_00000000_00000000_00000000;

pub struct Move {
    from: usize,
    to: usize,
}

impl Move {
    pub fn new(from_x: usize, from_y: usize, to_x: usize, to_y: usize) -> Self {
        Self {
            from: BitBoard::coord_to_index(from_x, from_y),
            to: BitBoard::coord_to_index(to_x, to_y),
        }
    }
}

pub struct BitBoard {
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

    pub fn get_color_board(&self, color: PieceColor) -> u64 {
        match color {
            PieceColor::White => self.white,
            PieceColor::Black => self.black,
        }
    }

    pub fn get_kings(&self) -> u64 {
        self.kings
    }

    #[inline]
    fn coord_to_index(x: usize, y: usize) -> usize {
        x + y * 8
    }

    #[inline]
    fn index_to_coord(index: usize) -> (usize, usize) {
        (index % 8, index / 8)
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

    pub fn is_king(&self, x: usize, y: usize) -> bool {
        let mask = Self::mask(x, y);

        self.kings & mask != 0
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

    fn get_valid_moves_for(&self, x: usize, y: usize) -> Vec<Move> {
        unimplemented!("Board::get_valid_moves_for")
    }

    pub fn get_valid_moves(&self, color: PieceColor) -> HashMap<(usize, usize), Vec<Move>> {
        let mut color_board = self.bit_board.get_color_board(color);
        let mut moves = HashMap::new();

        while color_board != 0 {
            let piece_index = color_board.trailing_zeros() as usize;
            let (x, y) = BitBoard::index_to_coord(piece_index);

            let piece_moves = self.get_valid_moves_for(x, y);
            moves.insert((x, y), piece_moves);

            color_board &= !(1 << piece_index);
        }

        moves
    }

    pub fn apply_move(&mut self, mv: Move) {
        unimplemented!("Board::apply_move")
    }

    pub fn backend(&self) -> &BitBoard {
        &self.bit_board
    }
}
