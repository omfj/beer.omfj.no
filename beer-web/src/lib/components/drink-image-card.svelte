<script lang="ts">
	import { CircleAlert, Trash2 } from '@lucide/svelte';
	import { api } from '$lib/api/client';
	import { calculateDrinkPoints } from '$lib/scoring';
	import type { Attendee } from '$lib/api/types';

	let {
		attendee,
		currentUserId,
		promille,
		deleting,
		onDelete
	}: {
		attendee: Attendee;
		currentUserId: string | undefined;
		promille: number | null | undefined;
		deleting: boolean;
		onDelete: (id: string) => void;
	} = $props();
</script>

<div class="bg-background-dark overflow-hidden border">
	{#if attendee.imageId}
		<div class="aspect-square">
			<a
				href={api.imageUrl(attendee.imageId)}
				target="_blank"
				rel="noopener noreferrer"
				class="block h-full w-full transition-transform hover:scale-105"
				aria-label="Se fullstørrelse bilde fra {attendee.username}"
			>
				<img
					src={api.imageUrl(attendee.imageId)}
					alt="Drink fra {attendee.username}"
					class="h-full w-full cursor-pointer object-cover"
					loading="lazy"
				/>
			</a>
		</div>
	{:else}
		<div class="flex aspect-square flex-col items-center justify-center bg-gray-200">
			<CircleAlert class="h-8 w-8 text-gray-500" />

			<p class="p-4 text-center text-sm text-balance text-gray-500">
				Bruker har ikke godkjent vilkår, og vi kan ikke vise bildet.
			</p>
		</div>
	{/if}

	<!-- Beer info -->
	<div class="p-3">
		<div class="flex items-start justify-between gap-2">
			<div class="min-w-0">
				<p class="truncate font-medium">{attendee.username}</p>
				<p class="text-xs text-gray-500">
					{new Date(attendee.createdAt * 1000).toLocaleDateString('no-NO', {
						day: 'numeric',
						month: 'short',
						hour: '2-digit',
						minute: '2-digit'
					})}
				</p>
			</div>
			{#if attendee.userId === currentUserId}
				<button
					type="button"
					onclick={() => onDelete(attendee.id)}
					disabled={deleting}
					class="hover:bg-background-darkest shrink-0 rounded p-1 text-red-600 transition-colors disabled:cursor-wait disabled:opacity-50"
					title="Slett registrering"
				>
					<Trash2 class="h-4 w-4" />
				</button>
			{/if}
		</div>

		<div class="bg-background-darkest mt-3 h-px"></div>

		<div class="mt-2 flex items-baseline justify-between">
			<div class="flex items-baseline gap-1">
				<span class="text-primary text-xl font-bold"
					>{calculateDrinkPoints(
						attendee.drinkSize?.volumeML || null,
						attendee.abv,
						attendee.drinkType?.multiplier
					)}</span
				>
				<span class="text-xs text-gray-500">poeng</span>
			</div>
			{#if promille !== null}
				<div class="text-right">
					<span class="text-sm font-medium text-gray-600">{promille?.toFixed(2)} ‰</span>
					<p class="text-xs text-gray-400">før denne</p>
				</div>
			{/if}
		</div>
	</div>
</div>
