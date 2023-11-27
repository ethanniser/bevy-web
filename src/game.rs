use rand::prelude::*;

#[derive(Clone)]
struct Game {
    board: [[u16; 4]; 4],
    score: u32,
    rng: ThreadRng,
}

impl std::cmp::PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        self.board == other.board && self.score == other.score
    }
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Game {
    fn new() -> Self {
        let game = Game {
            board: [[0; 4]; 4],
            score: 0,
            rng: thread_rng(),
        };
        game.add_random_tile().add_random_tile()
    }

    fn add_random_tile(&self) -> Game {
        let mut new_board = self.clone();
        let mut empty_tiles = Vec::new();
        for (y, row) in new_board.board.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if *tile == 0 {
                    empty_tiles.push((x, y));
                }
            }
        }
        if empty_tiles.is_empty() {
            return new_board;
        }
        let (x, y) = empty_tiles[new_board.rng.gen_range(0..empty_tiles.len())];
        new_board.board[y][x] = if new_board.rng.gen_range(0..4) == 0 { 4 } else { 2 };

        new_board
    }

    // Returns the new game state if the move was valid, or None if the move was invalid.
    fn move_tiles(&self, direction: Direction) -> Option<Game> {
        todo!();
    }

    fn is_game_over(&self) -> bool {
        [Direction::Up, Direction::Down, Direction::Left, Direction::Right]
            .iter()
            .all(|dir| self.move_tiles(*dir) == None)
    }

    fn reset(&mut self) {
        *self = Game::new();
    }
}
