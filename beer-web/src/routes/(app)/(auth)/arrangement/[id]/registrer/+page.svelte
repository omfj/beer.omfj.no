<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { ArrowLeft } from '@lucide/svelte';
	import Select from '$lib/components/select.svelte';
	import { calculateDrinkPoints } from '$lib/scoring';
	import { ApiError } from '$lib/api';
	import { api } from '$lib/api/client';
	import { getUser } from '$lib/context/user.svelte';
	import DrinkPhotoInput from '$lib/components/drink-photo-input.svelte';
	import ErrorAlert from '$lib/components/error-alert.svelte';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
	const auth = getUser();
	const eventId = page.params.id!;
	let event = $derived(data.event);
	let drinkOptions = $derived(data.drinkOptions);
	let formError = $state<string | null>(null);
	let files = $state<FileList | null>(null);
	let isUploading = $state(false);
	let selectedDrinkType = $state<string>('');
	let selectedDrinkSize = $state<string>('');
	let abvInput = $state<number | null>(null);
	let fileError = $state<string | null>(null);

	// Computed available sizes based on selected drink type
	let availableSizes = $derived.by(() => {
		if (!selectedDrinkType) return [];
		return drinkOptions.drinkTypeSizes
			.filter((combo) => combo.drinkTypeId === selectedDrinkType)
			.map((combo) => drinkOptions.drinkSizes.find((size) => size.id === combo.drinkSizeId)!)
			.sort((a, b) => {
				if (!a.volumeML || !b.volumeML) return 0;
				return a.volumeML - b.volumeML;
			});
	});

	function sizeIsAvailable(typeId: string, sizeId: string) {
		return drinkOptions.drinkTypeSizes.some(
			(combo) => combo.drinkTypeId === typeId && combo.drinkSizeId === sizeId
		);
	}

	function changeDrinkType(event: globalThis.Event) {
		selectedDrinkType = (event.currentTarget as HTMLSelectElement).value;
		if (!sizeIsAvailable(selectedDrinkType, selectedDrinkSize)) {
			selectedDrinkSize = '';
		}
		abvInput = drinkOptions.drinkTypes.find((type) => type.id === selectedDrinkType)?.abv ?? null;
	}

	function usePreviousDrink() {
		const previous = data.previousDrink;
		if (!previous?.drinkType) return;
		selectedDrinkType = previous.drinkType.id;
		selectedDrinkSize =
			previous.drinkSize && sizeIsAvailable(selectedDrinkType, previous.drinkSize.id)
				? previous.drinkSize.id
				: '';
		abvInput = previous.abv;
	}

	// Calculate preview points based on selected size and entered ABV
	let previewPoints = $derived.by(() => {
		if (!selectedDrinkSize || abvInput == null) {
			return 0.5;
		}
		const selectedSize = availableSizes.find((size) => size?.id === selectedDrinkSize);
		if (!selectedSize) return 0.5;
		const typeData = drinkOptions.drinkTypes.find((t) => t.id === selectedDrinkType);
		return calculateDrinkPoints(selectedSize.volumeML, abvInput, typeData?.multiplier);
	});

	function handleApiError(error: unknown) {
		if (error instanceof ApiError && error.status === 401) {
			auth.clear();
			return;
		}
		if (error instanceof ApiError && error.status === 403) {
			void goto(`/arrangement/${eventId}/unlock`, { replaceState: true });
			return;
		}
		throw error;
	}

	async function submitDrink(event: SubmitEvent) {
		event.preventDefault();
		if (!files?.[0] || isUploading) return;

		isUploading = true;
		formError = null;
		try {
			await api.createDrink(eventId, {
				image: files[0],
				drinkTypeId: selectedDrinkType || null,
				drinkSizeId: selectedDrinkSize || null,
				abv: abvInput
			});
			await goto(`/arrangement/${eventId}`);
		} catch (error) {
			try {
				handleApiError(error);
			} catch {
				formError = error instanceof Error ? error.message : 'Kunne ikke registrere drinken';
			}
		} finally {
			isUploading = false;
		}
	}
