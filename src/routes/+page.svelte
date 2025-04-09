<script lang="ts">
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

	<a
		class="bg-background-darker hover:bg-background-darkest flex h-14 w-fit items-center justify-center p-4 text-lg font-medium transition-colors"
		href="/arrangementer/ny"
	>
		Lag nytt arrangement
	</a>
</div>

<ul>
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
