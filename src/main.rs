use clap::{arg, command, ArgAction};
use game::Piece;
use rand::seq::SliceRandom;

use crate::{
    ai::{get_random_move, get_random_valid_move, pick_best_move},
    game::{
        apply_move, display_board, is_game_won, is_valid_move, move_code_to_position, no_more_moves,
    },
};

mod ai;
mod game;

enum GameMode {
    PvP,
    PvC,
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
        .get_matches();

    let game_mode = if *matches.get_one::<bool>("pvp").unwrap() {
        GameMode::PvP
    } else {
        GameMode::PvC
    };

    match game_mode {
        GameMode::PvP => play_pvp(),
        GameMode::PvC => play_pvc(),
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
            let computers_move = pick_best_move(&mut rng, &board, current_piece);
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
