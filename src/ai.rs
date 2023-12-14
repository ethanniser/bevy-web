use crate::game::{Game, Grid, Turn};

#[derive(Debug, Clone, Copy)]
pub struct Weights;

pub struct AI {
    pub current_game_state: Game,
    pub weights: Weights,
}

impl AI {
    pub fn new(game: Game, weights: Weights) -> AI {
        AI {
            current_game_state: game,
            weights,
        }
    }

    pub fn run_to_completion(&mut self) {
        while !self.current_game_state.is_game_over() {
            self.current_game_state = self.current_game_state.make_move(self.get_best_move());
        }
    }

    pub fn get_best_move(&self) -> usize {
        todo!()
    }

    fn evaluate_board(&self, board: Grid, turn: Turn) -> u32 {
        todo!()
    }
}
