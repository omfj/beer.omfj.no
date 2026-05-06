<script lang="ts">
	import { enhance } from '$app/forms';
	import { ArrowLeft, Lock } from '@lucide/svelte';
	import { resolve } from '$app/paths';

	let { data, form } = $props();
</script>

<svelte:head>
	<title>{data.eventName} - Passord kreves</title>
</svelte:head>

<a href={resolve('/')} class="my-4 flex items-center gap-4 text-2xl font-light hover:underline">
	<ArrowLeft class="h-6 w-6" /> Tilbake hjem
</a>

<div class="flex flex-col items-center justify-center py-12">
	<div class="w-full">
		<div class="mb-8 flex flex-col items-center gap-3 text-center">
			<Lock class="h-12 w-12 text-gray-400" />
			<h1 class="text-3xl font-medium">{data.eventName}</h1>
			<p class="text-gray-500">Dette arrangementet er passordbeskyttet</p>
		</div>

		{#if form?.message}
			<p class="mb-4 text-red-500">{form.message}</p>
		{/if}

		<form class="flex flex-col gap-4" method="post" action="?/unlock" use:enhance>
			<label class="flex flex-col text-xl font-medium">
				Passord
				<input
					class="bg-background-dark h-14 p-4 text-2xl"
					name="password"
					type="password"
					required
				/>
			</label>

			<button
				class="bg-background-darker hover:bg-background-darkest h-14 text-xl transition-colors hover:underline"
			>
				Gå inn i arrangement
			</button>
		</form>
	</div>
</div>
