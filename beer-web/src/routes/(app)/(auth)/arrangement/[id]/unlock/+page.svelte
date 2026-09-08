<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { ArrowLeft, Lock } from '@lucide/svelte';
	import { resolve } from '$app/paths';
	import { ApiError } from '$lib/api';
	import { api } from '$lib/api/client';
	import { getUser } from '$lib/context/user.svelte';

	const auth = getUser();
	const eventId = page.params.id!;
	let password = $state('');
	let message = $state<string | null>(null);
	let isSubmitting = $state(false);

	function clearAuth(error: unknown) {
		if (error instanceof ApiError && error.status === 401) {
			auth.clear();
			return true;
		}
		return false;
	}

	async function unlock(event: SubmitEvent) {
		event.preventDefault();
		if (isSubmitting) return;
		isSubmitting = true;
		message = null;
		try {
			await api.unlockEvent(eventId, { password });
			await goto(`/arrangement/${eventId}`, { replaceState: true });
		} catch (error) {
			if (!clearAuth(error)) {
				message = error instanceof Error ? error.message : 'Kunne ikke låse opp arrangementet';
			}
		} finally {
			isSubmitting = false;
		}
	}
</script>

<svelte:head>
	<title>Passordbeskyttet arrangement - Beer Counter</title>
</svelte:head>

<a href={resolve('/')} class="my-4 flex items-center gap-4 text-2xl font-light hover:underline">
	<ArrowLeft class="h-6 w-6" /> Tilbake hjem
</a>

<div class="flex flex-col items-center justify-center py-12">
	<div class="w-full">
		<div class="mb-8 flex flex-col items-center gap-3 text-center">
			<Lock class="h-12 w-12 text-gray-400" />
			<h1 class="text-3xl font-medium">Passordbeskyttet arrangement</h1>
			<p class="text-gray-500">Dette arrangementet er passordbeskyttet</p>
		</div>

		{#if message}
			<p class="mb-4 text-red-500">{message}</p>
		{/if}

		<form class="flex flex-col gap-4" onsubmit={unlock}>
			<label class="flex flex-col text-xl font-medium">
				Passord
				<input
					class="bg-background-dark h-14 p-4 text-2xl"
					name="password"
					type="password"
					bind:value={password}
					required
				/>
			</label>

			<button
				disabled={isSubmitting}
				class="bg-background-darker hover:bg-background-darkest h-14 text-xl transition-colors hover:underline"
			>
				{isSubmitting ? 'Låser opp...' : 'Gå inn i arrangement'}
			</button>
		</form>
	</div>
</div>
