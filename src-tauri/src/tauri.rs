// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use std::sync::{LazyLock, RwLock};

use chess_rs_lib::{bitboard::BitBoard, board::Board};

static BOARD_STATE: LazyLock<RwLock<Board>> = LazyLock::new(|| {
	#[cfg(debug_assertions)]
	{
		use chess_rs_lib::piece::{ChessCell, Color, Moved, PieceType};

		let mut b = Board::empty();
		b.add_piece(
			ChessCell::with_piece(PieceType::Rook, Color::Black, Moved::No),
			36,
		);
		b.add_piece(
			ChessCell::with_piece(PieceType::Bishop, Color::Black, Moved::No),
			35,
		);
		b.add_piece(
			ChessCell::with_piece(PieceType::Bishop, Color::White, Moved::No),
			0,
		);
		b.add_piece(
			ChessCell::with_piece(PieceType::Bishop, Color::White, Moved::No),
			7,
		);

		b.add_piece(
			ChessCell::with_piece(PieceType::Bishop, Color::Black, Moved::No),
			63,
		);

		b.add_piece(
			ChessCell::with_piece(PieceType::Bishop, Color::Black, Moved::No),
			63 - 7,
		);
		RwLock::new(b)
	}
	#[cfg(not(debug_assertions))]
	{
		RwLock::new(Board::new())
	}
});

#[tauri::command]
fn greet(name: &str) -> String {
	format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_board() -> Board {
	BOARD_STATE.read().unwrap().clone()
}

#[tauri::command]
fn get_legal_moves(index: u8) -> BitBoard {
	BOARD_STATE.read().unwrap().get_legal_moves(index)
}

#[tauri::command]
fn move_piece(index: u8, target: u8) -> Board {
	let mut board = BOARD_STATE.write().unwrap();
	board.move_piece_unchecked(index, target);
	board.clone()
}

#[tauri::command]
fn restart() -> Board {
	let mut board = BOARD_STATE.write().unwrap();
	board.reset();
	board.clone()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	tauri::Builder::default()
		.plugin(tauri_plugin_opener::init())
		.invoke_handler(tauri::generate_handler![
			greet,
			get_board,
			get_legal_moves,
			move_piece,
			restart
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
