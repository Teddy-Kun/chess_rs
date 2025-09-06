// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use std::sync::{LazyLock, RwLock};

use chess_rs_lib::{bitboard::BitBoard, board::Board, piece::ChessCell};

static BOARD_STATE: LazyLock<RwLock<Board>> = LazyLock::new(|| RwLock::new(Board::new()));

#[tauri::command]
fn greet(name: &str) -> String {
	format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_board() -> Box<[ChessCell]> {
	BOARD_STATE.read().unwrap().get_pieces()
}

#[tauri::command]
fn get_legal_moves(index: u8) -> BitBoard {
	// TODO
	BitBoard::new()
}

#[tauri::command]
fn move_piece(index: u8, target: u8) -> Box<[ChessCell]> {
	// TODO
	let board = BOARD_STATE.write().unwrap();
	// board.force_move_piece(index, target);
	board.get_pieces()
}

#[tauri::command]
fn restart() -> Box<[ChessCell]> {
	let mut board = BOARD_STATE.write().unwrap();
	board.reset();
	board.get_pieces()
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
