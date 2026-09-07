<script lang="ts">
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import { ApiError } from '$lib/api';
	import { api } from '$lib/api/client';
	import Button from '$lib/components/button.svelte';
	import Input from '$lib/components/input.svelte';
	import SEO from '$lib/components/seo.svelte';
	import { getUser } from '$lib/context/user.svelte';

	let event = page.url.searchParams.get('event');
	let registerUrl = $derived(
		event ? `/registrer?event=${encodeURIComponent(event)}` : '/registrer'
	);
	const auth = getUser();
	let username = $state('');
	let password = $state('');
	let isLoading = $state(false);
	let errorMessage = $state<string | null>(null);

	async function login(event_: SubmitEvent) {
		event_.preventDefault();
		isLoading = true;
		errorMessage = null;

		try {
			const { user } = await api.login({ username, password });
			auth.setUser(user);
			await goto(event ? `/arrangement/${encodeURIComponent(event)}` : '/');
		} catch (error) {
			errorMessage =
				error instanceof ApiError ? error.message : 'Kunne ikke logge inn. Prøv igjen.';
		} finally {
			isLoading = false;
		}
	}
</script>

<SEO
	title="Logg inn"
	description="Logg inn på Beer Counter for å begynne å telle øl på arrangementer med venner."
/>

<h1 class="mb-8 text-5xl">Logg inn</h1>

{#if errorMessage}
	<p class="text-red-500">{errorMessage}</p>
{/if}

<form class="flex flex-col gap-6" onsubmit={login}>
	<label class="flex flex-col gap-1 text-xl font-medium">
		Brukernavn
		<Input bind:value={username} name="username" autocomplete="username" required />
	</label>

	<label class="flex flex-col gap-1 text-xl font-medium">
		Passord
		<Input
			bind:value={password}
			type="password"
			name="password"
			autocomplete="current-password"
			required
		/>
	</label>

	<Button disabled={isLoading}>{isLoading ? 'Logger inn...' : 'Logg inn'}</Button>
	<!-- eslint-disable-next-line svelte/no-navigation-without-resolve -->
	<a class="text-primary text-center hover:underline" href={registerUrl}>
		Har du ikke bruker? Registrer deg her.</a
	>
</form>
