use crate::game::{Direction, Game, Grid};

#[derive(Debug, Clone, Copy)]
pub struct Weights {
    pub max_tile: f32,
    pub adjacent_tiles: f32,
    pub empty_cells: f32,
    pub monotonicity: f32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct WeightsKey {
    pub max_tile: u32,
    pub adjacent_tiles: u32,
    pub empty_cells: u32,
    pub monotonicity: u32,
}

impl WeightsKey {
    pub fn from_weights(weights: Weights) -> WeightsKey {
        WeightsKey {
            max_tile: (weights.max_tile * 10.0) as u32,
            adjacent_tiles: (weights.adjacent_tiles * 10.0) as u32,
            empty_cells: (weights.empty_cells * 10.0) as u32,
            monotonicity: (weights.monotonicity * 10.0) as u32,
        }
    }
}

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

    fn get_best_move(&self) -> Direction {
        let mut best_score = 0;
        let mut best_direction = Direction::Up; // Default move

        for &direction in &[
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            if self.can_move(direction) {
                let test_game = self.current_game_state.make_move(direction);
                let score = self.evaluate_board(test_game.board());

                if score > best_score {
                    best_score = score;
                    best_direction = direction;
                }
            }
        }

        best_direction
    }

    fn can_move(&self, direction: Direction) -> bool {
        let test_game = self.current_game_state.make_move(direction);
        test_game.board() != self.current_game_state.board()
    }

    fn evaluate_board(&self, board: Grid) -> u32 {
        let mut max_tile_score = 0;
        let mut adjacent_tiles_score = 0;
        let mut empty_cells_score = 0;
        let mut monotonicity_score = 0;

        // Iterate over each cell in the grid
        for i in 0..4 {
            for j in 0..4 {
                let tile_value = board[i][j] as u32;

                // max tile: check for the highest tile value
                max_tile_score = max_tile_score.max(tile_value);

                // empty cells: check for empty cells
                if tile_value == 0 {
                    empty_cells_score += 1;
                }

                // adjacent tiles: check for adjacent tiles with the same value
                if i > 0 && tile_value == board[i - 1][j] as u32 {
                    adjacent_tiles_score += tile_value as u32;
                }
                if j > 0 && tile_value == board[i][j - 1] as u32 {
                    adjacent_tiles_score += tile_value as u32;
                }

                // Monotonicity: check for smooth increases or decreases in tile values
                if i > 0 {
                    let vertical_difference = tile_value as i32 - board[i - 1][j] as i32;
                    monotonicity_score += vertical_difference.abs() as u32;
                }
                if j > 0 {
                    let horizontal_difference = tile_value as i32 - board[i][j - 1] as i32;
                    monotonicity_score += horizontal_difference.abs() as u32;
                }
            }
        }

        // Calculate the final score using weights
        let final_score = (adjacent_tiles_score as f32 * self.weights.adjacent_tiles)
            + (max_tile_score as f32 * self.weights.max_tile)
            + (empty_cells_score as f32 * self.weights.empty_cells)
            + (monotonicity_score as f32 * self.weights.monotonicity);

        // Convert the final score to u32, rounding down
        final_score as u32
    }
}
