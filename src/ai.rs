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
        let mut new_board = board.clone();
        apply_move(&mut new_board, &possible_move, piece);
        let score = minimax(&new_board, 0, 9, piece, piece, alpha, beta);

        if score > best_score {
            best_score = score;
            best_move = Some(possible_move);
        }

        // alpha = std::cmp::max(alpha, score);
        // if beta <= alpha {
        //     break;
        // }
    }

    best_move.expect("No valid moves available")
}

fn minimax(
    board: &Board,
    depth: i32,
    max_depth: i32,
    perspective: Piece,
    piece: Piece,
    mut alpha: i32,
    mut beta: i32,
) -> i32 {
    if depth == max_depth || is_game_over(board) {
        return evaluate(board, perspective, depth, max_depth);
    }

    let piece = match piece {
        Piece::X => Piece::O,
        Piece::O => Piece::X,
    };
    if piece == perspective {
        let mut max_eval = i32::MIN;
        for possible_move in get_available_moves(board) {
            let mut new_board = board.clone();
            apply_move(&mut new_board, &possible_move, piece);
            let eval = minimax(
                &new_board,
                depth + 1,
                max_depth,
                perspective,
                piece,
                alpha,
                beta,
            );
            max_eval = std::cmp::max(max_eval, eval);
            // alpha = std::cmp::max(alpha, eval);
            // if beta <= alpha {
            //     break;
            // }
        }
        max_eval
    } else {
        let mut min_eval = i32::MAX;
        for possible_move in get_available_moves(board) {
            let mut new_board = board.clone();
            apply_move(&mut new_board, &possible_move, piece);
            let eval = minimax(
                &new_board,
                depth + 1,
                max_depth,
                perspective,
                piece,
                alpha,
                beta,
            );
            min_eval = std::cmp::min(min_eval, eval);
            // beta = std::cmp::min(beta, eval);
            // if beta <= alpha {
            //     break;
            // }
        }
        min_eval
    }
}

fn evaluate(board: &Board, perspective: Piece, depth: i32, max_depth: i32) -> i32 {
    let winner = is_game_won(board);
    if let Some(winner) = winner {
        if winner == perspective {
            10_000
        } else {
            -10_000
        }
    } else {
        0
    }
}

// winner = getWinner(board)
// if not winner:
//     return -100 * math.pow(2, 9 - depth)    #   tie is always depth 9..
// elif winner == togglePlayer(myPlayer):
//     return -1000 * math.pow(2, 9 - depth)
// elif winner == myPlayer:
//     return 100000000 / depth #/ math.pow(2, depth)

// fn evaluate(board: &Board, piece: Piece, depth: i32) -> i32 {
//     let winner = is_game_won(board);
//     if let Some(winner) = winner {
//         if winner == piece {
//             10_000
//         } else {
//             -10_000
//         }
//     } else {
//         0
//     }
// }
