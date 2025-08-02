<script lang="ts">
	import { enhance } from '$app/forms';
	import { DoorOpen, TriangleAlert } from '@lucide/svelte';
	import Button from '$lib/components/button.svelte';

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

{#if data.showTermsPopup}
	<div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm">
		<div class="bg-background mx-4 max-w-lg rounded-lg p-6 shadow-2xl">
			<div class="mb-6 flex items-center gap-3">
				<TriangleAlert class="h-8 w-8 text-orange-500" />
				<h2 class="text-2xl font-medium">Godta vilkår</h2>
			</div>

			<div class="mb-6 space-y-4">
				<p class="text-gray-700">
					Du må godta våre vilkår for å fortsette å bruke Beer Counter. Vilkårene beskriver hvordan
					vi håndterer opplastede filer og deling av innhold. <a
						class="text-primary hover:underline"
						href="/vilkar">Les vilkårene her</a
					>.
				</p>

				<p class="text-sm text-gray-700">
					Om du ikke godtar vilkårene, kan du ikke bruke Beer Counter.
				</p>
			</div>

			<div class="flex flex-col gap-3 sm:flex-row">
				<form method="post" action="?/acceptTerms" use:enhance class="flex-1">
					<Button class="w-full bg-green-600 text-white hover:bg-green-700">Godta vilkår</Button>
				</form>

				<form method="post" action="?/deleteAccount" use:enhance>
					<Button class="w-full bg-red-600 px-4 text-white hover:bg-red-700">Slett konto</Button>
				</form>
			</div>
		</div>
	</div>
{/if}
