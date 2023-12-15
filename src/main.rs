#![allow(unused)]
mod ai;
mod game;

use game::{Game, GameResult, Turn};
use std::io::{self, Write};

use ai::AI;

fn main() {
    let mut game = Game::new();

    loop {
        // clearscreen::clear().unwrap();

        if check_end_of_game(&game) {
            break;
        }

        match game.turn() {
            Turn::PlayerOne => println!("YOUR Turn"),
            Turn::PlayerTwo => println!("AI's Turn"),
        }

        game = match game.turn() {
            Turn::PlayerOne => prompt_and_make_human_move(&game),
            Turn::PlayerTwo => prompt_and_make_ai_move(&game),
        };

        print!("\r"); // Carriage return to the beginning of the line
    }
}

fn check_end_of_game(game: &Game) -> bool {
    println!("\n{game:?}\n");
    let result = game.check_for_result();

    if let Some(result) = result {
        match result {
            GameResult::PlayerOneWin => println!("\nYOU Win!"),
            GameResult::PlayerTwoWin => println!("\nAI Wins!"),
            GameResult::Draw => println!("\nDraw!"),
        }
        true
    } else {
        false
    }
}

fn prompt_and_make_human_move(game: &Game) -> Game {
    println!("Enter a column number (1-7): ");

    loop {
        let mut input = String::new();
        io::stdout().flush().unwrap(); // Flush the output to ensure it's printed immediately
        io::stdin().read_line(&mut input).unwrap();

        let raw_input = input.trim();

        match raw_input.parse::<usize>() {
            Ok(column) if column < 8 => {
                break game.make_move(column - 1);
            }
            _ => {
                println!("Invalid input");
            }
        }
    }
}

fn prompt_and_make_ai_move(game: &Game) -> Game {
    let ai = AI::new(game.clone());
    let best_move = ai.get_best_move();
    println!("AI's move: {}", best_move + 1);
    game.make_move(best_move)
}
