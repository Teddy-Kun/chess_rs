use std::{fmt::Display, ops::Index};

use serde::{Serialize, ser::SerializeMap};

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

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
struct BoardIndex {
	i: u8,
}

impl BoardIndex {
	pub fn new(u: u8) -> Self {
		BoardIndex { i: u }
	}

	pub fn none() -> Self {
		BoardIndex { i: 255 }
	}

	pub fn get(&self) -> Option<u8> {
		if self.i > 63 { None } else { Some(self.i) }
	}
}

impl Display for BoardIndex {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self.get())
	}
}

impl Board {
	fn board_overflow_add(start: u8, b: u8, diag: bool) -> BoardIndex {
		let res = start + b;
		if res > 63 {
			return BoardIndex::none();
		}

		let res_col = res % 8;
		match diag {
			true => {
				if b > 8 {
					if res_col < start % 8 {
						return BoardIndex::none();
					}
				} else if res_col > start % 8 {
					return BoardIndex::none();
				}
			}
			false => {
				// horizontal
				if res_col < start % 8 {
					return BoardIndex::none();
				}
			}
		}

		BoardIndex::new(res)
	}

	fn board_underflow_sub(start: u8, b: u8, diag: bool) -> BoardIndex {
		let res = match start.checked_sub(b) {
			None => return BoardIndex::none(),
			Some(r) => r,
		};

		let res_col = res % 8;
		match diag {
			true => {
				if b > 8 {
					if res_col > start % 8 {
						return BoardIndex::none();
					}
				} else if res_col < start % 8 {
					return BoardIndex::none();
				}
			}
			false => {
				// horizontal
				if res_col > start % 8 {
					return BoardIndex::none();
				}
			}
		}
		BoardIndex::new(res)
	}

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
				0b1111111111111111000000000000000000000000000000001111111111111111,
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
			None => return ChessCell::empty(),
		};
		self[index]
	}

	pub fn move_piece_unchecked(&mut self, index: u8, target: u8) {
		let mut piece = self[index];
		if !piece.is_empty() {
			piece.set_moved(true);
			self.board[index as usize] = ChessCell::empty();
			self.board[target as usize] = piece;
			self.occupation.remove(index);
			self.occupation.insert(target);
		}
	}

	fn get_pawn_moves(&self, i: u8) -> BitBoard {
		let mut legal = BitBoard::new();

		let piece = self[i];
		let own_col = piece.get_color();

		let col = piece.get_color();
		let math = match col {
			Color::Black => {
				if !piece.has_moved() {
					legal.insert(i + 16); // no need to do a checked add because we haven't moved so we can't overflow
				}
				Self::board_overflow_add
			}

			Color::White => {
				if !piece.has_moved() {
					legal.insert(i - 16); // no need to do a checked sub because we haven't moved so we can't underflow
				}
				Self::board_underflow_sub
			}
		};

		if let Some(step) = math(i, 8, false).get()
			&& !self.occupation.contains(step)
		{
			legal.insert(step);
		}

		if let Some(left_take) = math(i, 7, true).get()
			&& self.occupation.contains(left_take)
			&& own_col != self[left_take].get_color()
		{
			legal.insert(left_take);
		}

		if let Some(right_take) = math(i, 9, true).get()
			&& self.occupation.contains(right_take)
			&& own_col != self[right_take].get_color()
		{
			legal.insert(right_take);
		}

		legal
	}

	fn get_knight_moves(&self, i: u8) -> BitBoard {
		let mut legal = BitBoard::new();

		// TODO: fix it being very borked

		if let Some(top_left) = Self::board_underflow_sub(i, 17, false).get() {
			legal.insert(top_left);
		}

		if let Some(top_right) = Self::board_underflow_sub(i, 15, false).get() {
			legal.insert(top_right);
		}

		if let Some(bottom_right) = Self::board_overflow_add(i, 17, false).get() {
			legal.insert(bottom_right);
		}

		if let Some(bottom_left) = Self::board_overflow_add(i, 15, false).get() {
			legal.insert(bottom_left);
		}

		if let Some(high_left) = Self::board_underflow_sub(i, 10, false).get() {
			legal.insert(high_left);
		}

		if let Some(high_right) = Self::board_underflow_sub(i, 6, false).get() {
			legal.insert(high_right);
		}

		if let Some(low_left) = Self::board_overflow_add(i, 6, false).get() {
			legal.insert(low_left);
		}

		if let Some(low_right) = Self::board_overflow_add(i, 10, false).get() {
			legal.insert(low_right);
		}

		legal
	}

	fn get_bishop_moves(&self, i: u8) -> BitBoard {
		let mut legal = BitBoard::new();
		let piece = self[i];
		let own_color = piece.get_color();

		let mut top_right_chk = i;
		while let Some(top_right) = Self::board_underflow_sub(top_right_chk, 7, true).get() {
			if self.occupation.contains(top_right) {
				if self[top_right].get_color() != own_color {
					legal.insert(top_right);
				}
				break;
			}

			legal.insert(top_right);
			top_right_chk = top_right;
		}

		let mut top_left_chk = i;
		while let Some(top_left) = Self::board_underflow_sub(top_left_chk, 9, true).get() {
			if self.occupation.contains(top_left) {
				if self[top_left].get_color() != own_color {
					legal.insert(top_left);
				}
				break;
			}

			legal.insert(top_left);
			top_left_chk = top_left;
		}

		let mut bottom_left_chk = i;
		while let Some(bottom_left) = Self::board_overflow_add(bottom_left_chk, 7, true).get() {
			if self.occupation.contains(bottom_left) {
				if self[bottom_left].get_color() != own_color {
					legal.insert(bottom_left);
				}
				break;
			}

			legal.insert(bottom_left);
			bottom_left_chk = bottom_left;
		}

		let mut bottom_right_chk = i;
		while let Some(bottom_right) = Self::board_overflow_add(bottom_right_chk, 9, true).get() {
			if self.occupation.contains(bottom_right) {
				if self[bottom_right].get_color() != own_color {
					legal.insert(bottom_right);
				}
				break;
			}

			legal.insert(bottom_right);
			bottom_right_chk = bottom_right;
		}

		legal
	}

	fn get_rook_moves(&self, i: u8) -> BitBoard {
		let mut legal = BitBoard::new();
		let piece = self[i];
		let own_color = piece.get_color();

		// top
		let mut top_chk = i;
		while let Some(top) = Self::board_underflow_sub(top_chk, 8, false).get() {
			if self.occupation.contains(top) {
				if self[top].get_color() != own_color {
					legal.insert(top);
				}
				break;
			}

			legal.insert(top);
			top_chk = top;
		}

		// bottom
		let mut bottom_chk = i;
		while let Some(bottom) = Self::board_overflow_add(bottom_chk, 8, false).get() {
			if self.occupation.contains(bottom) {
				if self[bottom].get_color() != own_color {
					legal.insert(bottom);
				}
				break;
			}

			legal.insert(bottom);
			bottom_chk = bottom;
		}

		let mut right_chk = i;
		while let Some(right) = Self::board_overflow_add(right_chk, 1, false).get() {
			if self.occupation.contains(right) {
				if self[right].get_color() != own_color {
					legal.insert(right);
				}
				break;
			}

			legal.insert(right);
			right_chk = right;
		}

		let mut left_chk = i;
		while let Some(left) = Self::board_underflow_sub(left_chk, 1, false).get() {
			if self.occupation.contains(left) {
				if self[left].get_color() != own_color {
					legal.insert(left);
				}
				break;
			}

			legal.insert(left);
			left_chk = left;
		}

		legal
	}

	fn get_queen_moves(&self, i: u8) -> BitBoard {
		let bishop = self.get_bishop_moves(i);
		bishop.union(self.get_rook_moves(i))
	}

	fn get_king_moves(&self, i: u8) -> BitBoard {
		let mut legal = BitBoard::new();

		let piece = self[i];
		let own_col = piece.get_color();

		if let Some(bottom_left) = Self::board_overflow_add(i, 7, true).get()
			&& (!self.occupation.contains(bottom_left) || self[bottom_left].get_color() != own_col)
		{
			legal.insert(bottom_left);
		};

		if let Some(bottom) = Self::board_overflow_add(i, 8, false).get()
			&& (!self.occupation.contains(bottom) || self[bottom].get_color() != own_col)
		{
			legal.insert(bottom);
		};

		if let Some(bottom_right) = Self::board_overflow_add(i, 9, true).get()
			&& (!self.occupation.contains(bottom_right)
				|| self[bottom_right].get_color() != own_col)
		{
			legal.insert(bottom_right);
		};

		if let Some(left) = Self::board_underflow_sub(i, 1, false).get()
			&& (!self.occupation.contains(left) || self[left].get_color() != own_col)
		{
			legal.insert(left);
		};

		if let Some(right) = Self::board_overflow_add(i, 1, false).get()
			&& (!self.occupation.contains(right) || self[right].get_color() != own_col)
		{
			legal.insert(right);
		};

		if let Some(top_left) = Self::board_underflow_sub(i, 9, true).get()
			&& (!self.occupation.contains(top_left) || self[top_left].get_color() != own_col)
		{
			legal.insert(top_left);
		};

		if let Some(top) = Self::board_underflow_sub(i, 8, false).get()
			&& (!self.occupation.contains(top) || self[top].get_color() != own_col)
		{
			legal.insert(top);
		};

		if let Some(top_right) = Self::board_underflow_sub(i, 7, true).get()
			&& (!self.occupation.contains(top_right) || self[top_right].get_color() != own_col)
		{
			legal.insert(top_right);
		};

		let king = self[i];
		if !king.has_moved() {
			// TODO: test

			let kingside_i: u8;
			let queenside_i: u8;
			let kingside_mask: BitBoard;
			let queenside_mask: BitBoard;

			if own_col == Color::Black {
				kingside_i = 0;
				queenside_i = 7;
				kingside_mask = BitBoard::from(
					0b0000000000000000000000000000000000000000000000000110000000000000,
				);
				queenside_mask = BitBoard::from(
					0b0000000000000000000000000000000000000000000000000000000000001110,
				);
			} else {
				// white
				kingside_i = 56;
				queenside_i = 63;
				kingside_mask = BitBoard::from(
					0b0110000000000000000000000000000000000000000000000000000000000000,
				);
				queenside_mask = BitBoard::from(
					0b0000000000001110000000000000000000000000000000000000000000000000,
				);
			}

			let kingside = self[kingside_i];
			if kingside.get_color() == own_col
				&& !kingside.has_moved()
				&& kingside_mask.join(self.occupation).is_empty()
			{
				legal.insert(kingside_i);
			}

			let queenside = self[queenside_i];
			if queenside.get_color() == own_col
				&& !queenside.has_moved()
				&& queenside_mask.join(self.occupation).is_empty()
			{
				legal.insert(queenside_i);
			}
		}

		legal
	}

	pub fn get_legal_moves(&self, i: u8) -> BitBoard {
		let c = self[i];
		match c.get_type() {
			Some(PieceType::Pawn) => self.get_pawn_moves(i),
			Some(PieceType::Knight) => self.get_knight_moves(i),
			Some(PieceType::Bishop) => self.get_bishop_moves(i),
			Some(PieceType::Rook) => self.get_rook_moves(i),
			Some(PieceType::Queen) => self.get_queen_moves(i),
			Some(PieceType::King) => self.get_king_moves(i),
			_ => BitBoard::new(),
		}
	}
}

impl Index<u8> for Board {
	type Output = ChessCell;
	fn index(&self, index: u8) -> &Self::Output {
		&self.board[index as usize]
	}
}

impl Serialize for Board {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		let mut state = serializer.serialize_map(Some(self.occupation.size() as usize))?;
		for (i, cell) in self.board.iter().enumerate() {
			if !cell.is_empty() {
				state.serialize_entry(&i, cell)?;
			}
		}
		state.end()
	}
}
