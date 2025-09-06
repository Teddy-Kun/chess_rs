use std::ops::Index;

use crate::{
	bitboard::BitBoard,
	piece::{ChessCell, Color, Moved, PieceType},
};

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Board {
	board: [ChessCell; 64],
	occupation: BitBoard,
}

impl Default for Board {
	fn default() -> Self {
		Self::new()
	}
}

impl Board {
	fn coords_to_index(x: u8, y: u8) -> Option<u8> {
		if x < 8 && y < 8 {
			let y = (y as i8 - 7).unsigned_abs();
			Some(y * 8 + x)
		} else {
			None
		}
	}

	pub fn notation_to_x(x: char) -> Option<u8> {
		let x = (x as u8).wrapping_sub(97);
		if x < 8 { Some(x) } else { None }
	}

	pub fn notation_to_index(x: char, y: u8) -> Option<u8> {
		// convert 'a' into 0. We don't care if any overflow or anything else occurs here. If its anything but 'a'-'h' it should just return a none
		let x = (x as u8).wrapping_sub(97);
		Self::coords_to_index(x, y - 1)
	}

	pub fn empty() -> Self {
		Board {
			board: [ChessCell::default(); 64],
			occupation: BitBoard::new(),
		}
	}

	pub fn add_piece(&mut self, piece: ChessCell, position: u8) {
		self.board[position as usize] = piece;
		self.occupation.insert(position);
	}

	pub fn new() -> Self {
		let mut board: [ChessCell; 64] = [ChessCell::default(); 64];
		// init white pieces
		{
			let w_rook = ChessCell::with_piece(PieceType::Rook, Color::White, Moved::No);
			board[63] = w_rook;

			let w_knight = ChessCell::with_piece(PieceType::Knight, Color::White, Moved::No);
			board[62] = w_knight;

			let w_bishop = ChessCell::with_piece(PieceType::Bishop, Color::White, Moved::No);
			board[61] = w_bishop;

			let w_queen = ChessCell::with_piece(PieceType::Queen, Color::White, Moved::No);
			board[60] = w_queen;

			let w_king = ChessCell::with_piece(PieceType::King, Color::White, Moved::No);
			board[59] = w_king;

			board[58] = w_bishop;
			board[57] = w_knight;
			board[56] = w_rook;

			let w_pawn = ChessCell::with_piece(PieceType::Pawn, Color::White, Moved::No);
			board[55] = w_pawn;
			board[54] = w_pawn;
			board[53] = w_pawn;
			board[52] = w_pawn;
			board[51] = w_pawn;
			board[50] = w_pawn;
			board[49] = w_pawn;
			board[48] = w_pawn;
		}

		// init black pieces
		{
			let w_rook = ChessCell::with_piece(PieceType::Rook, Color::Black, Moved::No);
			board[0] = w_rook;

			let w_knight = ChessCell::with_piece(PieceType::Knight, Color::Black, Moved::No);
			board[1] = w_knight;

			let w_bishop = ChessCell::with_piece(PieceType::Bishop, Color::Black, Moved::No);
			board[2] = w_bishop;

			let w_queen = ChessCell::with_piece(PieceType::Queen, Color::Black, Moved::No);
			board[3] = w_queen;

			let w_king = ChessCell::with_piece(PieceType::King, Color::Black, Moved::No);
			board[4] = w_king;

			board[5] = w_bishop;
			board[6] = w_knight;
			board[7] = w_rook;

			let w_pawn = ChessCell::with_piece(PieceType::Pawn, Color::Black, Moved::No);
			board[8] = w_pawn;
			board[9] = w_pawn;
			board[10] = w_pawn;
			board[11] = w_pawn;
			board[12] = w_pawn;
			board[13] = w_pawn;
			board[14] = w_pawn;
			board[15] = w_pawn;
		}

		Self {
			board,
			occupation: BitBoard::from(
				// this is the inital position of all pieces on the board
				0b111111111111111000000000000000000000000000000001111111111111111,
			),
		}
	}

	pub fn reset(&mut self) {
		let n = Self::new();
		self.board = n.board;
		self.occupation = n.occupation;
	}

	pub fn get_at_position(&self, x: u8, y: u8) -> ChessCell {
		let index = match Self::coords_to_index(x, y) {
			Some(i) => i,
			None => return ChessCell::new(),
		};
		self.board[index as usize]
	}

	// pub fn force_move_piece(&mut self, index: u8, target: u8) {
	// 	let piece = self.board[index as usize];

	// 	if let Some(color) = piece.get_color() {
	// 		self.board[index as usize] = BoardOption::default();
	// 		self.board[target as usize] = BoardOption::new(index, color);
	// 		self.occupation.remove(index);
	// 		self.occupation.insert(target);
	// 		let index = piece.get();
	// 		let piece = match color {
	// 			Color::Black => &mut self.black[index as usize],
	// 			Color::White => &mut self.white[index as usize],
	// 		};

	// 		piece.force_position(target);
	// 	}
	// }

	// pub fn castle(far: bool) -> bool {
	// 	// TODO
	// 	false
	// }

	pub fn get_pieces(&self) -> Box<[ChessCell]> {
		let pieces: Vec<ChessCell> = self
			.board
			.iter()
			.filter_map(|c| match c.is_empty() {
				true => None,
				false => Some(*c),
			})
			.collect();
		pieces.into_boxed_slice()
	}

	// pub fn get_legal_moves(&mut self, index: u8) -> BitBoard {
	// 	BitBoard::new()
	// }
}

impl Index<u8> for Board {
	type Output = ChessCell;
	fn index(&self, index: u8) -> &Self::Output {
		&self.board[index as usize]
	}
}
