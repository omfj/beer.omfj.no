<script lang="ts">
	import { goto } from '$app/navigation';
	import { ApiError, type UpdateProfileRequest } from '$lib/api';
	import { api } from '$lib/api/client';
	import Button from '$lib/components/button.svelte';
	import Input from '$lib/components/input.svelte';
	import Select from '$lib/components/select.svelte';
	import { getUser } from '$lib/context/user.svelte';

	const auth = getUser();

	let isSubmitting = $state(false);
	let saveError = $state<string | null>(null);
	let saved = $state(false);
	let weight = $state('');
	let gender = $state('');
	let initializedUserId = $state<string | null>(null);

	let deleteDialog = $state<HTMLDialogElement>();
	let deleteConfirmation = $state('');
	let isDeleting = $state(false);
	let deleteError = $state<string | null>(null);

	let user = $derived(auth.state.status === 'authenticated' ? auth.state.user : null);

	$effect(() => {
		if (user && user.id !== initializedUserId) {
			weight = user.weight ?? '';
			gender = user.gender ?? '';
			initializedUserId = user.id;
		}
	});

	async function saveProfile(event: SubmitEvent) {
		event.preventDefault();
		isSubmitting = true;
		saveError = null;
		saved = false;

		try {
			const result = await api.updateMe({
				weight: (weight || null) as UpdateProfileRequest['weight'],
				gender: (gender || null) as UpdateProfileRequest['gender']
			});
			auth.setUser(result.user);
			saved = true;
		} catch (error) {
			if (error instanceof ApiError && error.status === 401) {
				auth.clear();
				return;
			}
			saveError = error instanceof Error ? error.message : 'Kunne ikke lagre profilen.';
		} finally {
			isSubmitting = false;
		}
	}

	function openDeleteDialog() {
		deleteConfirmation = '';
		deleteError = null;
		deleteDialog?.showModal();
	}

	async function deleteAccount(event: SubmitEvent) {
		event.preventDefault();
		if (!user || deleteConfirmation !== user.username) return;
		isDeleting = true;
		deleteError = null;

		try {
			await api.deleteMe();
			deleteDialog?.close();
			auth.clear();
			await goto('/logg-inn', { replaceState: true });
		} catch (error) {
			if (error instanceof ApiError && error.status === 401) {
				auth.clear();
				return;
			}
			deleteError = error instanceof Error ? error.message : 'Kunne ikke slette kontoen.';
		} finally {
			isDeleting = false;
		}
	}
</script>

<svelte:head>
	<title>Profil - Beer Counter</title>
</svelte:head>

<h1 class="mb-6 text-3xl font-medium">Din profil</h1>

{#if user}
	<div class="mb-4 space-y-2">
		<div>
			<span class="text-foreground-muted text-sm">Brukernavn</span>
			<p class="text-xl font-medium">{user.username}</p>
		</div>
	</div>

	<div class="bg-background-dark mb-8 p-4">
		<h2 class="mb-1 text-lg font-medium">Promilleinnstillinger</h2>
		<p class="text-foreground-muted mb-4 text-sm">
			Brukes til å estimere promille i arrangementer.
		</p>

		<form onsubmit={saveProfile}>
			<div class="space-y-4">
				<div class="flex flex-col gap-1">
					<label for="weight" class="text-foreground-muted text-sm">Vektklasse</label>
					<Select id="weight" bind:value={weight} class="text-foreground" name="weight">
						<option value="">Ikke oppgitt</option>
						<option value="light">Lett (40–60 kg)</option>
						<option value="medium">Middels (61–80 kg)</option>
						<option value="heavy">Tung (81+ kg)</option>
					</Select>
				</div>

				<div class="flex flex-col gap-1">
					<label for="gender" class="text-foreground-muted text-sm">Kjønn</label>
					<Select id="gender" bind:value={gender} class="text-foreground" name="gender">
						<option value="">Ikke oppgitt</option>
						<option value="male">Mann</option>
						<option value="female">Dame</option>
						<option value="other">Annet</option>
					</Select>
				</div>

				{#if saveError}
					<p class="text-sm text-red-500" role="alert">{saveError}</p>
				{:else if saved}
					<p class="text-sm text-green-500" role="status">Profilen er lagret.</p>
				{/if}

				<Button disabled={isSubmitting} class="disabled:cursor-not-allowed disabled:opacity-75">
					{isSubmitting ? 'Lagrer…' : 'Lagre'}
				</Button>
			</div>
		</form>
	</div>

	<div class="border-danger border-2 p-4">
		<h2 class="mb-1 text-lg font-medium">Slett konto</h2>
		<p class="text-foreground-muted mb-4 text-sm">
			Kontoen din og alle registreringene dine slettes permanent. Arrangementer du har opprettet
			blir liggende. Dette kan ikke angres.
		</p>
		<Button variant="destructive" type="button" onclick={openDeleteDialog}>Slett konto</Button>
	</div>

	<dialog
		bind:this={deleteDialog}
		aria-labelledby="delete-account-title"
		class="bg-background text-foreground m-auto w-[calc(100%-2rem)] max-w-md border-2 p-6 backdrop:bg-black/60"
	>
		<form onsubmit={deleteAccount} class="space-y-4">
			<h2 id="delete-account-title" class="text-xl font-medium">Er du helt sikker?</h2>
			<p class="text-foreground-muted text-sm">
				Dette sletter kontoen <strong class="text-foreground">{user.username}</strong> og alle registreringene
				dine for godt.
			</p>
			<div class="flex flex-col gap-1">
				<label for="delete-confirmation" class="text-foreground-muted text-sm">
					Skriv brukernavnet ditt for å bekrefte
				</label>
				<Input
					id="delete-confirmation"
					bind:value={deleteConfirmation}
					autocomplete="off"
					autocapitalize="off"
					spellcheck="false"
					class="text-xl"
				/>
			</div>

			{#if deleteError}
				<p class="text-sm text-red-500" role="alert">{deleteError}</p>
			{/if}

			<div class="flex flex-col-reverse gap-2 sm:flex-row sm:justify-end">
				<Button type="button" variant="outline" onclick={() => deleteDialog?.close()}>Avbryt</Button
				>
				<Button variant="destructive" disabled={isDeleting || deleteConfirmation !== user.username}>
					{isDeleting ? 'Sletter…' : 'Slett kontoen min'}
				</Button>
			</div>
		</form>
	</dialog>
{/if}
