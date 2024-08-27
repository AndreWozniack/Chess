use chess::{Board, BoardStatus, Color, File, Piece, Square, ALL_SQUARES};

// Auxiliary Functions to print the board
pub fn print_board(board: &Board) {
    println!("  +-----------------+");
    for rank in (0..8).rev() {
        print!("{} | ", rank + 1);
        for file in 0..8 {
            let square =
                Square::make_square(chess::Rank::from_index(rank), chess::File::from_index(file));
            if let Some(piece) = board.piece_on(square) {
                let piece_char = match piece {
                    Piece::Pawn => 'P',
                    Piece::Knight => 'N',
                    Piece::Bishop => 'B',
                    Piece::Rook => 'R',
                    Piece::Queen => 'Q',
                    Piece::King => 'K',
                };

                let piece_display = if board.color_on(square) == Some(Color::White) {
                    piece_char
                } else {
                    piece_char.to_ascii_lowercase()
                };

                print!("{}", piece_display);
            } else {
                print!(".");
            }
            print!(" ");
        }
        println!("|");
    }
    println!("  +-----------------+");
    println!("    a b c d e f g h");
}

pub fn piece_name(piece: Piece) -> &'static str {
    match piece {
        Piece::Pawn => "Pawn",
        Piece::Knight => "Knight",
        Piece::Bishop => "Bishop",
        Piece::Rook => "Rook",
        Piece::Queen => "Queen",
        Piece::King => "King",
    }
}
fn piece_value(piece: Piece) -> i32 {
    match piece {
        Piece::Pawn => 1,
        Piece::Knight => 3,
        Piece::Bishop => 3,
        Piece::Rook => 5,
        Piece::Queen => 9,
        Piece::King => 100,
    }
}

pub(crate) fn square_name(square: Square) -> String {
    format!(
        "{}{}",
        file_to_char(square.get_file()),
        square.get_rank().to_index() + 1
    )
}

fn file_to_char(file: File) -> char {
    (file.to_index() as u8 + b'a') as char
}

pub fn print_move(board: &Board, mov: chess::ChessMove) {
    let from = mov.get_source();
    let to = mov.get_dest();
    let piece = board.piece_on(from).unwrap();
    println!(
        "Moving {} from {} to {}",
        piece_name(piece),
        square_name(from).to_ascii_uppercase(),
        square_name(to).to_ascii_uppercase()
    );
}

pub fn evaluate_board(board: &Board) -> i32 {
    let mut score = 0;
    for &square in ALL_SQUARES.iter() {
        if let Some(piece) = board.piece_on(square) {
            let value = piece_value(piece);
            if board.color_on(square) == Some(Color::White) {
                score += value;
            } else {
                score -= value;
            }
        }
    }
    score
}

pub fn is_terminal(board: &Board) -> bool {
    let status = board.status();
    match status {
        BoardStatus::Checkmate => true,
        BoardStatus::Stalemate => true,
        BoardStatus::Ongoing => false,
    }
}
