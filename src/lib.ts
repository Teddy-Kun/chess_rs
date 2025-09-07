import { invoke } from "@tauri-apps/api/core";

export type PieceType = "pawn" | "knight" | "bishop" | "rook" | "queen" | "king";
export type Color = "black" | "white";

interface BackendPiece {
	type: string
	color: string
	has_moved: boolean
}

export interface Piece {
	type: PieceType
	color: Color
	has_moved: boolean
}

export type BackendState = Record<number, BackendPiece>;
export type BoardState = Record<number, Piece>;

function convert_piece(bp: BackendPiece): Piece {
	return {
		type: bp.type.toLowerCase() as PieceType,
		color: bp.color.toLowerCase() as Color,
		has_moved: bp.has_moved,
	};
}

function convert_board_state(arr: BackendState): BoardState {
	const board_state: BoardState = {};

	console.log("backend", arr);

	for (const key in arr) {
		const bp = arr[key];
		const p = convert_piece(bp);
		board_state[Number.parseInt(key)] = p;
	}

	return board_state;
}

export async function get_board(): Promise<BoardState> {
	const res = await invoke("get_board") as BackendState;
	return convert_board_state(res);
}

export async function get_legal_moves(index: number): Promise<number[]> {
	return await invoke("get_legal_moves", { index }) as number[];
}

export async function move_piece(index: number, target: number): Promise<BoardState> {
	const res = await invoke("move_piece", { index, target }) as BackendState;
	return convert_board_state(res);
}

export async function restart(): Promise<BoardState> {
	const res = await invoke("restart") as BackendState;
	return convert_board_state(res);
}
