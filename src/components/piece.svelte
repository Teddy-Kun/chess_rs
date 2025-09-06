<script lang="ts">
	import { browser } from "$app/environment";
	import { get_board, type Color, type PieceType } from "../lib";

	let {
		piece,
		color,
		onclick,
	}: {
		piece: PieceType;
		color: Color;
		onclick: () => void;
	} = $props();

	let svgContent: string | null = $state(null);

	async function load_svg() {
		if (browser) {
			const module = await import(`../assets/${piece}_${color}.svg?raw`);
			svgContent = module.default;
		}
	}

	$effect(() => {
		load_svg();
	});
</script>

<button {onclick}>
	{#if svgContent}
		{@html svgContent}
	{/if}
</button>
