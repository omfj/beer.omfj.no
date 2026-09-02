<script lang="ts">
	import { ChevronRight, KeyRound, Pencil, Trash2 } from '@lucide/svelte';
	import Input from '$lib/components/input.svelte';
	import { formatDate } from '$lib/date';
	import { resolve } from '$app/paths';
	import { invalidateAll } from '$app/navigation';
	import Select from '$lib/components/select.svelte';
	import Button from '$lib/components/button.svelte';
	import { enhance } from '$app/forms';
	import { registerPasskey, genericPasskeyError, PasskeyCancelledError } from '$lib/passkey-client';

	let { data } = $props();

	let isSubmitting = $state(false);

	let user = $derived(data.user);
	let createdEvents = $derived(data.createdEvents);
	let passkeys = $derived(data.passkeys);

	let isAddingPasskey = $state(false);
	let passkeyError = $state<string | null>(null);
	let renamingPasskeyId = $state<string | null>(null);

	async function addPasskey() {
		isAddingPasskey = true;
		passkeyError = null;
		try {
			await registerPasskey();
			await invalidateAll();
		} catch (e) {
			if (!(e instanceof PasskeyCancelledError)) {
				passkeyError = e instanceof Error ? e.message : genericPasskeyError;
			}
		} finally {
			isAddingPasskey = false;
		}
	}

	let weight = $state('');
	let gender = $state('');

	$effect(() => {
		weight = data.user.weight ?? '';
		gender = data.user.gender ?? '';
	});
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

<div class="bg-background-dark mb-8 p-4">
	<h2 class="mb-1 text-lg font-medium">Passkeys</h2>
	<p class="text-foreground-muted mb-4 text-sm">Legg til passkey</p>

	{#if passkeyError}
		<p class="mb-4 text-sm text-red-500">{passkeyError}</p>
	{/if}

	{#if passkeys.length > 0}
		<ul class="mb-4 flex flex-col gap-2">
			{#each passkeys as passkey (passkey.id)}
				<li class="bg-background flex items-center justify-between gap-4 p-3">
					{#if renamingPasskeyId === passkey.id}
						<form
							method="post"
							action="?/renamePasskey"
							class="flex flex-1 items-center gap-2"
							use:enhance={() => {
								return async ({ update }) => {
									renamingPasskeyId = null;
									await update();
								};
							}}
						>
							<input type="hidden" name="id" value={passkey.id} />
							<Input
								name="name"
								value={passkey.name ?? ''}
								placeholder="Navn på passkey"
								maxlength={64}
								class="h-10 min-w-0 flex-1 p-2 text-base"
								autofocus
							/>
							<Button size="sm" type="submit">Lagre</Button>
							<Button
								variant="ghost"
								size="sm"
								type="button"
								onclick={() => (renamingPasskeyId = null)}
							>
								Avbryt
							</Button>
						</form>
					{:else}
						<div class="flex items-center gap-3">
							<KeyRound class="text-foreground-muted h-5 w-5 shrink-0" />
							<div class="flex flex-col">
								<span class="font-medium">{passkey.name || 'Passkey'}</span>
								<span class="text-foreground-muted text-xs">
									Lagt til {formatDate(passkey.createdAt)}
								</span>
							</div>
						</div>
						<div class="flex items-center gap-1">
							<Button
								variant="ghost"
								size="sm"
								type="button"
								aria-label="Gi passkey nytt navn"
								onclick={() => (renamingPasskeyId = passkey.id)}
							>
								<Pencil class="h-5 w-5" />
							</Button>
							<form method="post" action="?/deletePasskey" use:enhance>
								<input type="hidden" name="id" value={passkey.id} />
								<Button variant="ghost" size="sm" type="submit" aria-label="Slett passkey">
									<Trash2 class="h-5 w-5 text-red-500" />
								</Button>
							</form>
						</div>
					{/if}
				</li>
			{/each}
		</ul>
	{/if}

	<Button
		type="button"
		disabled={isAddingPasskey}
		onclick={addPasskey}
		class="disabled:cursor-not-allowed disabled:opacity-75"
	>
		Legg til passkey
	</Button>
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
