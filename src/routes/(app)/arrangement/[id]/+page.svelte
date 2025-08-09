<script lang="ts">
	import ButtonLink from '$lib/components/button-link.svelte';
	import Button from '$lib/components/button.svelte';
	import { ArrowLeft, CircleAlert, Trophy, Share, QrCode, Trash2 } from '@lucide/svelte';
	import { fly } from 'svelte/transition';
	import { enhance } from '$app/forms';
	import SEO from '$lib/components/seo.svelte';
	import { calculateDrinkPoints } from '$lib/scoring';

	let { data } = $props();

	let event = $derived(data.event);
	let attendees = $derived(data.attendees);
	let user = $derived(data.user);

	// Create scoreboard: calculate points per user
	let scoreboard = $derived.by(() => {
		const userStats = new Map<
			string,
			{
				user: {
					id: string;
					username: string;
				};
				points: number;
				count: number;
			}
		>();

		attendees.toReversed().forEach((attendee) => {
			const userId = attendee.userId;

			// Calculate points for this drink
			const points = calculateDrinkPoints(
				attendee.drinkSize?.volumeML || null,
				attendee.drinkType?.abv || null
			);

			if (userStats.has(userId)) {
				userStats.get(userId)!.points += points;
				userStats.get(userId)!.count++;
			} else {
				userStats.set(userId, {
					user: {
						id: attendee.userId,
						username: attendee.username
					},
					points: points,
					count: 1
				});
			}
		});

		// Convert to array and sort by points (highest first)
		return Array.from(userStats.values())
			.map((entry) => ({
				...entry,
				points: Math.round(entry.points * 10) / 10 // Round to 1 decimal place
			}))
			.sort((a, b) => b.points - a.points);
	});

	// Pagination state
	let scoreboardLimit = $state(10);
	let imagesLimit = $state(10);

	const loadMoreScoreboard = () => {
		scoreboardLimit += 10;
	};

	const loadMoreImages = () => {
		imagesLimit += 10;
	};

	const shareEvent = async () => {
		const url = window.location.href;
		const title = `${event.name} - Beer Counter`;
		const text = `Bli med på arrangementet "${event.name}" på Beer Counter!`;

		if (navigator.share) {
			try {
				await navigator.share({
					title,
					text,
					url
				});
			} catch {
				// User cancelled sharing
			}
		} else {
			// Fallback: copy to clipboard
			try {
				await navigator.clipboard.writeText(url);
				alert('Lenke kopiert til utklippstavlen!');
			} catch {
				alert(`Del denne lenken: ${url}`);
			}
		}
	};

	const handleDelete = () => {
		const confirmed = confirm(
			'Er du sikker på at du vil slette denne registreringen? Dette kan ikke angres.'
		);
		return confirmed;
	};
</script>

<SEO
	title={event.name}
	description={`Bli med på arrangementet "${event.name}" og se hvem som har høyest poengsum! ${attendees.length} drinker registrert totalt.`}
	type="article"
/>

<a href="/" class="my-4 flex items-center gap-4 text-2xl font-light hover:underline">
	<ArrowLeft class="h-6 w-6" /> Tilbake hjem
</a>

