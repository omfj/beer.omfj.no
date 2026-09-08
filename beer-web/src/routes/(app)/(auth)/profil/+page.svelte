<script lang="ts">
	import { ApiError, type UpdateProfileRequest } from '$lib/api';
	import { api } from '$lib/api/client';
	import Button from '$lib/components/button.svelte';
	import Select from '$lib/components/select.svelte';
	import { getUser } from '$lib/context/user.svelte';

	const auth = getUser();

	let isSubmitting = $state(false);
	let saveError = $state<string | null>(null);
	let saved = $state(false);
	let weight = $state('');
	let gender = $state('');
	let initializedUserId = $state<string | null>(null);

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
{/if}
