use serde::{Serialize, ser::SerializeStruct};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[repr(u8)]
pub enum PieceType {
	Pawn = 0b00000001,
	Knight = 0b00000010,
	Bishop = 0b00000011,
	Rook = 0b00000100,
	Queen = 0b00000101,
	King = 0b00000110,
}

impl TryFrom<u8> for PieceType {
	type Error = ();

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			0b00000001 => Ok(PieceType::Pawn),
			0b00000010 => Ok(PieceType::Knight),
			0b00000011 => Ok(PieceType::Bishop),
			0b00000100 => Ok(PieceType::Rook),
			0b00000101 => Ok(PieceType::Queen),
			0b00000110 => Ok(PieceType::King),
			_ => Err(()),
		}
	}
}

impl TryFrom<char> for PieceType {
	type Error = ();

	fn try_from(value: char) -> Result<Self, Self::Error> {
		match value.to_ascii_lowercase() {
			'P' => Ok(PieceType::Pawn),
			'N' => Ok(PieceType::Knight),
			'B' => Ok(PieceType::Bishop),
			'R' => Ok(PieceType::Rook),
			'Q' => Ok(PieceType::Queen),
			'K' => Ok(PieceType::King),
			_ => Err(()),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[repr(u8)]
pub enum Color {
	White = 0b00000000,
	Black = 0b00001000,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[repr(u8)]
pub enum Moved {
	No = 0b00000000,
	Yes = 0b00010000,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ChessCell {
	piece: u8,
}

impl ChessCell {
	pub fn new() -> Self {
		Self { piece: 0 }
	}

	pub fn is_empty(&self) -> bool {
		self.piece == 0
	}

	pub fn with_piece(pt: PieceType, col: Color, moved: Moved) -> Self {
		Self {
			piece: pt as u8 | col as u8 | moved as u8,
		}
	}

	pub fn set_piece(&mut self, pt: PieceType, col: Color, moved: Moved) {
		self.piece = pt as u8 | col as u8 | moved as u8;
	}

	/// Returns None if the piece is empty
	pub fn get_color(&self) -> Option<Color> {
		if self.is_empty() {
			None
		} else if (self.piece & Color::Black as u8) > 0 {
			Some(Color::Black)
		} else {
			Some(Color::White)
		}
	}

	/// Returns None if the piece is empty
	pub fn has_moved(&self) -> Option<bool> {
		if self.is_empty() {
			None
		} else {
			Some(self.piece & (Moved::Yes as u8) != 0)
		}
	}

	/// Returns None if the piece is empty or if we somehow have 0b00000111 as the final bits, the latter of which should never happen
	pub fn get_type(&self) -> Option<PieceType> {
		PieceType::try_from(self.piece & 0b00000111).ok()
	}

	pub fn clear(&mut self) {
		self.piece = 0
	}

	pub fn set_type(&mut self, t: PieceType) {
		// set type and readd higher bits
		self.piece = t as u8 | (self.piece & 0b11111000)
	}

	pub fn set_color(&mut self, col: Color) {
		self.piece = (self.piece & !(Color::Black as u8)) | col as u8
	}

	pub fn set_moved(&mut self, moved: bool) {
		match moved {
			true => self.piece |= Moved::Yes as u8,
			false => self.piece &= !(Moved::Yes as u8),
		}
	}
}

impl Serialize for ChessCell {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		let color = self.get_color();
		let pt = self.get_type();

		let mut state = serializer.serialize_struct("Piece", 3)?;
		state.serialize_field("type", &pt)?;
		state.serialize_field("color", &color)?;
		state.serialize_field("has_moved", &self.has_moved())?;
		state.end()
	}
}

impl From<ChessCell> for char {
	fn from(value: ChessCell) -> Self {
		let c = match value.get_type() {
			Some(PieceType::Pawn) => '♙',
			Some(PieceType::Knight) => '♘',
			Some(PieceType::Bishop) => '♗',
			Some(PieceType::Rook) => '♖',
			Some(PieceType::Queen) => '♕',
			Some(PieceType::King) => '♔',
			_ => return ' ',
		};

		match value.get_color().unwrap() {
			Color::Black => unsafe { char::from_u32_unchecked(c as u32 + 6) },
			Color::White => c,
		}
	}
}

impl Default for ChessCell {
	fn default() -> Self {
		Self::new()
	}
}

#[cfg(test)]
mod tests {
	use crate::piece::{ChessCell, Color, Moved, PieceType};

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
				let piece = ChessCell::with_piece(piece_type, col, Moved::Yes);
				assert_eq!(piece.get_color(), Some(col));
				assert_eq!(piece.get_type(), Some(piece_type));
				assert_eq!(piece.has_moved(), Some(true));
			}
		}
	}
}
