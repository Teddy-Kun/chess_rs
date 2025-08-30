use std::{error::Error, fmt::Display};

use crate::board_hash::BoardHash;

#[derive(Debug)]
pub struct PieceTypeError {
	invalid_char: char,
}

impl Display for PieceTypeError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"'{}' is not valid chess piece notation",
			self.invalid_char
		)
	}
}

impl Error for PieceTypeError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PieceType {
	Pawn,
	Knight,
	Bishop,
	Rook,
	Queen,
	King,
}

impl TryFrom<char> for PieceType {
	type Error = PieceTypeError;

	fn try_from(value: char) -> Result<Self, Self::Error> {
		match value.to_ascii_lowercase() {
			'P' => Ok(PieceType::Pawn),
			'N' => Ok(PieceType::Knight),
			'B' => Ok(PieceType::Bishop),
			'R' => Ok(PieceType::Rook),
			'Q' => Ok(PieceType::Queen),
			'K' => Ok(PieceType::King),
			_ => Err(PieceTypeError {
				invalid_char: value,
			}),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
	White = 0b00000000,
	Black = 0b00001000,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Piece {
	piece: u8,
	position: u8,
	has_moved: bool,
}

impl Piece {
	pub fn new(pt: PieceType, col: Color) -> Self {
		Self {
			piece: pt as u8 | col as u8,
			position: 0,
			has_moved: false,
		}
	}

	pub fn get_color(&self) -> Color {
		if (self.piece & 0b00001000) > 0 {
			Color::Black
		} else {
			Color::White
		}
	}

	pub fn get_type(&self) -> PieceType {
		match self.piece & 0b00000111 {
			0 => PieceType::Pawn,
			1 => PieceType::Knight,
			2 => PieceType::Bishop,
			3 => PieceType::Rook,
			4 => PieceType::Queen,
			5 => PieceType::King,
			x => unreachable!("Illegal piece {}", x),
		}
	}

	pub fn get_i8(&self) -> u8 {
		self.piece
	}

	pub fn try_to_move(&mut self, target: u8) -> bool {
		if self.get_legal_moves().contains(target) {
			self.has_moved = true;
			self.force_position(target);
			true
		} else {
			false
		}
	}

	pub fn force_position(&mut self, position: u8) {
		self.position = position
	}

	pub fn get_position(&self) -> u8 {
		self.position
	}

	pub fn get_legal_moves(&self) -> BoardHash {
		// TODO: consider captures somehow
		match self.get_type() {
			PieceType::Pawn => chk_move_pawn(self.get_position(), self.get_color(), self.has_moved),
			PieceType::Knight => chk_move_knight(self.get_position()),
			PieceType::Bishop => chk_move_bishop(self.get_position()),
			PieceType::Rook => chk_move_rook(self.get_position()),
			PieceType::Queen => chk_move_queen(self.get_position()),
			PieceType::King => chk_move_king(self.get_position()),
		}
	}
}

fn piece_checked_add(current_pos: u8, target_pos: u8) -> Option<u8> {
	let maybe = current_pos.checked_add(target_pos)?;
	if maybe > 63 { None } else { Some(maybe) }
}

fn chk_move_pawn(current_pos: u8, color: Color, has_moved: bool) -> BoardHash {
	let mut res = BoardHash::new();

	if !has_moved {
		res.insert(match color {
			// can be unchecked since we have not moved
			Color::White => current_pos - 16,
			Color::Black => current_pos + 16,
		});
	}

	let maybe = match color {
		Color::White => current_pos.checked_sub(8),
		Color::Black => piece_checked_add(current_pos, 8),
	};

	if let Some(legal) = maybe {
		res.insert(legal);
	}

	// TODO: check taking and fucking en passant

	res
}

fn chk_move_knight(current_pos: u8) -> BoardHash {
	let mut res = BoardHash::new();

	if let Some(top_left) = current_pos.checked_sub(17) {
		res.insert(top_left);
	};

	if let Some(top_right) = current_pos.checked_sub(15) {
		res.insert(top_right);
	};

	if let Some(bottom_left) = piece_checked_add(current_pos, 17) {
		res.insert(bottom_left);
	};

	if let Some(bottom_right) = piece_checked_add(current_pos, 15) {
		res.insert(bottom_right);
	};

	if let Some(high_left) = current_pos.checked_sub(10) {
		res.insert(high_left);
	};

	if let Some(high_right) = current_pos.checked_sub(6) {
		res.insert(high_right);
	};

	if let Some(low_left) = piece_checked_add(current_pos, 6) {
		res.insert(low_left);
	};

	if let Some(low_right) = piece_checked_add(current_pos, 10) {
		res.insert(low_right);
	};

	res
}

fn chk_move_bishop(current_pos: u8) -> BoardHash {
	let mut res = BoardHash::new();

	// top left
	let mut check_next = current_pos;
	loop {
		let prev = check_next;
		match check_next.checked_sub(9) {
			None => break,
			Some(next) => {
				let next_row = next / 8;
				if next_row == current_pos / 8 || next_row.abs_diff(prev / 8) != 1 {
					break;
				}
				res.insert(next);
				check_next = next;
			}
		}
	}

	// top right
	check_next = current_pos;
	loop {
		let prev = check_next;
		match check_next.checked_sub(7) {
			None => break,
			Some(next) => {
				let next_row = next / 8;
				if next_row == current_pos / 8 || next_row.abs_diff(prev / 8) != 1 {
					break;
				}
				res.insert(next);
				check_next = next;
			}
		}
	}

	// bottom left
	check_next = current_pos;
	loop {
		let prev = check_next;
		match piece_checked_add(check_next, 7) {
			None => break,
			Some(next) => {
				let next_row = next / 8;
				if next_row == current_pos / 8 || next_row.abs_diff(prev / 8) != 1 {
					break;
				}
				res.insert(next);
				check_next = next;
			}
		}
	}

	// bottom right
	check_next = current_pos;
	loop {
		let prev = check_next;
		match piece_checked_add(check_next, 9) {
			None => break,
			Some(next) => {
				let next_row = next / 8;
				if next_row == current_pos / 8 || next_row.abs_diff(prev / 8) != 1 {
					break;
				}
				res.insert(next);
				check_next = next;
			}
		}
	}

	res
}

fn chk_move_rook(current_pos: u8) -> BoardHash {
	let mut res = BoardHash::new();

	let current_row = current_pos % 8;
	println!("current_row {current_row}");
	let mut next = current_pos;
	loop {
		next = match piece_checked_add(next, 1) {
			Some(n) => n,
			None => break,
		};

		let next_col = next % 8;
		if next_col == 0 {
			// rows changes
			break;
		}

		res.insert(next);
	}

	next = current_pos;
	loop {
		next = match next.checked_sub(1) {
			Some(n) => n,
			None => break,
		};

		let next_col = next % 8;
		if next_col == 7 {
			// rows changes
			break;
		}

		res.insert(next);
	}

	next = current_pos;
	loop {
		next = match next.checked_sub(8) {
			Some(n) => n,
			None => break,
		};
		res.insert(next);
	}

	next = current_pos;
	loop {
		next = match piece_checked_add(next, 8) {
			Some(n) => n,
			None => break,
		};
		res.insert(next);
	}

	res
}

fn chk_move_queen(current_pos: u8) -> BoardHash {
	let mut moves = chk_move_bishop(current_pos);
	moves.union(chk_move_rook(current_pos));
	moves
}

fn chk_move_king(current_pos: u8) -> BoardHash {
	// for efficiency reasons castling should be checked on the board

	let mut res = BoardHash::new();
	if let Some(bottom_left) = piece_checked_add(current_pos, 7) {
		res.insert(bottom_left);
	};

	if let Some(bottom) = piece_checked_add(current_pos, 8) {
		res.insert(bottom);
	};

	if let Some(bottom_right) = piece_checked_add(current_pos, 9) {
		res.insert(bottom_right);
	};

	if let Some(left) = current_pos.checked_sub(1) {
		res.insert(left);
	};

	if let Some(right) = piece_checked_add(current_pos, 1) {
		res.insert(right);
	};

	if let Some(top_left) = current_pos.checked_sub(9) {
		res.insert(top_left);
	};

	if let Some(top) = current_pos.checked_sub(8) {
		res.insert(top);
	};

	if let Some(top_right) = current_pos.checked_sub(7) {
		res.insert(top_right);
	};

	res
}

impl From<Piece> for char {
	fn from(value: Piece) -> Self {
		let c = match value.get_type() {
			PieceType::Pawn => '♙',
			PieceType::Knight => '♘',
			PieceType::Bishop => '♗',
			PieceType::Rook => '♖',
			PieceType::Queen => '♕',
			PieceType::King => '♔',
		};

		match value.get_color() {
			Color::Black => unsafe { char::from_u32_unchecked(c as u32 + 6) },
			Color::White => c,
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::piece::{Color, Piece, PieceType};

	#[test]
	fn test_pieces() {
		let all_types = [
			PieceType::Pawn,
			PieceType::Knight,
			PieceType::Bishop,
			PieceType::Rook,
			PieceType::Queen,
			PieceType::King,
		];
		let colors = [Color::White, Color::Black];

		for col in colors {
			for piece_type in all_types {
				let piece = Piece::new(piece_type, col);
				assert_eq!(piece.get_color(), col);
				assert_eq!(piece.get_type(), piece_type);
			}
		}
	}
}
