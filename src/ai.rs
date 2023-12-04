use crate::game::{Direction, Game};

pub struct AI {
    pub current_game_state: Game,
}

impl AI {
    pub fn new(game: Game) -> AI {
        AI {
            current_game_state: game,
        }
    }

    pub fn run_to_completion(&mut self) {
        while !self.current_game_state.is_game_over() {
            self.current_game_state = self.current_game_state.make_move(self.get_best_move());
        }
    }

    fn get_best_move(&self) -> Direction {
        if self.can_move(Direction::Right) {
            Direction::Right
        } else if self.can_move(Direction::Up) {
            Direction::Up
        } else if self.can_move(Direction::Left) {
            Direction::Left
        } else {
            Direction::Down
        }
    }

    fn can_move(&self, direction: Direction) -> bool {
        let test_game = self.current_game_state.make_move(direction);
        test_game.board() != self.current_game_state.board()
    }
}
