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
    }

    fn move_up(&self) -> Game {
        let mut new_board = self.clone();
        for x in 0..4 {
            let mut y = 0;
            while y < 4 {
                if new_board.board[y][x] == 0 {
                    let mut y2 = y + 1;
                    while y2 < 4 {
                        if new_board.board[y2][x] != 0 {
                            new_board.board[y][x] = new_board.board[y2][x];
                            new_board.board[y2][x] = 0;
                            break;
                        }
                        y2 += 1;
                    }
                }
                y += 1;
            }
        }
        new_board
    }

    fn move_down(&self) -> Game {
        todo!()
    }

    fn move_left(&self) -> Game {
        todo!()
    }

    fn move_right(&self) -> Game {
        todo!()
    }

    pub fn is_game_over(&self) -> bool {
        todo!()
    }

    pub fn reset(&mut self) {
        *self = Game::new();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pretty_print() {
        let game = Game::new();
        println!("{:?}", game);

        let game = Game::from([
            [2, 4, 8, 16],
            [0, 2, 4, 8],
            [0, 0, 2, 4],
            [0, 0, 0, 2],
        ]);

        println!("{:?}", game);
    }

    #[test]
    fn row_collapse() {
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
        let after = [0, 0, 4, 2];
        assert_eq!(slide_row_foward(before), after);

        let before = [2, 2, 2, 2];
        let after = [0, 0, 4, 4];
        assert_eq!(slide_row_foward(before), after);

        let before = [2, 2, 4, 4];
        let after = [0, 0, 4, 8];
        assert_eq!(slide_row_foward(before), after);
    }
}

fn slide_row_foward(mut row: [u16; 4]) -> [u16; 4] {
    // first slide over
    for i in 0..3 {
        let cur = row[i];
        let next = row[i + 1];

        if next == 0 {
            // next is empty so slide over
            row[i] = 0;
            row[i + 1] = cur;
        }
    }

    // then collapse same cells
    for i in 0..3 {
        let cur = row[i];
        let next = row[i + 1];

        if cur != 0 {
            if next == cur {
                // they are the same so collapse
                row[i] = 0;
                row[i + 1] = cur + next;
            }
        }
    }

    // then slide over again

    for i in 0..3 {
        let cur = row[i];
        let next = row[i + 1];

        if next == 0 {
            // next is empty so slide over
            row[i] = 0;
            row[i + 1] = cur;
        }
    }

    row
}
