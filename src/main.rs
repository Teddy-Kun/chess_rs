use chess_rs::{
	board::Board,
	piece::{Color, Piece, PieceType},
};

fn main() {
	let mut board = Board::empty();
	// let m = notation::parse_notation("Pc2c3", Color::White);
	// println!("{m:?}");

	let mut debug_queen = Piece::new(PieceType::Queen, Color::Black);
	debug_queen.force_position(Board::notation_to_index('b', 2).unwrap());
	board.add_piece(debug_queen);

	for l_move in debug_queen.get_legal_moves() {
		board.insert_debug_spot(l_move);
	}

	println!("{board}");
}

// fn debug1() {
// use chess_rs::{
// 	board::Board,
// 	notation::{Move, parse_notation, start_notation},
// 	piece::{Color, Piece, PieceType},
// };
// 	let mut board: Board = start_notation("kf1", Color::Black).unwrap();
// 	// let mut board = Board::empty();

// 	let parsed = parse_notation("g1", Color::Black);

// 	let mut debug_pawn = Piece::new(PieceType::Pawn, Color::Black);
// 	match parsed.unwrap() {
// 		Move::Move(_, index) => {
// 			debug_pawn.force_position(index);
// 			println!("pawn index: {}", index);
// 		}
// 		_ => panic!("?"),
// 	}
// 	board.add_piece(debug_pawn);

// 	let parsed = parse_notation("nc1", Color::Black);
// 	let mut debug_knight: Piece;
// 	match parsed.unwrap() {
// 		Move::Move(data, index) => {
// 			debug_knight = Piece::new(data.piece.unwrap(), Color::Black);
// 			debug_knight.force_position(index);
// 			board.add_piece(debug_knight);
// 		}
// 		_ => panic!("?"),
// 	}

// 	println!("{board}");
// }
