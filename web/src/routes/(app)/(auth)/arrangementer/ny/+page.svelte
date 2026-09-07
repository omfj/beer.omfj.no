<script lang="ts">
	import { goto } from '$app/navigation';
	import { ArrowLeft } from '@lucide/svelte';
	import { resolve } from '$app/paths';
	import { ApiError } from '$lib/api';
	import { api } from '$lib/api/client';
	import { getUser } from '$lib/context/user.svelte';

	const auth = getUser();
	let name = $state('');
	let password = $state('');
	let isLoading = $state(false);
	let errorMessage = $state<string | null>(null);

	async function createEvent(event: SubmitEvent) {
		event.preventDefault();
		isLoading = true;
		errorMessage = null;

		try {
			const { event: createdEvent } = await api.createEvent({
				name,
				password: password || null
			});
			await goto(`/arrangement/${encodeURIComponent(createdEvent.id)}`);
		} catch (error) {
			if (error instanceof ApiError && error.status === 401) {
				auth.clear();
				return;
			}
			errorMessage =
				error instanceof ApiError ? error.message : 'Kunne ikke opprette arrangementet.';
		} finally {
			isLoading = false;
		}
	}
</script>

<svelte:head>
	<title>Nytt arrangement - Beer Counter</title>
</svelte:head>

<a href={resolve('/')} class="my-4 flex items-center gap-4 text-2xl font-light hover:underline">
	<ArrowLeft class="h-6 w-6" /> Tilbake hjem
</a>

<h1 class="mb-8 text-5xl">Lag nytt arrangement</h1>

{#if errorMessage}
	<p class="text-red-500">{errorMessage}</p>
{/if}

<form class="flex flex-col gap-4" onsubmit={createEvent}>
	<label class="flex flex-col text-xl font-medium">
		Navn
		<input class="bg-background-dark h-14 p-4 text-3xl" bind:value={name} name="name" required />
	</label>

	<label class="flex flex-col text-xl font-medium">
		<span class="flex items-center gap-2">
			Passord <span class="text-base font-normal text-gray-500">(valgfritt)</span>
		</span>
		<input
			class="bg-background-dark h-14 p-4 text-3xl"
			bind:value={password}
			name="password"
			type="password"
		/>
		<p class="text-foreground-muted mt-2 text-sm">
			Legg til et passord for å begrense hvem som kan se og delta i arrangementet. Hvis du ikke
			legger til et passord, vil arrangementet være åpent for alle med linken.
		</p>
	</label>

	<button
		class="bg-background-darker hover:bg-background-darkest h-14 text-xl transition-colors hover:underline"
		disabled={isLoading}>{isLoading ? 'Lager arrangement...' : 'Lag arrangement'}</button
	>
</form>
