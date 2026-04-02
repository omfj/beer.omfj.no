<script lang="ts">
	import { ChevronRight } from '@lucide/svelte';
	import { formatDate } from '$lib/date';
	import { resolve } from '$app/paths';
	import Select from '$lib/components/select.svelte';
	import Button from '$lib/components/button.svelte';
	import { enhance } from '$app/forms';

	let { data } = $props();

	let isSubmitting = $state(false);

	let user = $derived(data.user);
	let createdEvents = $derived(data.createdEvents);

	let weight = $state(data.user.weight ?? '');
	let gender = $state(data.user.gender ?? '');
</script>

<svelte:head>
	<title>Profil - Beer Counter</title>
</svelte:head>

<h1 class="mb-6 text-3xl font-medium">Din profil</h1>

<div class="mb-4 space-y-2">
	<div>
		<span class="text-foreground-muted text-sm">Brukernavn</span>
		<p class="text-xl font-medium">{user.username}</p>
	</div>

	{#if user.createdAt}
		<div>
			<span class="text-foreground-muted text-sm">Medlem siden</span>
			<p class="text-lg">{formatDate(user.createdAt)}</p>
		</div>
	{/if}
</div>

<div class="bg-background-dark mb-8 p-4">
	<h2 class="mb-1 text-lg font-medium">Promilleinnstillinger</h2>
	<p class="text-foreground-muted mb-4 text-sm">Brukes til å estimere promille i arrangementer.</p>

	<form
		method="post"
		action="?/saveProfile"
		use:enhance={() => {
			isSubmitting = true;
			return async ({ update }) => {
				isSubmitting = false;
				await update({ reset: false });
			};
		}}
	>
		<div class="space-y-4">
			<div class="flex flex-col gap-1">
				<label for="weight" class="text-foreground-muted text-sm">Vektklasse</label>
				<Select bind:value={weight} class="text-foreground" name="weight">
					<option value="">Ikke oppgitt</option>
					<option value="light">Lett (40–60 kg)</option>
					<option value="medium">Middels (61–80 kg)</option>
					<option value="heavy">Tung (81+ kg)</option>
				</Select>
			</div>

			<div class="flex flex-col gap-1">
				<label for="gender" class="text-foreground-muted text-sm">Kjønn</label>
				<Select bind:value={gender} class="text-foreground" name="gender">
					<option value="">Ikke oppgitt</option>
					<option value="male">Mann</option>
					<option value="female">Dame</option>
					<option value="other">Annet</option>
				</Select>
			</div>

			<Button disabled={isSubmitting} class="disabled:cursor-not-allowed disabled:opacity-75"
				>Lagre</Button
			>
		</div>
	</form>
</div>

<section>
	<h2 class="mb-2 text-xl font-medium">Arrangementer du har laget ({createdEvents.length})</h2>

	<ul class="flex flex-col gap-5">
		{#each createdEvents as event (event.id)}
			<li>
				<a
					class="bg-background-dark hover:bg-background-darker group flex h-24 items-center justify-between p-4 transition-colors"
					href={resolve('/arrangement/[id]', { id: event.id })}
				>
					<div class="flex items-center gap-4">
						<div
							class="h-4 w-4 shrink-0 rounded-full border"
							style="background-color: {event.color}"
						></div>
						<div class="flex flex-col gap-2">
							<span class="text-2xl group-hover:underline">{event.name}</span>
							<ul class="text-foreground-muted flex items-center gap-4 text-xs">
								<li>{event.distinctUsers} deltakere</li>
								<li>{event.totalAttendees} enheter registrert</li>
								<li>Opprettet {formatDate(event.createdAt)}</li>
							</ul>
						</div>
					</div>
					<div>
						<ChevronRight
							class="text-foreground-muted group-hover:text-primary h-6 w-6 transition-colors"
						/>
					</div>
				</a>
			</li>
		{:else}
			<li>
				<p class="bg-background-dark flex items-center justify-center h-20 p-4 transition-colors">
					Du har enda ikke laget noen arrangementer
				</p>
			</li>
		{/each}
	</ul>
</section>
