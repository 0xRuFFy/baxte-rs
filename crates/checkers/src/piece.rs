use std::fmt::Display;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Default, PartialEq, Eq, Clone, Copy, Debug)]
pub enum PieceType {
    #[default]
    Man,
    King,
}

pub struct Piece {
    color: PieceColor,
    piece_type: PieceType,
}

impl Piece {
    pub fn white() -> Self {
        Self {
            color: PieceColor::White,
            piece_type: PieceType::default(),
        }
    }

    pub fn black() -> Self {
        Self {
            color: PieceColor::Black,
            piece_type: PieceType::default(),
        }
    }

    pub fn promote(&mut self) {
        self.piece_type = PieceType::King;
    }

    pub fn is_king(&self) -> bool {
        self.piece_type == PieceType::King
    }

    pub fn color(&self) -> PieceColor {
        self.color
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match (self.color, self.piece_type) {
            (PieceColor::White, PieceType::Man) => 'w',
            (PieceColor::Black, PieceType::Man) => 'b',
            (PieceColor::White, PieceType::King) => 'W',
            (PieceColor::Black, PieceType::King) => 'B',
        })
    }
}
