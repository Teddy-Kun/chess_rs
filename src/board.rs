use std::fmt::Display;

use crate::piece::{Color, Piece, PieceType};

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
// Index into the White or Black array of the board
struct BoardOption(u8);

impl BoardOption {
	pub const NONE: u8 = u8::MAX;
	const BLACK_FLAG: u8 = 0b01000000;

	pub fn new(inner: u8, col: Color) -> Self {
		if inner >= 16 {
			return Self(Self::NONE);
		}

		match col {
			Color::White => Self(inner),
			Color::Black => Self(inner | Self::BLACK_FLAG),
		}
	}

	fn is_none(&self) -> bool {
		self.0 == Self::NONE
	}

	fn get_color(&self) -> Option<Color> {
		if self.is_none() {
			return None;
		}

		if (self.0 & Self::BLACK_FLAG) > 0 {
			Some(Color::Black)
		} else {
			Some(Color::White)
		}
	}

	fn get(&self) -> u8 {
		if self.is_none() {
			Self::NONE
		} else {
			self.0 & 0b10111111 // strip black flag
		}
	}
}

impl Default for BoardOption {
	fn default() -> Self {
		Self(Self::NONE)
	}
}

#[repr(C)]
#[derive(Debug)]
pub struct Board {
	board: [BoardOption; 64],

	white: [Piece; 16],
	black: [Piece; 16],
}

impl Default for Board {
	fn default() -> Self {
		Self::new()
	}
}

impl Board {
	fn coords_to_index(x: u8, y: u8) -> Option<usize> {
		if x < 8 && y < 8 {
			Some((y as usize) * 8 + (x as usize))
		} else {
			None
		}
	}

	pub fn new() -> Self {
		let mut board: [BoardOption; 64] = [BoardOption::default(); 64];
		let white: [Piece; 16];
		let black: [Piece; 16];

		// init white pieces
		{
			let mut w_rook = Piece::new(PieceType::Rook, Color::White);
			w_rook.force_position(63);

			let mut w_knight = Piece::new(PieceType::Knight, Color::White);
			w_knight.force_position(62);

			let mut w_bishop = Piece::new(PieceType::Bishop, Color::White);
			w_bishop.force_position(61);

			let mut w_queen = Piece::new(PieceType::Queen, Color::White);
			w_queen.force_position(60);

			let mut w_king = Piece::new(PieceType::King, Color::White);
			w_king.force_position(59);

			let mut w_bishop_2 = w_bishop;
			w_bishop_2.force_position(58);

			let mut w_knight_2 = w_knight;
			w_knight_2.force_position(57);

			let mut w_rook_2 = w_rook;
			w_rook_2.force_position(56);

			let mut w_pawn = Piece::new(PieceType::Pawn, Color::White);
			w_pawn.force_position(55);

			let mut w_pawn_2 = w_pawn;
			w_pawn_2.force_position(54);

			let mut w_pawn_3 = w_pawn;
			w_pawn_3.force_position(53);

			let mut w_pawn_4 = w_pawn;
			w_pawn_4.force_position(52);

			let mut w_pawn_5 = w_pawn;
			w_pawn_5.force_position(51);

			let mut w_pawn_6 = w_pawn;
			w_pawn_6.force_position(50);

			let mut w_pawn_7 = w_pawn;
			w_pawn_7.force_position(49);

			let mut w_pawn_8 = w_pawn;
			w_pawn_8.force_position(48);

			white = [
				w_rook, w_knight, w_bishop, w_queen, w_king, w_bishop_2, w_knight_2, w_rook_2,
				w_pawn, w_pawn_2, w_pawn_3, w_pawn_4, w_pawn_5, w_pawn_6, w_pawn_7, w_pawn_8,
			];
		}

		// init black pieces
		{
			let mut b_rook = Piece::new(PieceType::Rook, Color::Black);
			b_rook.force_position(0);

			let mut b_knight = Piece::new(PieceType::Knight, Color::Black);
			b_knight.force_position(1);

			let mut b_bishop = Piece::new(PieceType::Bishop, Color::Black);
			b_bishop.force_position(2);

			let mut b_king = Piece::new(PieceType::King, Color::Black);
			b_king.force_position(3);

			let mut b_queen = Piece::new(PieceType::Queen, Color::Black);
			b_queen.force_position(4);

			let mut b_bishop_2 = b_bishop;
			b_bishop_2.force_position(5);

			let mut b_knight_2 = b_knight;
			b_knight_2.force_position(6);

			let mut b_rook_2 = b_rook;
			b_rook_2.force_position(7);

			let mut b_pawn = Piece::new(PieceType::Pawn, Color::Black);
			b_pawn.force_position(8);

			let mut b_pawn_2 = b_pawn;
			b_pawn_2.force_position(9);

			let mut b_pawn_3 = b_pawn;
			b_pawn_3.force_position(10);

			let mut b_pawn_4 = b_pawn;
			b_pawn_4.force_position(11);

			let mut b_pawn_5 = b_pawn;
			b_pawn_5.force_position(12);

			let mut b_pawn_6 = b_pawn;
			b_pawn_6.force_position(13);

			let mut b_pawn_7 = b_pawn;
			b_pawn_7.force_position(14);

			let mut b_pawn_8 = b_pawn;
			b_pawn_8.force_position(15);

			black = [
				b_rook, b_knight, b_bishop, b_queen, b_king, b_bishop_2, b_knight_2, b_rook_2,
				b_pawn, b_pawn_2, b_pawn_3, b_pawn_4, b_pawn_5, b_pawn_6, b_pawn_7, b_pawn_8,
			];
		}

		// assign board values
		{
			for (i, w) in white.iter().enumerate() {
				let index = w.get_position() as usize;
				board[index] = BoardOption::new(i as u8, Color::White);
			}

			for (i, b) in black.iter().enumerate() {
				let index = b.get_position() as usize;
				board[index] = BoardOption::new(i as u8, Color::Black);
			}
		}

		Self {
			board,
			white,
			black,
		}
	}

