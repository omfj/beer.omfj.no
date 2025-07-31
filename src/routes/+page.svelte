<script lang="ts">
	import { enhance } from '$app/forms';
	import { DoorOpen } from '@lucide/svelte';

	let { data } = $props();
	let events = $derived(data.events);
</script>

<svelte:head>
	<title>Beer Counter</title>
</svelte:head>

<div class="mb-8">
	<h1 class="mb-3 text-3xl font-medium">Arrangementer</h1>

	<p class="mb-4 text-xl font-light">
		Her finner du en oversikt over arrangementene våre. Trykk på et arrangement for å se mer
		informasjon og melde deg på.
	</p>

	<div class="flex items-center justify-between">
		<a
			class="bg-background-darker hover:bg-background-darkest flex h-14 w-fit items-center justify-center p-4 text-lg font-medium transition-colors"
			href="/arrangementer/ny"
		>
			Lag nytt arrangement
		</a>

		<form action="/logg-ut" method="post" use:enhance>
			<button class="flex items-center gap-3 text-2xl font-light hover:underline">
				<DoorOpen class="h-6 w-6" />
				Logg ut
			</button>
		</form>
	</div>
</div>

<ul class="flex flex-col gap-5">
	{#each events as event}
		<li>
			<a
				class="bg-background-dark hover:bg-background-darker flex h-20 items-center justify-center p-4 text-3xl transition-colors hover:underline"
				href={`/arrangement/${event.id}`}>{event.name}</a
			>
		</li>
	{:else}
		<li>
			<p class="bg-background-dark flex items-center justify-center h-20 p-4 transition-colors">
				Ingen arrangementer tilgjengelig
			</p>
		</li>
	{/each}
</ul>
