use crate::game::{CellState, Game, Grid, Turn};

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

    pub fn get_best_move(&self) -> usize {
        let depth = 1; // You can adjust this value based on the desired difficulty

        let (best_score, best_move) = self.minimax(&self.current_game_state, depth, true);
        best_move
    }

    fn minimax(&self, game: &Game, depth: usize, is_maximizing_player: bool) -> (u32, usize) {
        if depth == 0 || game.is_game_over() {
            println!(
                "Evaluating board: \n{game:?} - score: {}",
                self.evaluate_board(game.board(), game.next_turn()),
            );
            return (self.evaluate_board(game.board(), game.next_turn()), 0);
        }

        let mut best_move = 0;
        if is_maximizing_player {
            let mut max_eval = u32::MIN;
            for &move_index in game.get_available_moves().iter() {
                let mut new_game = game.make_move(move_index);
                let (eval, _) = self.minimax(&new_game, depth - 1, false);
                if eval > max_eval {
                    max_eval = eval;
                    best_move = move_index;
                }
            }
            (max_eval, best_move)
        } else {
            let mut min_eval = u32::MAX;
            for &move_index in game.get_available_moves().iter() {
                let mut new_game = game.make_move(move_index);
                let (eval, _) = self.minimax(&new_game, depth - 1, true);
                if eval < min_eval {
                    min_eval = eval;
                    best_move = move_index;
                }
            }
            (min_eval, best_move)
        }
    }

    fn evaluate_board(&self, board: Grid, turn: Turn) -> u32 {
        let mut score = 0;

        // Check each row
        for row in &board {
            score += self.evaluate_line(row, turn);
        }

        // Check each column
        for col in 0..7 {
            let mut column = [CellState::Empty; 6];
            for (row_idx, row) in board.iter().enumerate() {
                column[row_idx] = row[col];
            }
            score += self.evaluate_line(&column, turn);
        }

        // Check diagonals (only those long enough for a win)
        score += self.evaluate_diagonals(&board, turn);

        score
    }

    fn evaluate_line(&self, line: &[CellState], turn: Turn) -> u32 {
        let mut score = 0;
        let mut count = 0;
        let player_cell = match turn {
            Turn::PlayerOne => CellState::PlayerOne,
            Turn::PlayerTwo => CellState::PlayerTwo,
        };

        for &cell in line {
            if cell == player_cell {
                count += 1;
            } else {
                count = 0;
            }
            if count >= 3 {
                score += 10; // Assign a score for three in a row
            }
        }

        score
    }

    fn evaluate_diagonals(&self, board: &Grid, turn: Turn) -> u32 {
        let mut score = 0;

        // Diagonals from top-left to bottom-right
        for row in 0..3 {
            // Only need to start from the first 3 rows for diagonals of length 4
            for col in 0..4 {
                // Similarly, start from the first 4 columns
                score += self.evaluate_diagonal(board, row, col, 1, 1, turn);
            }
        }

        // Diagonals from bottom-left to top-right
        for row in 3..6 {
            // Start from the bottom 3 rows
            for col in 0..4 {
                // Start from the first 4 columns
                score += self.evaluate_diagonal(board, row, col, -1, 1, turn);
            }
        }

        score
    }

    fn evaluate_diagonal(
        &self,
        board: &Grid,
        start_row: usize,
        start_col: usize,
        row_inc: isize,
        col_inc: isize,
        turn: Turn,
    ) -> u32 {
        let mut score = 0;
        let mut count = 0;
        let player_cell = match turn {
            Turn::PlayerOne => CellState::PlayerOne,
            Turn::PlayerTwo => CellState::PlayerTwo,
        };

        let (mut row, mut col) = (start_row as isize, start_col as isize);

        while row >= 0 && row < 6 as isize && col >= 0 && col < 7 as isize {
            if board[row as usize][col as usize] == player_cell {
                count += 1;
            } else {
                count = 0;
            }
            if count >= 3 {
                score += 10; // Assign a score for three in a row
            }

            row += row_inc;
            col += col_inc;
        }

        score
    }
}
