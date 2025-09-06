use bitflags::bitflags;

use crate::{
	board::Board,
	piece::{ChessCell, Color, Moved, PieceType},
};

bitflags! {
	#[derive(Debug, Clone, Copy)]
	pub struct SpecialMove: u8 {
		const CAPTURE = 1;
		const PROMOTE_KNIGHT = 1 << 1;
		const PROMOTE_BISHOP = 1 << 2;
		const PROMOTE_ROOK = 1 << 3;
		const PROMOTE_QUEEN = 1 << 4;
	}
}

impl Default for SpecialMove {
	fn default() -> Self {
		Self::empty()
	}
}

#[derive(Debug, Default, Clone, Copy)]
pub struct MoveData {
	pub x: Option<char>,
	pub y: Option<u8>,
	pub piece: Option<PieceType>,
	pub special: SpecialMove,
}

#[derive(Debug, Clone, Copy)]
pub enum GameResult {
	White,
	Black,
	Draw,
}

#[derive(Debug, Clone, Copy)]
pub enum Move {
	Move(MoveData, u8),
	Castle(MoveData, u8),
	Check(MoveData, u8),
	Checkmate(MoveData, u8),
	End(GameResult),
}

fn get_index_from_chars(chars: [char; 2]) -> Option<u8> {
	let x = chars[0];
	let y = chars[1].to_digit(9)? as u8;

	let index = Board::notation_to_index(x, y)?;
	Some(index)
}

pub fn start_notation(notation: &str, col: Color) -> Option<Board> {
	let mut board = Board::empty();

	for piece_str in notation.split(' ') {
		if piece_str.len() != 3 {
			return None;
		}

		let chars: [char; 3] = piece_str.chars().collect::<Vec<char>>().try_into().ok()?;
		let ptype = PieceType::try_from(chars[0]).ok()?;

		let index = get_index_from_chars([chars[1], chars[2]])?;

		let piece = ChessCell::with_piece(ptype, col, Moved::No);

		board.add_piece(piece, index);
	}

	Some(board)
}

pub fn parse_notation(notation: &str, color: Color) -> Option<Move> {
	let mut notation = notation.to_string();
	// converts our String into either a [char; 2] or a Vec<char>
	// if its a [char; 2] we know it can only be a pawn moving and doing nothing else
	let mut chars = match notation.chars().collect::<Vec<char>>().try_into() {
		Ok(notation) => return parse_simple_pawn(notation),
		Err(v) => v,
	};

	let castle_check = notation.replace("o", "0");
	{
		let start_row: u8 = match color {
			Color::Black => 8,
			Color::White => 1,
		};

		if castle_check == "0-0" {
			let index = Board::notation_to_index('g', start_row).unwrap();

			return Some(Move::Castle(
				MoveData {
					x: Some('e'),
					y: Some(start_row),
					piece: Some(PieceType::King),
					special: SpecialMove::empty(),
				},
				index,
			));
		} else if castle_check == "0-0-0" {
			let index = Board::notation_to_index('c', start_row).unwrap();
			return Some(Move::Castle(
				MoveData {
					x: Some('e'),
					y: Some(start_row),
					piece: Some(PieceType::King),
					special: SpecialMove::empty(),
				},
				index,
			));
		}
	}

	// end result check
	if notation.contains('-') {
		// 1-0; 0-1 indicates that someone won. The only other time a '-' occurs in the notation is castling which was checked before
		let half_count = chars.iter().filter(|c| **c == '½').count();
		if half_count == 2 {
			return Some(Move::End(GameResult::Draw));
		}

		if chars[0] == '1' || chars[0] == '½' || chars[0] == '+' {
			return Some(Move::End(GameResult::White));
		} else {
			return Some(Move::End(GameResult::Black));
		}
	}

	chars = match chars.try_into() {
		Ok(notation) => return parse_simple_piece(notation),
		Err(v) => v,
	};

	let ptype = PieceType::try_from(chars[0]).ok()?;

	let mut special: SpecialMove = SpecialMove::empty();
	if notation.contains('x') {
		special.set(SpecialMove::CAPTURE, true);
	};

	// if a pawn gets promoted, the piece type it becomes can be wrapped in brackets, since this is optional, remove brackets and continue
	if notation.contains(')') {
		notation.remove_matches(')');
		notation.remove_matches('(');
		chars = notation.chars().collect();
	}

	let last = *chars.last().unwrap();
	if let Ok(p) = PieceType::try_from(last) {
		match p {
			PieceType::Knight => special.set(SpecialMove::PROMOTE_KNIGHT, true),
			PieceType::Bishop => special.set(SpecialMove::PROMOTE_BISHOP, true),
			PieceType::Rook => special.set(SpecialMove::PROMOTE_ROOK, true),
			PieceType::Queen => special.set(SpecialMove::PROMOTE_QUEEN, true),
			_ => return None,
		}
	};

	let mut data = MoveData {
		x: None,
		y: None,
		piece: Some(ptype),
		special,
	};

	let mut index = u8::MAX;

	let is_check = notation.contains('+');
	let is_mate = notation.contains('#');

	let mut temp_x: Option<char> = None;
	let mut temp_y: Option<u8> = None;
	for char in chars.iter().skip(1) {
		match char.to_digit(9) {
			Some(y) => {
				if let Some(temp_y_data) = temp_y {
					if data.y.is_none() {
						data.y = Some(temp_y_data);
						temp_y = Some(y as u8);
					} else {
						return None;
					}
				} else {
					temp_y = Some(y as u8)
				}
			}
			None => {
				if Board::notation_to_x(*char).is_some()
					&& let Some(temp_x_data) = temp_x
				{
					if data.x.is_none() {
						data.x = Some(temp_x_data);
						temp_x = Some(*char);
					} else {
						return None;
					}
				} else {
					temp_x = Some(*char);
				}
				// we ignore other illegal characters, those might be actually garbage, but we also have stuff like 'x' for capturing
			}
		}
	}

	if let Some(x) = temp_x
		&& let Some(y) = temp_y
	{
		// we need both an x and a y so that the move done has a valid target
		index = Board::notation_to_index(x, y)?;
	}

	if index == u8::MAX {
		return None;
	}

	if is_check {
		Some(Move::Check(data, index))
	} else if is_mate {
		Some(Move::Checkmate(data, index))
	} else {
		Some(Move::Move(data, index))
	}
}

fn parse_simple_pawn(notation: [char; 2]) -> Option<Move> {
	let index = get_index_from_chars(notation)?;
	Some(Move::Move(
		MoveData {
			x: None,
			y: None,
			piece: Some(PieceType::Pawn),
			special: SpecialMove::empty(),
		},
		index,
	))
}

fn parse_simple_piece(notation: [char; 3]) -> Option<Move> {
	let ptype = PieceType::try_from(notation[0]).ok()?;
	let index = get_index_from_chars([notation[1], notation[2]])?;
	Some(Move::Move(
		MoveData {
			x: None,
			y: None,
			piece: Some(ptype),
			special: SpecialMove::empty(),
		},
		index,
	))
}
