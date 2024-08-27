use crate::alfa_beta::min;
use chess::{Board, ChessMove, MoveGen};
use rand::prelude::IndexedRandom;
use rand::thread_rng;

#[allow(dead_code)]
pub fn alpha_beta_with_randomness(board: &Board, depth: usize) -> ChessMove {
    let alpha = i32::MIN;
    let beta = i32::MAX;
    let mut best_moves = Vec::new();
    let mut best_value = i32::MIN;

    for mov in MoveGen::new_legal(board) {
        let new_board = board.make_move_new(mov);
        let value = min(&new_board, alpha, beta, depth - 1);
        if value > best_value {
            best_moves.clear();
            best_moves.push(mov);
            best_value = value;
        } else if value == best_value {
            best_moves.push(mov);
        }
    }

    let mut rng = thread_rng();
    best_moves
        .choose(&mut rng)
        .expect("No legal moves available")
        .clone()
}
