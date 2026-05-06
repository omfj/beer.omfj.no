<script lang="ts">
	import { enhance } from '$app/forms';
	import { page } from '$app/state';
	import Button from '$lib/components/button.svelte';
	import Input from '$lib/components/input.svelte';
	import SEO from '$lib/components/seo.svelte';

	let { form } = $props();

	let event = page.url.searchParams.get('event');
	let registerUrl = $derived(event ? `/registrer?event=${event}` : '/registrer');
</script>

<SEO
	title="Logg inn"
	description="Logg inn på Beer Counter for å begynne å telle øl på arrangementer med venner."
/>

<h1 class="mb-8 text-5xl">Logg inn</h1>

{#if form?.message}
	<p class="text-red-500">{form.message}</p>
{/if}

<form class="flex flex-col gap-6" method="post" action="?/login" use:enhance>
	<input type="hidden" name="eventId" value={event} />

	<label class="flex flex-col gap-1 text-xl font-medium">
		Brukernavn
		<Input name="username" required />
	</label>

	<label class="flex flex-col gap-1 text-xl font-medium">
		Passord
		<Input type="password" name="password" required />
	</label>

	<Button>Logg inn</Button>
	<!-- eslint-disable-next-line svelte/no-navigation-without-resolve -->
	<a class="text-primary text-center hover:underline" href={registerUrl}>
		Har du ikke bruker? Registrer deg her.</a
	>
</form>
