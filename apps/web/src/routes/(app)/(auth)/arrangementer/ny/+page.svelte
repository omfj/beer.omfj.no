<script lang="ts">
	import { enhance } from '$app/forms';
	import { ArrowLeft } from '@lucide/svelte';
	import { resolve } from '$app/paths';

	let { form } = $props();
</script>

<svelte:head>
	<title>Nytt arrangement - Beer Counter</title>
</svelte:head>

<a href={resolve('/')} class="my-4 flex items-center gap-4 text-2xl font-light hover:underline">
	<ArrowLeft class="h-6 w-6" /> Tilbake hjem
</a>

<h1 class="mb-8 text-5xl">Lag nytt arrangement</h1>

<p style="color: red">{form?.message ?? ''}</p>

<form class="flex flex-col gap-4" method="post" action="?/create" use:enhance>
	<label class="flex flex-col text-xl font-medium">
		Navn
		<input class="bg-background-dark h-14 p-4 text-3xl" name="name" required />
	</label>

	<label class="flex flex-col text-xl font-medium">
		<span class="flex items-center gap-2">
			Passord <span class="text-base font-normal text-gray-500">(valgfritt)</span>
		</span>
		<input class="bg-background-dark h-14 p-4 text-3xl" name="password" type="password" />
		<p class="text-foreground-muted mt-2 text-sm">
			Legg til et passord for å begrense hvem som kan se og delta i arrangementet. Hvis du ikke
			legger til et passord, vil arrangementet være åpent for alle med linken.
		</p>
	</label>

	<button
		class="bg-background-darker hover:bg-background-darkest h-14 text-xl transition-colors hover:underline"
		>Lag arrangement</button
	>
</form>
