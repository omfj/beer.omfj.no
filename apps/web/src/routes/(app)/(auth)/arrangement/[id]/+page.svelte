<script lang="ts">
	import ButtonLink from '$lib/components/button-link.svelte';
	import Button from '$lib/components/button.svelte';
	import { resolve } from '$app/paths';
	import { ArrowLeft, CircleAlert, Trophy, Share, QrCode, Trash2 } from '@lucide/svelte';
	import { fly } from 'svelte/transition';
	import { enhance } from '$app/forms';
	import SEO from '$lib/components/seo.svelte';
	import { calculateDrinkPoints, calculateBac, weightToKg } from '$lib/scoring';
	import { SvelteMap } from 'svelte/reactivity';

	let { data } = $props();

	let event = $derived(data.event);
	let attendees = $derived(data.attendees);
	let user = $derived(data.user);

	// Create scoreboard: calculate points and promille per user
	let scoreboard = $derived.by(() => {
		const userStats = new SvelteMap<
			string,
			{
				user: {
					id: string;
					username: string;
				};
				points: number;
				count: number;
				totalAlcoholGrams: number;
				firstDrinkAt: Date;
				lastDrinkAt: Date;
				weight: string | null;
				gender: string | null;
			}
		>();

		attendees.toReversed().forEach((attendee) => {
			const userId = attendee.userId;

			const points = calculateDrinkPoints(
				attendee.drinkSize?.volumeML || null,
				attendee.drinkType?.abv || null
			);

			const volumeL = (attendee.drinkSize?.volumeML ?? 0) / 1000;
			const abvDecimal = (attendee.drinkType?.abv ?? 0) / 100;
			const alcoholGrams = volumeL * abvDecimal * 0.789 * 1000;

			if (userStats.has(userId)) {
				const entry = userStats.get(userId)!;
				entry.points += points;
				entry.count++;
				entry.totalAlcoholGrams += alcoholGrams;
				if (attendee.createdAt < entry.firstDrinkAt) {
					entry.firstDrinkAt = attendee.createdAt;
				}
				if (attendee.createdAt > entry.lastDrinkAt) {
					entry.lastDrinkAt = attendee.createdAt;
				}
			} else {
				userStats.set(userId, {
					user: { id: attendee.userId, username: attendee.username },
					points,
					count: 1,
					totalAlcoholGrams: alcoholGrams,
					firstDrinkAt: attendee.createdAt,
					lastDrinkAt: attendee.createdAt,
					weight: attendee.userWeight,
					gender: attendee.userGender
				});
			}
		});

		// Convert to array and sort by points (highest first)
		return Array.from(userStats.values())
			.map((entry) => {
				// Use current time so promille reflects ongoing elimination
				const hoursElapsed = (Date.now() - entry.firstDrinkAt.getTime()) / 3_600_000;
				let promille: number | null = null;
				if (
					entry.weight &&
					entry.gender &&
					(entry.weight === 'light' || entry.weight === 'medium' || entry.weight === 'heavy') &&
					(entry.gender === 'male' || entry.gender === 'female' || entry.gender === 'other')
				) {
					promille = Math.max(
						0,
						calculateBac(
							entry.totalAlcoholGrams,
							weightToKg(entry.weight),
							hoursElapsed,
							entry.gender
						)
					);
				}
				return {
					user: entry.user,
					points: Math.round(entry.points * 10) / 10,
					count: entry.count,
					promille
				};
			})
			.sort((a, b) => b.points - a.points);
	});

	// Pre-compute cumulative promille at the moment of each drink registration
	let attendeePromille = $derived.by(() => {
		const result = new SvelteMap<string, number | null>();
		const userAccum = new SvelteMap<string, { totalAlcohol: number; firstDrinkAt: Date }>();

		// Process chronologically so accumulation is correct
		const sorted = [...attendees].sort((a, b) => a.createdAt.getTime() - b.createdAt.getTime());

		for (const attendee of sorted) {
			const volumeL = (attendee.drinkSize?.volumeML ?? 0) / 1000;
			const abvDecimal = (attendee.drinkType?.abv ?? 0) / 100;
			const alcoholGrams = volumeL * abvDecimal * 0.789 * 1000;

			const prev = userAccum.get(attendee.userId);
			const prevTotal = prev?.totalAlcohol ?? 0;
			const firstDrinkAt = prev?.firstDrinkAt ?? attendee.createdAt;

			// Show promille BEFORE this drink (exclude current drink from calculation)
			const w = attendee.userWeight;
			const g = attendee.userGender;
			if (
				prevTotal > 0 &&
				(w === 'light' || w === 'medium' || w === 'heavy') &&
				(g === 'male' || g === 'female' || g === 'other')
			) {
				const hours = (attendee.createdAt.getTime() - firstDrinkAt.getTime()) / 3_600_000;
				result.set(attendee.id, Math.max(0, calculateBac(prevTotal, weightToKg(w), hours, g)));
			} else {
				result.set(attendee.id, null);
			}

			userAccum.set(attendee.userId, { totalAlcohol: prevTotal + alcoholGrams, firstDrinkAt });
		}

		return result;
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

<a href={resolve('/')} class="my-4 flex items-center gap-4 text-2xl font-light hover:underline">
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
		<ButtonLink
			class="w-full"
			href={resolve('/arrangement/[id]/registrer', {
				id: data.event.id
			})}>Registrer ny drink</ButtonLink
		>
	</div>

	<div class="flex flex-col gap-4 sm:flex-row">
		<ButtonLink
			variant="outline"
			class="w-full gap-2"
			href={resolve('/arrangement/[id]/qr', {
				id: data.event.id
			})}
		>
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
		<p class="text-foreground-muted text-sm">
			Promille er et estimat beregnet akkurat nå, basert på registrerte drinker siden første
			registrering. Krever at vekt og kjønn er satt i
			<a href="/profil" class="underline">profilen din</a>.
		</p>

		<div class="space-y-3">
			{#each scoreboard.slice(0, scoreboardLimit) as { user: scoreUser, points, count, promille }, index (scoreUser.id)}
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
							{#if scoreUser.id === user?.id}
								<p class="text-primary text-xs">Det er deg!</p>
							{/if}
						</div>
					</div>

					<div class="flex flex-col items-end gap-0.5">
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
						{#if promille !== null && count >= 2}
							<span class="text-xs text-gray-500">~{promille.toFixed(2)} ‰</span>
						{/if}
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
								href={resolve('/api/image/[id]', {
									id: attendee.imageId
								})}
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
					<div class="p-3">
						<div class="flex items-start justify-between gap-2">
							<div class="min-w-0">
								<p class="truncate font-medium">{attendee.username}</p>
								<p class="text-xs text-gray-500">
									{attendee.createdAt.toLocaleDateString('no-NO', {
										day: 'numeric',
										month: 'short',
										hour: '2-digit',
										minute: '2-digit'
									})}
								</p>
							</div>
							{#if attendee.userId === user?.id}
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
										class="shrink-0 rounded p-1 text-red-600 transition-colors hover:bg-red-50 hover:text-red-700"
										title="Slett registrering"
									>
										<Trash2 class="h-4 w-4" />
									</button>
								</form>
							{/if}
						</div>

						<div class="bg-background-darkest mt-3 h-px"></div>

						<div class="mt-2 flex items-baseline justify-between">
							<div class="flex items-baseline gap-1">
								<span class="text-primary text-xl font-bold"
									>{calculateDrinkPoints(
										attendee.drinkSize?.volumeML || null,
										attendee.drinkType?.abv || null
									)}</span
								>
								<span class="text-xs text-gray-500">poeng</span>
							</div>
							{#if attendeePromille.get(attendee.id) !== null}
								<div class="text-right">
									<span class="text-sm font-medium text-gray-600"
										>{attendeePromille.get(attendee.id)?.toFixed(2)} ‰</span
									>
									<p class="text-xs text-gray-400">før denne</p>
								</div>
							{/if}
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
		<p class="text-foreground-muted text-lg">Ingen drinker registrert ennå</p>
	</div>
{/if}
