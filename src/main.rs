#![allow(unused)]
mod game;

use game::Game;
use std::io::{self, Write};

fn main() {
    let mut game = Game::new();

    loop {
        clearscreen::clear().unwrap();
        print!("\n\n\n{game:?}\n\n\n");

        let result = game.check_for_winner();

        if let Some(result) = result {
            match result {
                game::GameResult::PlayerOneWin => println!("\nPlayer One Wins!"),
                game::GameResult::PlayerTwoWin => println!("\nPlayer Two Wins!"),
                game::GameResult::Draw => println!("\nDraw!"),
            }
            break;
        }

        match game.turn() {
            game::Turn::PlayerOne => println!("\nPlayer One's Turn"),
            game::Turn::PlayerTwo => println!("\nPlayer Two's Turn"),
        }

        let mut input = String::new();
        io::stdout().flush().unwrap(); // Flush the output to ensure it's printed immediately
        io::stdin().read_line(&mut input).unwrap();

        let raw_input = input.trim();

        match raw_input.parse::<usize>() {
            Ok(column) if column < 8 => {
                game = game.make_move(column - 1);
            }
            _ => {
                println!("Invalid input");
            }
        }

        print!("\r"); // Carriage return to the beginning of the line
    }
}
