#[allow(unused)]
mod ai;
#[allow(unused)]
mod game;

use std::time::Instant;

use ai::AI;
use game::Game;

struct RunResult {
    max_tile: u16,
    moves: u16,
    duration_ms: u128,
}

fn main() {
    let n_runs = 100;

    let results: Vec<RunResult> = (0..n_runs)
        .map(|_| {
            let mut ai = AI::new(Game::new());

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
    let average_max_tile =
        results.iter().map(|result| result.max_tile).sum::<u16>() as f32 / n_runs as f32;
    let average_moves =
        results.iter().map(|result| result.moves).sum::<u16>() as f32 / n_runs as f32;
    let average_duration_ms = results
        .iter()
        .map(|result| result.duration_ms)
        .sum::<u128>() as f32
        / n_runs as f32;

    println!("REPORT OVER {} RUNS", n_runs);
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
