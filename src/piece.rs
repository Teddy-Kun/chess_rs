#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i8)]
pub enum PieceType {
	Pawn,
	Knight,
	Bishop,
	Rook,
	Queen,
	King,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i8)]
pub enum Color {
	White = 0b00000000,
	Black = 0b00001000,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Piece {
	piece: i8,
	position: u8,
}

impl Piece {
	pub fn new(pt: PieceType, col: Color) -> Self {
		Self {
			piece: pt as i8 | col as i8,
			position: 0,
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

	pub fn get_i8(&self) -> i8 {
		self.piece
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
