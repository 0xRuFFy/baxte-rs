use crate::board::Move;
use crate::Board;

pub trait Player {
    fn make_move(&self, board: &Board) -> Move;
}
