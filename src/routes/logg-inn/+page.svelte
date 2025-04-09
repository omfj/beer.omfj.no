<script lang="ts">
	import { enhance } from '$app/forms';

	let { form } = $props();
</script>

<svelte:head>
	<title>Logg inn - Beer Counter</title>
</svelte:head>

<h1 class="mb-8 text-5xl">Logg inn / <br />Registrer deg</h1>

<form class="flex flex-col gap-4" method="post" action="?/login" use:enhance>
	<label class="flex flex-col text-xl font-medium">
		Brukernavn
		<input class="bg-background-dark h-14 p-4 text-3xl" name="username" required />
	</label>

	<label class="flex flex-col text-xl font-medium">
		Passord
		<input class="bg-background-dark h-14 p-4 text-3xl" type="password" name="password" required />
	</label>

	<button
		class="bg-background-darker hover:bg-background-darkest h-14 text-xl transition-colors hover:underline"
		>Logg inn</button
	>

	<button
		class="bg-background-darker hover:bg-background-darkest h-14 text-xl transition-colors hover:underline"
		formaction="?/register">Register ny bruker</button
	>
</form>

<p style="color: red">{form?.message ?? ''}</p>
