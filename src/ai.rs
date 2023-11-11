use rand::{rngs::ThreadRng, seq::SliceRandom};

use crate::game::{
    apply_move, get_available_moves, is_game_over, is_game_won, Board, Piece, Position,
};

pub fn get_random_move(rng: &mut ThreadRng) -> Position {
    *[
        Position { x: 0, y: 0 },
        Position { x: 1, y: 0 },
        Position { x: 2, y: 0 },
        Position { x: 0, y: 1 },
        Position { x: 1, y: 1 },
        Position { x: 2, y: 1 },
        Position { x: 0, y: 2 },
        Position { x: 1, y: 2 },
        Position { x: 2, y: 2 },
    ]
    .choose(rng)
    .unwrap()
}

pub fn get_random_valid_move(rng: &mut ThreadRng, board: &Board) -> Position {
    *get_available_moves(board).choose(rng).unwrap()
}

pub fn pick_best_move(rng: &mut ThreadRng, board: &Board, piece: Piece) -> Position {
    let mut best_score = i32::MIN;
    let mut best_move = None;
    let mut alpha = i32::MIN;
    let beta = i32::MAX;

    for possible_move in get_available_moves(board) {
        println!("Trying move: {}", possible_move);
        let mut new_board = board.clone(); // Assuming Board is Cloneable
        apply_move(&mut new_board, &possible_move, piece);
        let score = minimax(&new_board, 9, true, piece, alpha, beta);

        if score > best_score {
            best_score = score;
            best_move = Some(possible_move);
        }

        alpha = std::cmp::max(alpha, score);
        if beta <= alpha {
            break;
        }
    }

    best_move.expect("No valid moves available")
}

fn minimax(
    board: &Board,
    depth: i32,
    is_maximizing_player: bool,
    piece: Piece,
    mut alpha: i32,
    mut beta: i32,
) -> i32 {
    if depth == 0 || is_game_over(board) {
        return evaluate(board, piece);
    }

    let piece = match piece {
        Piece::X => Piece::O,
        Piece::O => Piece::X,
    };
    if is_maximizing_player {
        let mut max_eval = i32::MIN;
        for possible_move in get_available_moves(board) {
            let mut new_board = board.clone();
            apply_move(&mut new_board, &possible_move, piece);
            let eval = minimax(&new_board, depth - 1, false, piece, alpha, beta);
            max_eval = std::cmp::max(max_eval, eval);
            alpha = std::cmp::max(alpha, eval);
            if beta <= alpha {
                break;
            }
        }
        max_eval
    } else {
        let mut min_eval = i32::MAX;
        for possible_move in get_available_moves(board) {
            let mut new_board = board.clone();
            apply_move(&mut new_board, &possible_move, piece);
            let eval = minimax(&new_board, depth - 1, true, piece, alpha, beta);
            min_eval = std::cmp::min(min_eval, eval);
            beta = std::cmp::min(beta, eval);
            if beta <= alpha {
                break;
            }
        }
        min_eval
    }
}

fn evaluate(board: &Board, piece: Piece) -> i32 {
    let winner = is_game_won(board);
    if let Some(winner) = winner {
        if winner == piece {
            10
        } else {
            -10
        }
    } else {
        0
    }
}
