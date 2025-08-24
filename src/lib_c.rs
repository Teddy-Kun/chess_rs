use crate::{
	// board::Board,
	piece::{Color, Piece, PieceType},
};

#[unsafe(no_mangle)]
pub extern "C" fn get_color(piece: &Piece) -> Color {
	piece.get_color()
}

#[unsafe(no_mangle)]
pub extern "C" fn get_type(piece: &Piece) -> PieceType {
	piece.get_type()
}

// #[unsafe(no_mangle)]
// pub extern "C" fn new_board() -> Board {
// 	Board::new()
// }
