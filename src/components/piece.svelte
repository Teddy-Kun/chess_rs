<script lang="ts">
	import { browser } from "$app/environment";
	import { onMount } from "svelte";
	import { type Color, type PieceType } from "../lib";

	let {
		piece,
		color,
		size,
		onclick,
	}: {
		piece: PieceType;
		color: Color;
		size: number;
		onclick: () => void;
	} = $props();

	let svgContent: string | null = $state(null);
	let scaleFactor = $derived(size / 45);

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

<button
	{onclick}
	style:transform={`scale(${scaleFactor})`}
	style:transformOrigin="center center"
>
	{#if svgContent}
		{@html svgContent}
	{/if}
</button>
