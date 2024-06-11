use crate::{Board, Player};

pub struct Checkers {
    board: Board,
    players: [Box<dyn Player>; 2],
    current_player: usize,
}

impl Checkers {
    pub fn new(player_1: Box<dyn Player>, player_2: Box<dyn Player>) -> Self {
        Self {
            board: Board::default(),
            players: [player_1, player_2],
            current_player: 0,
        }
    }

    pub fn current_player(&self) -> &dyn Player {
        &*self.players[self.current_player]
    }

    pub fn next_player(&mut self) {
        self.current_player = 1 - self.current_player;
    }
}
