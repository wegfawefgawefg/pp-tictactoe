use core::time;
use std::os::unix::raw::time_t;

use clap::{arg, command, ArgAction};
use game::Piece;
use indicatif::ProgressIterator;
use rand::{seq::SliceRandom, Rng};

use crate::{
    ai::{pick_best_move, pick_best_move_par},
    game::{
        apply_move, display_board, get_available_moves, is_game_won, is_valid_move,
        move_code_to_position, no_more_moves,
    },
};

mod ai;
mod game;

#[cfg(test)]
mod test;

enum GameMode {
    PlayerVsPlayer,
    PlayerVsComputer,
}

fn main() {
    let matches = command!() // Automatically sets up the command with metadata from Cargo.toml
        .about("Plays a game of Tic Tac Toe")
        .arg(
            arg!(-p --pvp "Enables Player vs Player mode").action(ArgAction::SetTrue), // Explicitly set the action
        )
        .arg(
            arg!(-c --pvc "Enables Player vs Computer mode")
                .default_value("true")
                .action(ArgAction::SetTrue), // Explicitly set the action
        )
        .arg(
            arg!(-k --performance "Reports the time taken to make a move.")
                .action(ArgAction::SetTrue), // Explicitly set the action
        )
        .get_matches();

    // perform a performance check
    if *matches.get_one::<bool>("performance").unwrap() {
        do_performance_check();
        return;
    }

    let game_mode = if *matches.get_one::<bool>("pvp").unwrap() {
        GameMode::PlayerVsPlayer
    } else {
        GameMode::PlayerVsComputer
    };

    match game_mode {
        GameMode::PlayerVsPlayer => play_pvp(),
        GameMode::PlayerVsComputer => play_pvc(),
    }
}

pub fn play_pvp() {
    let mut board = vec![vec![None; 3]; 3];
    let mut current_piece = Piece::X;

    loop {
        display_board(&board);
        println!("Player {}, enter your move [1..9]:", current_piece);

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        let pos = match move_code_to_position(input) {
            Some(p) => p,
            None => {
                println!("Invalid input, try again.");
                continue;
            }
        };

        if !is_valid_move(&board, &pos) {
            println!("Invalid move, try again.");
            continue;
        }

        apply_move(&mut board, &pos, current_piece);
        if let Some(winner) = is_game_won(&board) {
            display_board(&board);
            println!("Player {} wins!", winner);
            break;
        }
        if no_more_moves(&board) {
            display_board(&board);
            println!("Game over! It's a draw!");
            break;
        }
        current_piece = if current_piece == Piece::X {
            Piece::O
        } else {
            Piece::X
        };
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Turn {
    Player,
    Computer,
}

pub fn play_pvc() {
    let mut board = vec![vec![None; 3]; 3];

    let mut current_piece = Piece::X;

    let mut turn: Turn;
    let mut rng = rand::thread_rng();
    match [Turn::Player, Turn::Computer].choose(&mut rng) {
        Some(choice) => turn = *choice,
        None => panic!("Failed to choose who goes first"),
    }

    match turn {
        Turn::Player => println!("Player goes first!"),
        Turn::Computer => {
            println!("Computer goes first!");
        }
    }

    loop {
        let pos = if turn == Turn::Player {
            display_board(&board);
            println!("Player {}, enter your move [1..9]:", current_piece);

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            match move_code_to_position(input) {
                Some(p) => p,
                None => {
                    println!("Invalid input, try again.");
                    continue;
                }
            }
        } else {
            // let computers_move = get_random_valid_move(&mut rng, &board);
            let computers_move = pick_best_move_par(&mut rng, &board, current_piece);
            println!("Computer chose position {}", computers_move);
            computers_move
        };

        if !is_valid_move(&board, &pos) {
            println!("Invalid move, try again.");
            continue;
        }

        apply_move(&mut board, &pos, current_piece);
        if let Some(winner) = is_game_won(&board) {
            display_board(&board);
            match turn {
                Turn::Player => println!("Player {} wins!", winner),
                Turn::Computer => println!("Computer wins!"),
            }
            break;
        }
        if no_more_moves(&board) {
            display_board(&board);
            println!("Game over! It's a draw!");
            break;
        }
        current_piece = if current_piece == Piece::X {
            Piece::O
        } else {
            Piece::X
        };
        turn = if turn == Turn::Player {
            Turn::Computer
        } else {
            Turn::Player
        };
    }
}

pub fn do_performance_check() {
    const NUM_GAMES: u32 = 1000;
    let total_time_taken_to_move: u128 = (0..NUM_GAMES)
        .progress_count(NUM_GAMES as u64)
        .map(|_| {
            let mut rng = rand::thread_rng();
            let mut board = vec![vec![None; 3]; 3];

            // random board with random pieces
            // (0..3).for_each(|x| {
            //     (0..3).for_each(|y| {
            //         board[y][x] = *[None, Some(Piece::X), Some(Piece::O)]
            //             .choose(&mut rng)
            //             .unwrap();
            //     });
            // });

            // make sure there is a move to make
            // let available_moves = get_available_moves(&board);
            // // if available moves is empty, then the board is full, so we need to remove a piece
            // if available_moves.is_empty() {
            //     let random_x = rng.gen_range(0..3);
            //     let random_y = rng.gen_range(0..3);
            //     board[random_y][random_x] = None;
            // }

            let piece = Piece::X;

            let now = std::time::Instant::now();
            pick_best_move(&mut rng, &board, piece);
            now.elapsed().as_nanos()
        })
        .sum();

    // compute stats
    let average_time_taken_to_move = total_time_taken_to_move as f64 / NUM_GAMES as f64;
    println!(
        "Average time taken to make a move: {:.2}ns",
        average_time_taken_to_move
    );

    // in millis
    let average_time_taken_to_move = average_time_taken_to_move / 1_000_000.0;
    println!(
        "Average time taken to make a move: {:.2}ms",
        average_time_taken_to_move
    );

    let total_time_taken_to_move: u128 = (0..NUM_GAMES)
        .progress_count(NUM_GAMES as u64)
        .map(|_| {
            let mut rng = rand::thread_rng();
            let mut board = vec![vec![None; 3]; 3];

            // random board with random pieces
            // (0..3).for_each(|x| {
            //     (0..3).for_each(|y| {
            //         board[y][x] = *[None, Some(Piece::X), Some(Piece::O)]
            //             .choose(&mut rng)
            //             .unwrap();
            //     });
            // });

            // make sure there is a move to make
            // let available_moves = get_available_moves(&board);
            // // if available moves is empty, then the board is full, so we need to remove a piece
            // if available_moves.is_empty() {
            //     let random_x = rng.gen_range(0..3);
            //     let random_y = rng.gen_range(0..3);
            //     board[random_y][random_x] = None;
            // }

            let piece = Piece::X;

            let now = std::time::Instant::now();
            pick_best_move_par(&mut rng, &board, piece);
            now.elapsed().as_nanos()
        })
        .sum();

    // compute stats
    let average_time_taken_to_move = total_time_taken_to_move as f64 / NUM_GAMES as f64;
    println!(
        "Average time taken to make a move with threading: {:.2}ns",
        average_time_taken_to_move
    );

    // in millis
    let average_time_taken_to_move = average_time_taken_to_move / 1_000_000.0;
    println!(
        "Average time taken to make a move with threading: {:.2}ms",
        average_time_taken_to_move
    );
}