	fn get_from_option(&self, option: BoardOption) -> Option<Piece> {
		let col = option.get_color()?; // auto returns None on empty cell
		let index = option.get() as usize;

		match col {
			Color::Black => Some(self.black[index]),
			Color::White => Some(self.white[index]),
		}
	}

	pub fn get_at_position(&self, x: u8, y: u8) -> Option<Piece> {
		let index = Self::coords_to_index(x, y)?;
		let piece = self.board[index];

		let color = piece.get_color()?;
		let index = piece.get();

		match color {
			Color::Black => Some(self.black[index as usize]),
			Color::White => Some(self.white[index as usize]),
		}
	}
}

impl Display for Board {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		const BORDER: &str = "  +-+-+-+-+-+-+-+-+";

		let into_cell_row = |row: [&BoardOption; 8], row_i: usize| -> String {
			let mut piece_chars: [char; 8] = [' '; 8];

			for (i, elem) in row.iter().enumerate() {
				let piece = self.get_from_option(**elem);
				let ch: char = match piece {
					None => continue,
					Some(p) => p.into(),
				};

				piece_chars[i] = ch;
			}

			let row_index = ((row_i as i8) - 8).abs();

			format!(
				"{} |{}|{}|{}|{}|{}|{}|{}|{}| {}\n",
				row_index,
				piece_chars[0],
				piece_chars[1],
				piece_chars[2],
				piece_chars[3],
				piece_chars[4],
				piece_chars[5],
				piece_chars[6],
				piece_chars[7],
				row_index,
			)
		};

		let min_string = String::with_capacity(0);
		let mut str_rows: [String; 8] = [
			min_string.clone(),
			min_string.clone(),
			min_string.clone(),
			min_string.clone(),
			min_string.clone(),
			min_string.clone(),
			min_string.clone(),
			min_string,
		];

		for (i, row) in self.board.iter().array_chunks::<8>().enumerate() {
			str_rows[i] = into_cell_row(row, i);
		}

		let mut out = String::with_capacity(457);

		const ROW_ABC: &str = "   a b c d e f g h";
		out += ROW_ABC;
		out.push('\n');

		for row in str_rows {
			out += BORDER;
			out.push('\n');
			out += row.as_str();
		}
		out += BORDER;
		out.push('\n');
		out += ROW_ABC;

		write!(f, "{out}")
	}
}