</script>

<svelte:head>
	<title>Registrer drink{event ? ` - ${event.name}` : ''} - Beer Counter</title>
</svelte:head>

{#if event}
	<a
		href={`/arrangement/${event.id}`}
		class="my-4 flex items-center gap-4 text-2xl font-light hover:underline"
	>
		<ArrowLeft class="h-6 w-6" /> Tilbake til arrangement
	</a>

	<div class="mb-8">
		<h1 class="mb-3 text-3xl font-medium">Registrer ny drink</h1>
		<p class="text-xl font-light">Velg type, størrelse og last opp et bilde.</p>
	</div>

	{#if formError}
		<ErrorAlert message={formError} class="mb-6" />
	{/if}

	{#if fileError}
		<ErrorAlert message={fileError} class="mb-6" />
	{/if}

	<form enctype="multipart/form-data" onsubmit={submitDrink} class="space-y-6">
		{#if data.previousDrink?.drinkType && drinkOptions.drinkTypes.some((type) => type.id === data.previousDrink?.drinkType?.id)}
			<button
				type="button"
				onclick={usePreviousDrink}
				class="bg-background-dark hover:bg-background-darker w-full p-4 text-lg font-medium transition-colors"
			>
				Fyll inn med forrige drikke
			</button>
		{/if}

		<!-- Drink Type and Size Selection -->
		<div class="space-y-4">
			<div>
				<label for="drinkTypeId" class="mb-2 block text-lg font-medium"
					>Drikketype <span class="text-sm text-gray-500">(valgfritt)</span></label
				>
				<Select
					bind:value={selectedDrinkType}
					onchange={changeDrinkType}
					id="drinkTypeId"
					name="drinkTypeId"
					class="text-foreground"
				>
					<option value="">Ikke oppgitt</option>
					{#each drinkOptions.drinkTypes as drinkType (drinkType.id)}
						<option value={drinkType.id}>{drinkType.name} ×{drinkType.multiplier}</option>
					{/each}
				</Select>
			</div>

			{#if selectedDrinkType}
				<div>
					<label for="drinkSizeId" class="mb-2 block text-lg font-medium"
						>Størrelse <span class="text-sm text-gray-500">(valgfritt)</span></label
					>
					<Select
						bind:value={selectedDrinkSize}
						id="drinkSizeId"
						name="drinkSizeId"
						class="text-foreground"
					>
						<option value="">Ikke oppgitt</option>
						{#each availableSizes as size (size.id)}
							<option value={size.id}>{size.name} ({size.volumeML}ml)</option>
						{/each}
					</Select>
				</div>

				<div>
					<label for="abv" class="mb-2 block text-lg font-medium">
						Alkoholprosent (%) <span class="text-sm text-gray-500">(alkoholinnhold på enheten)</span
						>
					</label>
					<input
						type="number"
						id="abv"
						name="abv"
						min="0"
						max="100"
						step="0.1"
						bind:value={abvInput}
						placeholder="f.eks. 5.2"
						class="bg-background-dark border-background-darkest focus:ring-primary focus:border-primary w-full border p-3 text-lg focus:ring-2 focus:outline-none"
					/>
				</div>
			{/if}

			<div class="bg-primary/10 border-primary/20 border p-4">
				<div class="flex items-center justify-between">
					<span class="text-lg font-medium">Forventet poengsum:</span>
					<span class="text-primary text-2xl font-bold">{previewPoints} poeng</span>
				</div>
				<p class="mt-1 text-sm text-gray-600">
					{#if !selectedDrinkSize || abvInput == null}
						Standard poeng (mangler størrelse eller prosent)
					{:else}
						Basert på størrelse og alkoholprosent
					{/if}
				</p>
			</div>
		</div>

		<DrinkPhotoInput bind:files bind:fileError {isUploading} />
	</form>
{/if}
