use rand::seq::SliceRandom;

use indicatif::ParallelProgressIterator;
use rayon::prelude::*;

use crate::game::{apply_move, display_board, is_game_won, no_more_moves, Piece};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Strategy {
    Random,
    Minimax,
}

// #[test]
fn never_lose_vs_random() {
    let mut wins_for_random = 0;
    let mut wins_for_minimax = 0;
    let mut draws = 0;

    const NUM_GAMES: u32 = 100;

    for _ in 0..NUM_GAMES {
        let mut board = vec![vec![None; 3]; 3];
        let mut current_piece = Piece::X;

        // let mut turn: Strategy;
        let mut turn: Strategy = Strategy::Minimax;
        let mut rng = rand::thread_rng();
        // match [Strategy::Random, Strategy::Minimax].choose(&mut rng) {
        //     Some(choice) => turn = *choice,
        //     None => panic!("Failed to choose who goes first"),
        // }

        loop {
            let pos = match turn {
                Strategy::Random => crate::ai::get_random_valid_move(&mut rng, &board),
                Strategy::Minimax => crate::ai::pick_best_move(&mut rng, &board, current_piece),
            };

            apply_move(&mut board, &pos, current_piece);
            if let Some(winner) = is_game_won(&board) {
                match turn {
                    Strategy::Random => wins_for_random += 1,
                    Strategy::Minimax => wins_for_minimax += 1,
                }
                break;
            }

            if no_more_moves(&board) {
                draws += 1;
                break;
            }

            current_piece = if current_piece == Piece::X {
                Piece::O
            } else {
                Piece::X
            };
            turn = if turn == Strategy::Random {
                Strategy::Minimax
            } else {
                Strategy::Random
            };
        }

        // clear screen
        print!("\x1B[2J\x1B[1;1H");
        println!(
            "Random: {}, Minimax: {}, Draws: {}",
            wins_for_random, wins_for_minimax, draws
        );

        // print win rate for minimax
        let total_games = wins_for_random + wins_for_minimax + draws;
        println!(
            "Minimax win rate: {}%",
            (wins_for_minimax as f32 / total_games as f32) * 100.0
        );

        // assert random never won
        assert_eq!(wins_for_random, 0);
    }
}

#[test]
fn many_games() {
    let mut wins_for_random = 0;
    let mut wins_for_minimax = 0;
    let mut draws = 0;

    const NUM_GAMES: u32 = 10000;

    let results: Vec<Option<Strategy>> = (0..NUM_GAMES)
        .into_par_iter()
        .progress_count(NUM_GAMES as u64)
        .map(|_| do_mm_vs_random_game())
        .collect();

    for result in results {
        match result {
            Some(Strategy::Random) => wins_for_random += 1,
            Some(Strategy::Minimax) => wins_for_minimax += 1,
            None => draws += 1,
        }
    }

    print!("\x1B[2J\x1B[1;1H");
    println!(
        "Random: {}, Minimax: {}, Draws: {}",
        wins_for_random, wins_for_minimax, draws
    );

    // print win rate for minimax
    let total_games = wins_for_random + wins_for_minimax + draws;
    println!(
        "Minimax win rate: {}%",
        (wins_for_minimax as f32 / total_games as f32) * 100.0
    );

    // assert random never won
    assert_eq!(wins_for_random, 0);
}

pub fn do_mm_vs_random_game() -> Option<Strategy> {
    let mut board = vec![vec![None; 3]; 3];
    let mut current_piece = Piece::X;

    let mut turn: Strategy;
    // let mut turn: Strategy = Strategy::Minimax;
    let mut rng = rand::thread_rng();
    match [Strategy::Random, Strategy::Minimax].choose(&mut rng) {
        Some(choice) => turn = *choice,
        None => panic!("Failed to choose who goes first"),
    }

    loop {
        let pos = match turn {
            Strategy::Random => crate::ai::get_random_valid_move(&mut rng, &board),
            Strategy::Minimax => crate::ai::pick_best_move(&mut rng, &board, current_piece),
        };

        apply_move(&mut board, &pos, current_piece);
        if let Some(winner) = is_game_won(&board) {
            match turn {
                Strategy::Random => return Some(Strategy::Random),
                Strategy::Minimax => return Some(Strategy::Minimax),
            }
        }

        if no_more_moves(&board) {
            return None;
        }

        current_piece = if current_piece == Piece::X {
            Piece::O
        } else {
            Piece::X
        };
        turn = if turn == Strategy::Random {
            Strategy::Minimax
        } else {
            Strategy::Random
        };
    }
}
