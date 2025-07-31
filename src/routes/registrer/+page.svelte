<script lang="ts">
	import { enhance } from '$app/forms';

	let { form } = $props();

	let password = $state('');
	let repeatPassword = $state('');

	let passwordIsMatching = $derived.by(() => {
		if (!password || !repeatPassword) return true;
		return password === repeatPassword;
	});
</script>

<svelte:head>
	<title>Registrer deg - Beer Counter</title>
</svelte:head>

<h1 class="mb-8 text-5xl">Registrer deg</h1>

{#if form?.message}
	<p class="text-red-500">{form.message}</p>
{/if}

<form class="flex flex-col gap-6" method="post" action="?/login" use:enhance>
	<label class="flex flex-col gap-1 text-xl font-medium">
		Brukernavn
		<input class="bg-background-dark h-14 p-4 text-3xl" name="username" required />
	</label>

	<label class="flex flex-col gap-1 text-xl font-medium">
		Passord
		<input
			bind:value={password}
			class="bg-background-dark h-14 p-4 text-3xl"
			type="password"
			name="password"
			required
		/>
	</label>

	<label class="flex flex-col gap-1 text-xl font-medium">
		Gjenta passord
		<input
			bind:value={repeatPassword}
			class="bg-background-dark h-14 p-4 text-3xl"
			type="password"
			name="repeat-password"
			required
		/>
	</label>

	{#if !passwordIsMatching}
		<p class="text-red-500">Passordene må være like.</p>
	{/if}

	<button
		class="bg-background-darker hover:bg-background-darkest h-14 text-xl transition-colors hover:underline"
		formaction="?/register">Register ny bruker</button
	>

	<a class="text-primary text-center hover:underline" href="/logg-inn">
		Har du allerede en bruker? Logg inn her.</a
	>
</form>
