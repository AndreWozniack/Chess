mod alfa_beta;
mod alpha_beta_whit_randomness;
mod board;

use crate::alfa_beta::alpha_beta;
use crate::board::{print_board, print_move};
use chess::{Board, ChessMove, Color, Square};
use std::io::{self, Write};
use std::time::{Duration, Instant};

fn main() {
    let mut board = Board::default();
    let mut current_color = Color::White;
    let move_time_limit = Duration::from_secs(120);

    loop {
        print_board(&board);

        if board.status() != chess::BoardStatus::Ongoing {
            break;
        }

        let start_time = Instant::now();
        let next_move = if current_color == Color::White {
            println!("White p's turn:");
            get_player_move(&board)
        } else {
            println!("Black p's turn:");
            Some(alpha_beta(&board, 5))
        };

        match next_move {
            Some(mov) => {
                let elapsed_time = start_time.elapsed();
                if elapsed_time > move_time_limit {
                    println!("{:?} took too long to play and lost!", current_color);
                    break;
                }
                print_move(&board, mov);
                board = board.make_move_new(mov);
                current_color = !current_color;
                println!("---------------------------------")
            }
            None => {
                println!("No valid movement provided, try again");
            }
        }
    }

    println!("End of game: {:?}", board.status());
    if board.status() == chess::BoardStatus::Checkmate {
        println!("{:?} won by checkmate!", !current_color);
    } else if board.status() == chess::BoardStatus::Stalemate {
        println!("The match ended in a stalemate.");
    }
}

fn get_player_move(board: &Board) -> Option<ChessMove> {
    loop {
        print!("Enter your move: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let trimmed_input = input.trim();

        if trimmed_input.len() == 4 {
            let from = trimmed_input[0..2].parse::<Square>().ok();
            let to = trimmed_input[2..4].parse::<Square>().ok();
            if let (Some(from), Some(to)) = (from, to) {
                let chess_move = ChessMove::new(from, to, None);
                if board.legal(chess_move) {
                    return Some(chess_move);
                } else {
                    println!("Invalid move! Try again.");
                }
            }
        } else {
            println!("Invalid move! Try again.");
        }
    }
}
