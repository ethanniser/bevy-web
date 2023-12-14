use rand::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CellState {
    Empty,
    PlayerOne,
    PlayerTwo,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Turn {
    PlayerOne,
    PlayerTwo,
}

pub enum GameResult {
    PlayerOneWin,
    PlayerTwoWin,
    Draw,
}

pub type Grid = [[CellState; 7]; 6];

#[derive(Clone)]
pub struct Game {
    board: Grid,
    turn: Turn,
    rng: ThreadRng,
}

impl std::cmp::PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        self.board == other.board
    }
}

impl std::fmt::Debug for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();

        for row in self.board.iter() {
            for tile in row.iter() {
                let cell_str = match tile {
                    CellState::Empty => "_ ",
                    CellState::PlayerOne => "1 ",
                    CellState::PlayerTwo => "2 ",
                };
                output.push_str(cell_str);
            }
            output.push('\n');
        }
        write!(f, "{output}")
    }
}

impl std::convert::From<(Grid, Turn)> for Game {
    fn from((grid, turn): (Grid, Turn)) -> Self {
        Game {
            board: grid,
            turn,
            rng: thread_rng(),
        }
    }
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: [[CellState::Empty; 7]; 6],
            turn: Turn::PlayerOne,
            rng: thread_rng(),
        }
    }

    pub fn board(&self) -> Grid {
        self.board
    }

    pub fn turn(&self) -> Turn {
        self.turn
    }

    pub fn make_move(&self, column: usize) -> Game {
        let mut new_game = self.clone();

        if new_game.is_col_full(column) {
            return new_game;
        }

        for row in (0..6).rev() {
            if new_game.board[row][column] == CellState::Empty {
                new_game.board[row][column] = match new_game.turn {
                    Turn::PlayerOne => CellState::PlayerOne,
                    Turn::PlayerTwo => CellState::PlayerTwo,
                };
                break;
            }
        }

        new_game.turn = match new_game.turn {
            Turn::PlayerOne => Turn::PlayerTwo,
            Turn::PlayerTwo => Turn::PlayerOne,
        };

        new_game
    }

    pub fn is_col_full(&self, column: usize) -> bool {
        for row in 0..6 {
            if self.board[row][column] == CellState::Empty {
                return false;
            }
        }
        true
    }

    pub fn check_for_result(&self) -> Option<GameResult> {
        // Check for a winner
        if self.check_winner() {
            return match self.turn {
                Turn::PlayerOne => Some(GameResult::PlayerTwoWin),
                Turn::PlayerTwo => Some(GameResult::PlayerOneWin),
            };
        }

        // Check for a draw (full board with no winner)
        if self.is_board_full() {
            return Some(GameResult::Draw);
        }

        // Game is not over
        None
    }

    pub fn is_game_over(&self) -> bool {
        self.check_for_result().is_some()
    }

    fn check_winner(&self) -> bool {
        // Check for a winner in rows, columns, and diagonals
        self.check_winner_in_rows()
            || self.check_winner_in_columns()
            || self.check_winner_in_diagonals()
    }

    fn check_winner_in_rows(&self) -> bool {
        for row in 0..6 {
            for col in 0..4 {
                if self.board[row][col] != CellState::Empty
                    && self.board[row][col] == self.board[row][col + 1]
                    && self.board[row][col] == self.board[row][col + 2]
                    && self.board[row][col] == self.board[row][col + 3]
                {
                    return true; // Winner found in a row
                }
            }
        }
        false
    }

    fn check_winner_in_columns(&self) -> bool {
        for col in 0..7 {
            for row in 0..3 {
                if self.board[row][col] != CellState::Empty
                    && self.board[row][col] == self.board[row + 1][col]
                    && self.board[row][col] == self.board[row + 2][col]
                    && self.board[row][col] == self.board[row + 3][col]
                {
                    return true; // Winner found in a column
                }
            }
        }
        false
    }

    fn check_winner_in_diagonals(&self) -> bool {
        // Check diagonals from top-left to bottom-right
        for row in 0..3 {
            for col in 0..4 {
                if self.board[row][col] != CellState::Empty
                    && self.board[row][col] == self.board[row + 1][col + 1]
                    && self.board[row][col] == self.board[row + 2][col + 2]
                    && self.board[row][col] == self.board[row + 3][col + 3]
                {
                    return true; // Winner found in a diagonal
                }
            }
        }

        // Check diagonals from top-right to bottom-left
        for row in 0..3 {
            for col in (3..7).rev() {
                if self.board[row][col] != CellState::Empty
                    && self.board[row][col] == self.board[row + 1][col - 1]
                    && self.board[row][col] == self.board[row + 2][col - 2]
                    && self.board[row][col] == self.board[row + 3][col - 3]
                {
                    return true; // Winner found in a diagonal
                }
            }
        }

        false
    }

    fn is_board_full(&self) -> bool {
        // Check if the board is completely filled (no Empty cells)
        for row in 0..6 {
            for col in 0..7 {
                if self.board[row][col] == CellState::Empty {
                    return false; // Board is not full
                }
            }
        }
        true
    }
}
