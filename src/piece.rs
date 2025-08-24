use std::{error::Error, fmt::Display};

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

	pub fn try_to_move(&mut self, target: u8) {
		// TODO: check if the move is legal
		self.has_moved = true;
		self.force_position(target);
	}

	pub fn force_position(&mut self, position: u8) {
		self.position = position
	}

	pub fn get_position(&self) -> u8 {
		self.position
	}
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
