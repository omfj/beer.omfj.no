<script lang="ts">
	import { enhance } from '$app/forms';
	import Button from '$lib/components/button.svelte';
	import Checkbox from '$lib/components/checkbox.svelte';
	import Input from '$lib/components/input.svelte';

	let { form } = $props();

	let username = $state('');
	let password = $state('');
	let repeatPassword = $state('');

	let isPasswordIsMatching = $derived.by(() => {
		if (!password || !repeatPassword) return true;
		return password === repeatPassword;
	});

	let hasAgreedToTerms = $state(false);

	let isAllowedToSignUp = $derived.by(() => {
		return !isPasswordIsMatching || !hasAgreedToTerms || !username;
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
		<Input bind:value={username} name="username" required />
	</label>

	<label class="flex flex-col gap-1 text-xl font-medium">
		Passord
		<Input bind:value={password} type="password" name="password" required />
	</label>

	<label class="flex flex-col gap-1 text-xl font-medium">
		Gjenta passord
		<Input bind:value={repeatPassword} type="password" name="repeat-password" required />
	</label>

	{#if !isPasswordIsMatching}
		<p class="text-red-500">Passordene må være like.</p>
	{/if}

	<div class="flex flex-col gap-2">
		<label class="flex flex-row items-center gap-4">
			<Checkbox
				bind:checked={hasAgreedToTerms}
				name="terms"
				required
				aria-label="Jeg godtar vilkårene"
				aria-required="true"
			/>

			<span>
				Jeg godtar <a class="text-primary hover:underline" href="/vilkar">vilkårene</a> for å bruke Beer
				Counter.
			</span>
		</label>

		<div>
			<span class="text-sm text-gray-500">
				Du må godta vilkårene for å opprette en konto. Dette er nødvendig for å sikre at alle
				brukere er klar over reglene for deling av innhold.
			</span>
		</div>
	</div>

	<Button disabled={isAllowedToSignUp} class="hover:underline" formaction="?/register"
		>Register ny bruker</Button
	>

	<a class="text-primary text-center hover:underline" href="/logg-inn">
		Har du allerede en bruker? Logg inn her.</a
	>
</form>
