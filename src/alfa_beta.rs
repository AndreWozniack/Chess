use crate::board::{evaluate_board, is_terminal};
use chess::{Board, ChessMove, MoveGen};

pub fn alpha_beta(board: &Board, depth: usize) -> ChessMove {
    let mut alpha = i32::MIN;
    let mut beta = i32::MAX;
    let mut best_move = None;
    let mut best_value = i32::MIN;

    for mov in MoveGen::new_legal(board) {
        let mut new_board = board.make_move_new(mov);
        let value = min(&new_board, alpha, beta, depth - 1);
        if value > best_value {
            best_value = value;
            best_move = Some(mov);
        }
    }

    best_move.expect("No legal moves available").clone()
}

pub fn max(board: &Board, alpha: i32, beta: i32, depth: usize) -> i32 {
    if depth == 0 || is_terminal(board) {
        return evaluate_board(board);
    }
    let mut value = i32::MIN;
    let mut alpha = alpha;

    for mov in MoveGen::new_legal(board) {
        let new_board = board.make_move_new(mov);
        value = value.max(min(&new_board, alpha, beta, depth - 1));
        if value >= beta {
            break;
        }
        alpha = alpha.max(value);
    }
    value
}

pub fn min(board: &Board, alpha: i32, beta: i32, depth: usize) -> i32 {
    if depth == 0 || is_terminal(board) {
        return evaluate_board(board);
    }
    let mut value = i32::MAX;
    let mut beta = beta;

    for mov in MoveGen::new_legal(board) {
        let new_board = board.make_move_new(mov);
        value = value.min(max(&new_board, alpha, beta, depth - 1));
        if value <= alpha {
            break;
        }
        beta = beta.min(value);
    }
    value
}
