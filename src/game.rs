use rand::prelude::*;

pub type Grid = [[u16; 4]; 4];

#[derive(Clone)]
pub struct Game {
    board: Grid,
    score: u32,
    rng: ThreadRng,
}

impl std::cmp::PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        self.board == other.board && self.score == other.score
    }
}

impl std::fmt::Debug for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();

        output.push_str(&format!("Score: {}\n", self.score));

        for row in self.board.iter() {
            for tile in row.iter() {
                output.push_str(&format!("{:4}", tile));
            }
            output.push('\n');
        }
        write!(f, "{}", output)
    }
}

impl std::convert::From<Grid> for Game {
    fn from(value: Grid) -> Self {
        Game {
            board: value,
            score: 0,
            rng: thread_rng(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Game {
    pub fn new() -> Self {
        let game = Game {
            board: [[0; 4]; 4],
            score: 0,
            rng: thread_rng(),
        };
        game.add_random_tile().add_random_tile()
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn board(&self) -> Grid {
        self.board
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
        new_board.board[y][x] = if new_board.rng.gen_range(0..4) == 0 {
            4
        } else {
            2
        };

        new_board
    }

    pub fn move_tiles(&self, direction: Direction) -> Game {
        match direction {
            Direction::Up => self.move_up(),
            Direction::Down => self.move_down(),
            Direction::Left => self.move_left(),
            Direction::Right => self.move_right(),
        }
        .add_random_tile()
    }

    fn move_up(&self) -> Game {
        let mut new_game = self.clone();
        transpose(&mut new_game.board);

        for row in new_game.board.iter_mut() {
            row.reverse();
            let mut new_row = slide_row_foward(*row);
            new_row.reverse();
            *row = new_row;
        }

        transpose(&mut new_game.board);
        new_game
    }

    fn move_down(&self) -> Game {
        let mut new_game = self.clone();
        transpose(&mut new_game.board);

        for row in new_game.board.iter_mut() {
            *row = slide_row_foward(*row);
        }

        transpose(&mut new_game.board);
        new_game
    }

    fn move_left(&self) -> Game {
        let mut new_game = self.clone();

        for row in new_game.board.iter_mut() {
            row.reverse();
            let mut new_row = slide_row_foward(*row);
            new_row.reverse();
            *row = new_row;
        }

        new_game
    }

    fn move_right(&self) -> Game {
        let mut new_game = self.clone();

        for row in new_game.board.iter_mut() {
            *row = slide_row_foward(*row);
        }

        new_game
    }

    pub fn is_game_over(&self) -> bool {
        todo!()
    }

    pub fn reset(&mut self) {
        *self = Game::new();
    }
}

fn transpose(board: &mut Grid) {
    for i in 0..4 {
        for j in i + 1..4 {
            let temp = board[i][j];
            board[i][j] = board[j][i];
            board[j][i] = temp;
        }
    }
}

fn slide_row_foward(mut row: [u16; 4]) -> [u16; 4] {
    // first slide over
    for i in (0..3).rev() {
        let mut cur_idx = i;
        let mut next_idx = i + 1;

        if row[cur_idx] == 0 {
            // current is empty so ignore
            continue;
        }

        while next_idx < 4 && row[next_idx] == 0 {
            // next is empty so slide over
            row[next_idx] = row[cur_idx];
            row[cur_idx] = 0;

            next_idx += 1;
            cur_idx += 1;
        }
    }

    // then collapse same cells
    if row[3] == row[2] {
        row[3] *= 2;
        row[2] = 0;
    }
    if row[2] == row[1] {
        row[2] *= 2;
        row[1] = 0;
    }
    if row[1] == row[0] {
        row[1] *= 2;
        row[0] = 0;
    }

    // then slide over again

    for i in (0..3).rev() {
        let mut cur_idx = i;
        let mut next_idx = i + 1;

        if row[cur_idx] == 0 {
            // current is empty so ignore
            continue;
        }

        while next_idx < 4 && row[next_idx] == 0 {
            // next is empty so slide over
            row[next_idx] = row[cur_idx];
            row[cur_idx] = 0;

            next_idx += 1;
            cur_idx += 1;
        }
    }

    row
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pretty_print() {
        let game = Game::new();
        println!("{:?}", game);

        let _game = Game::from([
            [2, 4, 8, 16],
            [0, 2, 4, 8],
            [0, 0, 2, 4],
            [0, 0, 0, 2],
        ]);

        // println!("{:?}", game);
    }

    #[test]
    fn row_collapse() {
        let before = [0, 2, 0, 0];
        let after = [0, 0, 0, 2];
        assert_eq!(slide_row_foward(before), after);

        let before = [0, 0, 2, 2];
        let after = [0, 0, 0, 4];
        assert_eq!(slide_row_foward(before), after);

        let before = [2, 2, 0, 0];
        let after = [0, 0, 0, 4];
        assert_eq!(slide_row_foward(before), after);

        let before = [2, 0, 2, 0];
        let after = [0, 0, 0, 4];
        assert_eq!(slide_row_foward(before), after);

        let before = [0, 2, 0, 2];
        let after = [0, 0, 0, 4];
        assert_eq!(slide_row_foward(before), after);

        let before = [2, 2, 2, 0];
        let after = [0, 0, 2, 4];
        assert_eq!(slide_row_foward(before), after);

        let before = [2, 2, 2, 2];
        let after = [0, 0, 4, 4];
        assert_eq!(slide_row_foward(before), after);

        let before = [2, 2, 4, 4];
        let after = [0, 0, 4, 8];
        assert_eq!(slide_row_foward(before), after);
    }

    #[test]
    fn move_right() {
        let before = [
            [32, 0, 0, 0],
            [0, 2, 2, 4],
            [2, 4, 8, 2],
            [0, 2, 8, 2],
        ];

        let after = [
            [0, 0, 0, 32],
            [0, 0, 4, 4],
            [2, 4, 8, 2],
            [0, 2, 8, 2],
        ];

        let game = Game::from(before);

        assert_eq!(game.move_right().board, after);
    }

    #[test]
    fn move_left() {
        let before = [
            [32, 0, 0, 0],
            [0, 2, 2, 4],
            [2, 4, 8, 2],
            [0, 2, 8, 2],
        ];

        let after = [
            [32, 0, 0, 0],
            [4, 4, 0, 0],
            [2, 4, 8, 2],
            [2, 8, 2, 0],
        ];
        let game = Game::from(before);

        assert_eq!(game.move_left().board, after);
    }

    #[test]
    fn move_up() {
        let before = [
            [32, 0, 0, 0],
            [0, 2, 2, 4],
            [2, 4, 8, 2],
            [0, 2, 8, 2],
        ];

        let after = [
            [32, 2, 2, 4],
            [2, 4, 16, 4],
            [0, 2, 0, 0],
            [0, 0, 0, 0],
        ];

        let game = Game::from(before);

        assert_eq!(game.move_up().board, after);
    }

    #[test]
    fn move_down() {
        let before = [
            [32, 0, 0, 0],
            [0, 2, 2, 4],
            [2, 4, 8, 2],
            [0, 2, 8, 2],
        ];

        let after = [
            [0, 0, 0, 0],
            [0, 2, 0, 0],
            [32, 4, 2, 4],
            [2, 2, 16, 4],
        ];

        let game = Game::from(before);

        assert_eq!(game.move_down().board, after);
    }

    #[test]
    fn transpose_test() {
        let mut before = [
            [32, 0, 0, 0],
            [0, 2, 2, 4],
            [2, 4, 8, 2],
            [0, 2, 8, 2],
        ];

        transpose(&mut before);

        let after = [
            [32, 0, 2, 0],
            [0, 2, 4, 2],
            [0, 2, 8, 8],
            [0, 4, 2, 2],
        ];

        assert_eq!(before, after);
    }
}