<div class="mb-4">
	<h1 class="mb-3 text-3xl font-medium">{event.name}</h1>

	<p class="mb-4 text-xl font-light">Registrer en drink for å bli med på arrangementet!</p>

	{#if attendees.length > 0}
		<p class="text-lg text-gray-600">{attendees.length} drinker registrert totalt</p>
	{/if}
</div>

<div class="my-4 flex flex-col gap-4">
	<div>
		<ButtonLink class="w-full" href="/arrangement/{data.event.id}/registrer"
			>Registrer ny drink</ButtonLink
		>
	</div>

	<div class="flex flex-col gap-4 sm:flex-row">
		<ButtonLink variant="outline" class="w-full gap-2" href="/arrangement/{data.event.id}/qr">
			<QrCode class="size-5" />
			QR-kode
		</ButtonLink>

		<Button variant="outline" class="w-full gap-2" onclick={shareEvent}>
			<Share class="size-5" />
			Del arrangement
		</Button>
	</div>
</div>

<div class="bg-background-darkest my-4 h-0.5"></div>

{#if scoreboard.length > 0}
	<!-- Scoreboard Section -->
	<div class="mb-8 space-y-4">
		<div class="flex items-center gap-3">
			<Trophy class="text-primary h-6 w-6" />
			<h2 class="text-2xl font-medium">Toppliste</h2>
		</div>

		<div class="space-y-3">
			{#each scoreboard.slice(0, scoreboardLimit) as { user: scoreUser, points, count }, index (scoreUser.id)}
				<div
					class="flex items-center justify-between rounded p-3 transition-colors {index === 0
						? 'bg-primary/10 border-primary/20 border'
						: index === 1
							? 'border border-amber-500/20 bg-amber-500/10'
							: index === 2
								? 'border border-orange-600/20 bg-orange-600/10'
								: 'bg-background-darker'}"
					transition:fly={{ duration: 200, delay: index * 50 }}
				>
					<div class="flex items-center gap-4">
						<div
							class="flex h-8 w-8 items-center justify-center rounded-full text-sm font-bold {index ===
							0
								? 'bg-primary text-white'
								: index === 1
									? 'bg-amber-500 text-white'
									: index === 2
										? 'bg-orange-600 text-white'
										: 'bg-background-darkest text-gray-600'}"
						>
							{#if index < 3}
								{index === 0 ? '🥇' : index === 1 ? '🥈' : '🥉'}
							{:else}
								{index + 1}
							{/if}
						</div>
						<div>
							<p class="font-medium">
								{scoreUser.username} <span class="text-sm text-gray-500">({count} stk)</span>
							</p>
							{#if scoreUser.id === user.id}
								<p class="text-primary text-xs">Det er deg!</p>
							{/if}
						</div>
					</div>

					<div class="flex items-center gap-2">
						<span
							class="text-2xl font-bold {index === 0
								? 'text-primary'
								: index === 1
									? 'text-amber-500'
									: index === 2
										? 'text-orange-600'
										: 'text-gray-600'}">{points}</span
						>
						<span class="text-sm text-gray-500">poeng</span>
					</div>
				</div>
			{/each}

			{#if scoreboard.length > scoreboardLimit}
				<button
					onclick={loadMoreScoreboard}
					class="hover:bg-background-darker w-full rounded p-3 text-center text-sm text-gray-600 transition-colors"
				>
					Vis 10 flere deltakere ({scoreboard.length - scoreboardLimit} gjenstår)
				</button>
			{/if}
		</div>
	</div>

	<div class="bg-background-darkest my-4 h-0.5"></div>
{/if}

{#if attendees.length > 0}
	<div class="space-y-4">
		<h2 class="text-2xl font-medium">Registrerte drinker</h2>

		<div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
			{#each attendees.slice(0, imagesLimit) as attendee (attendee.id)}
				<div class="bg-background-dark overflow-hidden border-2">
					{#if attendee.imageId}
						<div class="aspect-square">
							<a
								href="/api/image/{attendee.imageId}"
								target="_blank"
								rel="noopener noreferrer"
								class="block h-full w-full transition-transform hover:scale-105"
								aria-label="Se fullstørrelse bilde fra {attendee.username}"
							>
								<img
									src="/api/image/{attendee.imageId}"
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
					<div class="p-4">
						<div class="flex items-center justify-between gap-2">
							<p class="truncate font-medium">{attendee.username}</p>
							<div class="flex items-center gap-2">
								<p class="flex-shrink-0 text-xs whitespace-nowrap text-gray-600">
									{attendee.createdAt.toLocaleDateString('no-NO', {
										day: 'numeric',
										month: 'short',
										hour: '2-digit',
										minute: '2-digit'
									})}
								</p>
								{#if attendee.userId === user.id}
									<form
										method="POST"
										action="?/delete"
										use:enhance={() => {
											return async ({ update }) => {
												if (handleDelete()) {
													await update();
												}
											};
										}}
									>
										<input type="hidden" name="attendeeId" value={attendee.id} />
										<button
											type="submit"
											class="rounded p-1 text-red-600 transition-colors hover:bg-red-50 hover:text-red-700"
											title="Slett registrering"
										>
											<Trash2 class="h-4 w-4" />
										</button>
									</form>
								{/if}
							</div>
						</div>
					</div>
				</div>
			{/each}
		</div>

		{#if attendees.length > imagesLimit}
			<div class="mt-6 text-center">
				<button
					onclick={loadMoreImages}
					class="bg-background-dark hover:bg-background-darker rounded px-6 py-3 text-sm transition-colors"
				>
					Vis 10 flere bilder ({attendees.length - imagesLimit} gjenstår)
				</button>
			</div>
		{/if}
	</div>
{:else}
	<div class="bg-background-dark flex h-32 items-center justify-center rounded-lg p-4">
		<p class="text-lg text-gray-600">Ingen drinker registrert ennå</p>
	</div>
{/if}
