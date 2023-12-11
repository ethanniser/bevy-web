#![allow(unused)]
mod game;

use game::Game;

use crate::game::Direction;

fn main() {
    let mut game = Game::new();

    while (!game.is_game_over()) {
        println!("{game:?}");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let direction = match input.trim() {
            "w" => Direction::Up,
            "a" => Direction::Left,
            "s" => Direction::Down,
            "d" => Direction::Right,
            _ => continue,
        };

        game = game.make_move(direction);
    }
}
