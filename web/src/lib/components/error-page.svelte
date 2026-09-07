<script lang="ts">
	import { page } from '$app/state';
	import { resolve } from '$app/paths';
	import ButtonLink from '$lib/components/button-link.svelte';

	let { contained = false }: { contained?: boolean } = $props();
	const notFound = $derived(page.status === 404);
</script>

<svelte:head>
	<title>{notFound ? 'Ikke funnet' : 'Noe gikk galt'} - Beer Counter</title>
</svelte:head>

<main
	class={contained
		? 'flex min-h-[60vh] w-full items-center justify-center py-16'
		: 'mx-auto flex min-h-[70vh] w-full max-w-2xl items-center justify-center px-8 py-16'}
>
	<div class="w-full text-center">
		<div class="relative mx-auto mb-4 flex size-28 items-center justify-center">
			<span
				class="text-background-darkest absolute text-7xl font-bold select-none"
				aria-hidden="true"
			>
				{page.status}
			</span>
		</div>

		<h1 class="mb-3 text-3xl font-medium">
			{notFound ? 'Finner ikke siden' : 'Noe gikk galt'}
		</h1>
		<p class="text-foreground-muted mx-auto mb-8 max-w-md text-lg font-light">
			{notFound
				? 'Arrangementet eller siden du leter etter finnes ikke.'
				: 'Vi klarte ikke å laste siden akkurat nå. Prøv igjen om litt.'}
		</p>

		<ButtonLink href={resolve('/')} size="sm" class="mx-auto w-fit">Til arrangementene</ButtonLink>
	</div>
</main>
