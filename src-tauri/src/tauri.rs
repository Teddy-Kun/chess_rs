// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use std::sync::{LazyLock, RwLock};

use chess_rs_lib::{board::Board, board_hash::BoardHash, piece::Piece};

static BOARD_STATE: LazyLock<RwLock<Board>> = LazyLock::new(|| RwLock::new(Board::new()));

#[tauri::command]
fn greet(name: &str) -> String {
	format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_board() -> Vec<Piece> {
	BOARD_STATE.read().unwrap().get_pieces()
}

#[tauri::command]
fn get_legal_moves(index: u8) -> BoardHash {
	match BOARD_STATE.read().unwrap().get_at_index(index) {
		None => BoardHash::new(),
		Some(piece) => piece.get_legal_moves(),
	}
}

#[tauri::command]
fn move_piece(index: u8, target: u8) -> Vec<Piece> {
	let mut board = BOARD_STATE.write().unwrap();
	board.force_move_piece(index, target);
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
			move_piece
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
