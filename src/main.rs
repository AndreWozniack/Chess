mod alfa_beta;
mod alpha_beta_whit_randomness;
mod board;

use crate::alfa_beta::alpha_beta;
use crate::board::{ print_board, print_move};
use chess::{Board, ChessMove, Color, Square};
use std::io::{self, Write};
use std::time::{Duration, Instant};

fn main() {
    let mut board = Board::default();
    let mut current_color = Color::White;
    let move_time_limit = Duration::from_secs(120); // 2 minutos para cada jogada

    loop {
        print_board(&board);

        if board.status() != chess::BoardStatus::Ongoing {
            break;
        }

        let start_time = Instant::now();
        let next_move = if current_color == Color::White {

            // Vez das Brancas (Humano)
            println!("Vez das Brancas (Humano):");
            get_player_move(&board)
        } else {
            // Vez das Pretas (IA)
            println!("Vez das Pretas (IA):");
            Some(alpha_beta(&board, 5))
        };

        match next_move {
            Some(mov) => {
                let elapsed_time = start_time.elapsed();
                if elapsed_time > move_time_limit {
                    println!("{:?} demorou muito para jogar e perdeu!", current_color);
                    break;
                }
                print_move(&board, mov);
                board = board.make_move_new(mov); // Faz a jogada.
                current_color = !current_color; // Troca a cor do jogador.
                println!("---------------------------------")
            },
            None => {
                println!("Nenhum movimento válido fornecido, tente novamente.");
            }
        }
    }

    println!("Final do jogo: {:?}", board.status());
    if board.status() == chess::BoardStatus::Checkmate {
        println!("{:?} venceu por xeque-mate!", !current_color);
    } else if board.status() == chess::BoardStatus::Stalemate {
        println!("O jogo terminou em empate por stalemate.");
    }
}

fn get_player_move(board: &Board) -> Option<ChessMove> {
    loop {
        print!("Digite seu movimento (por exemplo, e2e4): ");
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
                    println!("Movimento ilegal! Tente novamente.");
                }
            }
        } else {
            println!("Entrada inválida! Use o formato de movimento como e2e4.");
        }
    }
}
