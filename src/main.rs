#![allow(unused)]
mod ai;
mod game;

use rayon::{prelude::*, result};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Instant;
use std::{collections::HashMap, env};

use ai::{Weights, AI};
use game::Game;

fn range(start: u32, end: u32, step: u32) -> Vec<u32> {
    let mut result = Vec::new();

    let mut current = start;
    while current <= end {
        result.push(current);
        current += step;
    }

    result
}

fn find_optimal_weights() {
    let weight_ranges: [(u32, u32, u32); 3] = [
        (0, 1, 1), // max_tile
        (0, 1, 1), // adjacent_tiles
        (0, 1, 1), // empty_cells
    ];
    let n_runs = 1; // Number of runs for each weight combination

    // let total_iterations = weight_ranges
    //     .iter()
    //     .map(|&(min, max, step)| ((max - min) / step) as usize + 1)
    //     .product::<usize>();

    let run_start_time = Instant::now();

    for max_tile_weight in range(weight_ranges[0].0, weight_ranges[0].1, weight_ranges[0].2) {
        for adjacent_tiles_weight in
            range(weight_ranges[1].0, weight_ranges[1].1, weight_ranges[1].2)
        {
            for empty_cells_weight in
                range(weight_ranges[2].0, weight_ranges[2].1, weight_ranges[2].2)
            {
                let weights = Weights {
                    max_tile: max_tile_weight,
                    adjacent_tiles: adjacent_tiles_weight,
                    empty_cells: empty_cells_weight,
                };

                // let mut max_tile = 0;
                // let mut max_tile_sum = 0;
                // let mut moves_sum = 0;
                // let mut duration_sum = 0;

                dbg!(weights);

                // let result = run_to_result(weights);
                // dbg!(result);
            }
        }
    }
}

#[derive(Debug)]
struct RunResult {
    max_tile: u16,
    moves: u128,
    duration_ms: u128,
}

fn run_to_result(weights: Weights) -> RunResult {
    let mut ai = AI::new(Game::new(), weights);

    let start_time = Instant::now();
    ai.run_to_completion();
    let duration_ms = start_time.elapsed().as_millis();

    RunResult {
        max_tile: ai.current_game_state.max_tile(),
        moves: ai.current_game_state.moves(),
        duration_ms,
    }
}

fn bench(weights: Weights) {
    let n_runs = 1000;
    let run_start_time = Instant::now();

    let results: Vec<RunResult> = (0..n_runs)
        .into_par_iter()
        .map(|_| run_to_result(weights))
        .collect();

    let best_game = results.iter().max_by_key(|result| result.max_tile).unwrap();
    let worst_game = results.iter().min_by_key(|result| result.max_tile).unwrap();
    let average_max_tile = results
        .iter()
        .map(|result| result.max_tile as u128)
        .sum::<u128>() as f32
        / n_runs as f32;
    let average_moves = results
        .iter()
        .map(|result| result.moves as u128)
        .sum::<u128>() as f32
        / n_runs as f32;
    let average_duration_ms = results
        .iter()
        .map(|result| result.duration_ms)
        .sum::<u128>() as f32
        / n_runs as f32;

    let total_duration_ms = run_start_time.elapsed().as_millis();

    println!("REPORT OVER {} RUNS - {:.2} ms", n_runs, total_duration_ms);
    println!(
        "Best game - max tile: {}, moves: {}, duration: {:.2} ms",
        best_game.max_tile, best_game.moves, best_game.duration_ms
    );
    println!(
        "Worst game - max tile: {}, moves: {}, duration: {:.2} ms",
        worst_game.max_tile, worst_game.moves, worst_game.duration_ms
    );
    println!("Average max tile: {:.2}", average_max_tile);
    println!("Average moves: {:.2}", average_moves);
    println!("Average duration: {:.2} ms", average_duration_ms);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "opt" => {
                find_optimal_weights();
            }
            "bench" => {
                bench(Weights {
                    max_tile: 10,
                    adjacent_tiles: 5,
                    empty_cells: 2,
                });
            }
            _ => {
                println!("Invalid argument");
            }
        }
    }
}
