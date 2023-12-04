#![allow(unused)]
mod ai;
mod game;

use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Instant;
use std::{collections::HashMap, env};

use ai::{Weights, AI};
use game::Game;

fn find_optimal_weights() {
    let weight_ranges: [(u32, u32); 4] = [
        (0, 5), // max_tile
        (0, 0), // adjacent_tiles
        (0, 0), // empty_cells
        (0, 0), // monotonicity
    ];
    let step = 1;
    let n_runs = 10; // Number of runs for each weight combination

    let results = Mutex::new(HashMap::new());

    let total_iterations = weight_ranges
        .iter()
        .map(|&(min, max)| ((max - min) / step) as usize + 1)
        .product::<usize>();
    let current_iteration = AtomicUsize::new(0);

    let run_start_time = Instant::now();

    (weight_ranges[0].0 as u32..=weight_ranges[0].1 as u32)
        .step_by(step as usize)
        .collect::<Vec<_>>()
        .par_iter()
        .for_each(|&max_tile| {
            for adjacent_tiles in
                (weight_ranges[1].0 as u32..=weight_ranges[1].1 as u32).step_by(step as usize)
            {
                for empty_cells in
                    (weight_ranges[2].0 as u32..=weight_ranges[2].1 as u32).step_by(step as usize)
                {
                    for monotonicity in (weight_ranges[3].0 as u32..=weight_ranges[3].1 as u32)
                        .step_by(step as usize)
                    {
                        let weights = Weights {
                            max_tile,
                            adjacent_tiles ,
                            empty_cells ,
                            monotonicity ,
                        };

                        let mut total_max_tile = 0;
                        let mut total_moves = 0;

                        for _ in 0..n_runs {
                            let mut ai = AI::new(Game::new(), weights);
                            ai.run_to_completion();
                            total_max_tile += ai.current_game_state.max_tile() as u128;
                            total_moves += ai.current_game_state.moves() as u128;
                        }

                        let average_max_tile = total_max_tile as f32 / n_runs as f32;
                        let average_moves = total_moves as f32 / n_runs as f32;

                        results
                            .lock()
                            .unwrap()
                            .insert(weights, (average_max_tile, average_moves));

                        current_iteration.fetch_add(1, Ordering::SeqCst);
                        let progress = current_iteration.load(Ordering::SeqCst) as f32 / total_iterations as f32 * 100.0;
                        println!(
                            "Progress: {} / {} ({}%), Tested weights: {:?}, Average Max Tile: {:.2}, Average Moves: {:.2}",
                            current_iteration.load(Ordering::SeqCst), total_iterations, progress, weights, average_max_tile, average_moves
                        );
                    }
                }
            }
        });

    let results = results.into_inner().unwrap();
    // Find the best performing weights based on your criteria (e.g., highest average max tile)
    let (best_weights, best_performance) = results
        .iter()
        .max_by_key(|&(_, (avg_max_tile, _))| *avg_max_tile as u128)
        .unwrap();

    let total_duration_ms = run_start_time.elapsed().as_millis();

    let report = format!(
        "\nFinal Report:\n\
        - Total Iterations: {}\n\
        - Optimal Weights: {:?} with performance: {:?}\n\
        - Total Duration: {:.2} ms",
        total_iterations, best_weights, best_performance, total_duration_ms
    );

    println!("{}", report);
}

struct RunResult {
    max_tile: u16,
    moves: u128,
    duration_ms: u128,
}

fn bench(weights: Weights) {
    let n_runs = 1000;
    let run_start_time = Instant::now();

    let results: Vec<RunResult> = (0..n_runs)
        .into_par_iter()
        .map(|_| {
            let mut ai = AI::new(Game::new(), weights);

            let start_time = Instant::now();
            ai.run_to_completion();
            let duration_ms = start_time.elapsed().as_millis();

            RunResult {
                max_tile: ai.current_game_state.max_tile(),
                moves: ai.current_game_state.moves(),
                duration_ms,
            }
        })
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
                    monotonicity: 0,
                });
            }
            _ => {
                println!("Invalid argument");
            }
        }
    }
}
