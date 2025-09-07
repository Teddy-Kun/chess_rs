<script lang="ts">
	import { onMount } from "svelte";
	import {
		get_board,
		get_legal_moves,
		move_piece,
		restart,
		type BoardState,
	} from "../lib";
	import Piece from "./piece.svelte";

	let board = $state<BoardState>({});
	let moves = $state<{ index: number; moves: number[] } | null>(null);
	let cell_size = $state(64);

	function get_class(i: number): string {
		// alternating colors
		let adjusted = Math.floor(i / 8) % 2 === 1 ? i + 1 : i;

		let cls: string;

		if (moves?.moves.includes(i)) {
			cls = adjusted % 2 === 1 ? "bg-green-800" : "bg-green-600";
		} else cls = adjusted % 2 === 1 ? "bg-[#6d3600]" : "bg-[#ffcf9f]";
		cls += " flex items-center justify-center";
		if (moves?.index === i) cls += " border-2 border-red-500";
		return cls;
	}

	const text_cls = "flex items-center justify-center my-2";

	function get_moves(index: number): void {
		if (moves?.index === index) moves = null;
		else {
			get_legal_moves(index).then((res) => {
				moves = {
					index,
					moves: res,
				};
			});
		}
	}

	function click_empty(index: number) {
		if (!moves) return;

		if (moves.moves.includes(index)) {
			move_piece(moves.index, index).then((res) => {
				board = res;
				moves = null;
			});
		} else moves = null;
	}

	function reset(): void {
		restart().then((res) => {
			board = res;
		});
	}

	function calc_cell_size(): void {
		const h = window.innerHeight;
		const w = window.innerWidth;

		const smaller = h < w ? h : w;

		cell_size = Math.floor(smaller / 10);
		console.log("h", h, "w", w, "smaller", smaller, "cell_size", cell_size);
	}

	onMount(() => {
		get_board().then((state) => {
			board = state;
		});
		calc_cell_size();
		addEventListener("resize", () => calc_cell_size());
	});
</script>

<button
	class="absolute top-4 left-4 text-white py-2 px-4 border border-border rounded-md hover:bg-gray-800 transition-colors cursor-pointer"
	onclick={reset}
>
	Reset
</button>
<div class="m-8 grid grid-cols-10 font-bold text-xl text-white">
	<p></p>
	<p class={text_cls} style:width={`${cell_size}px`}>A</p>
	<p class={text_cls} style:width={`${cell_size}px`}>B</p>
	<p class={text_cls} style:width={`${cell_size}px`}>C</p>
	<p class={text_cls} style:width={`${cell_size}px`}>D</p>
	<p class={text_cls} style:width={`${cell_size}px`}>E</p>
	<p class={text_cls} style:width={`${cell_size}px`}>F</p>
	<p class={text_cls} style:width={`${cell_size}px`}>G</p>
	<p class={text_cls} style:width={`${cell_size}px`}>H</p>
	<p></p>
	{#each { length: 64 }, i}
		{#if i % 8 === 0}
			<span
				class="flex justify-end items-center mr-4"
				style:height={`${cell_size}px`}
			>
				{(i / 8 - 8) * -1}
			</span>
		{/if}
		<span
			class={get_class(i)}
			style:width={`${cell_size}px`}
			style:height={`${cell_size}px`}
		>
			{#if board[i]}
				<Piece
					piece={board[i].type}
					color={board[i].color}
					size={cell_size}
					onclick={() => get_moves(i)}
				/>
			{:else}
				<button class="size-full" onclick={() => click_empty(i)}>
					<p class="sr-only">empty</p>
				</button>
			{/if}
		</span>
		{#if i % 8 === 7}
			<span
				class="flex justify-start items-center ml-4"
				style:height={`${cell_size}px`}
			>
				{((i - 7) / 8 - 8) * -1}
			</span>
		{/if}
	{/each}
	<p></p>
	<p class={text_cls}>A</p>
	<p class={text_cls}>B</p>
	<p class={text_cls}>C</p>
	<p class={text_cls}>D</p>
	<p class={text_cls}>E</p>
	<p class={text_cls}>F</p>
	<p class={text_cls}>G</p>
	<p class={text_cls}>H</p>
	<p></p>
</div>
