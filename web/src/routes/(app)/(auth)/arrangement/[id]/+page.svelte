<script lang="ts">
	import ButtonLink from '$lib/components/button-link.svelte';
	import Button from '$lib/components/button.svelte';
	import Checkbox from '$lib/components/checkbox.svelte';
	import { resolve } from '$app/paths';
	import { ArrowLeft, CircleAlert, Trophy, Share, QrCode, Trash2 } from '@lucide/svelte';
	import { fly } from 'svelte/transition';
	import { enhance } from '$app/forms';
	import SEO from '$lib/components/seo.svelte';
	import { calculateDrinkPoints, calculateBac, alcoholGrams, weightToKg } from '$lib/scoring';
	import { SvelteMap, SvelteSet } from 'svelte/reactivity';

	let { data } = $props();

	let event = $derived(data.event);
	let attendees = $derived(data.attendees);
	let accessUsers = $derived(data.accessUsers);
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
				drinks: { grams: number; consumedAtMs: number }[];
				weight: string | null;
				gender: string | null;
			}
		>();

		// Seed all users with access at 0 so they appear even without drinks
		for (const u of accessUsers) {
			userStats.set(u.id, {
				user: { id: u.id, username: u.username },
				points: 0,
				count: 0,
				drinks: [],
				weight: u.weight,
				gender: u.gender
			});
		}

		attendees.toReversed().forEach((attendee) => {
			const userId = attendee.userId;

			const points = calculateDrinkPoints(
				attendee.drinkSize?.volumeML || null,
				attendee.abv,
				attendee.drinkType?.multiplier
			);

			const drink = {
				grams: alcoholGrams(attendee.drinkSize?.volumeML, attendee.abv),
				consumedAtMs: attendee.createdAt.getTime()
			};

			if (userStats.has(userId)) {
				const entry = userStats.get(userId)!;
				entry.points += points;
				entry.count++;
				entry.drinks.push(drink);
			} else {
				userStats.set(userId, {
					user: { id: attendee.userId, username: attendee.username },
					points,
					count: 1,
					drinks: [drink],
					weight: attendee.userWeight,
					gender: attendee.userGender
				});
			}
		});

		// Convert to array and sort by points (highest first)
		return Array.from(userStats.values())
			.map((entry) => {
				let promille: number | null = null;
				if (
					entry.weight &&
					entry.gender &&
					(entry.weight === 'light' || entry.weight === 'medium' || entry.weight === 'heavy') &&
					(entry.gender === 'male' || entry.gender === 'female' || entry.gender === 'other')
				) {
					// Per-drink elimination against current time so old drinks decay away
					promille = calculateBac(entry.drinks, weightToKg(entry.weight), entry.gender, Date.now());
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
		const priorDrinks = new SvelteMap<string, { grams: number; consumedAtMs: number }[]>();

		// Process chronologically so accumulation is correct
		const sorted = [...attendees].sort((a, b) => a.createdAt.getTime() - b.createdAt.getTime());

		for (const attendee of sorted) {
			const drinks = priorDrinks.get(attendee.userId) ?? [];

			// Show promille BEFORE this drink (exclude current drink from calculation)
			const w = attendee.userWeight;
			const g = attendee.userGender;
			if (
				drinks.length > 0 &&
				(w === 'light' || w === 'medium' || w === 'heavy') &&
				(g === 'male' || g === 'female' || g === 'other')
			) {
				// Evaluate at this drink's own timestamp so each prior drink decays from its own time
				result.set(
					attendee.id,
					calculateBac(drinks, weightToKg(w), g, attendee.createdAt.getTime())
				);
			} else {
				result.set(attendee.id, null);
			}

			drinks.push({
				grams: alcoholGrams(attendee.drinkSize?.volumeML, attendee.abv),
				consumedAtMs: attendee.createdAt.getTime()
			});
			priorDrinks.set(attendee.userId, drinks);
		}

		return result;
	});

	// User filter state
	let selectedUserIds = $state(new Set<string>());
	let filterOpen = $state(false);

	const toggleUser = (id: string) => {
		const next = new SvelteSet(selectedUserIds);
		if (next.has(id)) next.delete(id);
		else next.add(id);
		selectedUserIds = next;
	};

	let filterLabel = $derived(
		selectedUserIds.size === 0
			? 'Alle deltakere'
			: selectedUserIds.size === 1
				? (scoreboard.find((e) => selectedUserIds.has(e.user.id))?.user.username ?? '1 valgt')
				: `${selectedUserIds.size} valgt`
	);

	let filteredAttendees = $derived(
		selectedUserIds.size === 0 ? attendees : attendees.filter((a) => selectedUserIds.has(a.userId))
	);

	// Pagination state
	let scoreboardLimit = $state(10);
	let imagesLimit = $state(12);

	const loadMoreScoreboard = () => {
		scoreboardLimit += 10;
	};

	const loadMoreImages = () => {
		imagesLimit += 12;
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
			href={resolve('/(app)/(auth)/arrangement/[id]/registrer', {
				id: data.event.id
			})}>Registrer ny drink</ButtonLink
		>
	</div>

	<div class="flex flex-col gap-4 sm:flex-row">
		<ButtonLink
			variant="outline"
			class="w-full gap-2"
			href={resolve('/(app)/(auth)/arrangement/[id]/qr', {
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

{#if scoreboard.length > 1}
	<div class="relative mb-4">
		<p class="text-foreground-muted mb-1 text-sm">Filtrer:</p>
		<button
			onclick={() => (filterOpen = !filterOpen)}
			class="bg-background-dark border-background-darkest focus:ring-primary focus:border-primary flex h-12 w-full items-center justify-between border px-4 text-lg text-gray-600 focus:ring-2 focus:outline-none"
		>
			<span>{filterLabel}</span>
			<svg
				class="h-5 w-5 text-gray-400 transition-transform {filterOpen ? 'rotate-180' : ''}"
				fill="none"
				viewBox="0 0 24 24"
				stroke="currentColor"
				stroke-width="2"
			>
				<path stroke-linecap="round" stroke-linejoin="round" d="M19 9l-7 7-7-7" />
			</svg>
		</button>

		{#if filterOpen}
			<div
				class="bg-background-dark border-background-darkest absolute z-10 mt-1 w-full border shadow-lg"
			>
				{#each scoreboard as { user: u } (u.id)}
					<div class="hover:bg-background-darker px-4 py-3">
						<Checkbox
							checked={selectedUserIds.has(u.id)}
							onchange={() => toggleUser(u.id)}
							label={u.username}
						/>
					</div>
				{/each}
				{#if selectedUserIds.size > 0}
					<div class="border-background-darkest border-t px-4 py-3">
						<Button variant="ghost" size="sm" onclick={() => (selectedUserIds = new Set())}>
							Nullstill filter
						</Button>
					</div>
				{/if}
			</div>
		{/if}
	</div>
{/if}

{#if filteredAttendees.length > 0}
	<div class="space-y-4">
		<div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
			{#each filteredAttendees.slice(0, imagesLimit) as attendee (attendee.id)}
				<div class="bg-background-dark overflow-hidden border-2">
					{#if attendee.imageId}
						<div class="aspect-square">
							<a
								href={resolve('/(app)/api/image/[id]', {
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
										attendee.abv,
										attendee.drinkType?.multiplier
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

		{#if filteredAttendees.length > imagesLimit}
			<div class="mt-6 text-center">
				<button
					onclick={loadMoreImages}
					class="bg-background-dark hover:bg-background-darker rounded px-6 py-3 text-sm transition-colors"
				>
					Vis {Math.min(12, filteredAttendees.length - imagesLimit)} flere bilder
				</button>
			</div>
		{/if}
	</div>
{:else if attendees.length > 0}
	<div class="bg-background-dark flex h-32 items-center justify-center rounded-lg p-4">
		<p class="text-foreground-muted text-lg">Ingen drinker å vise for valgte brukere</p>
	</div>
{:else}
	<div class="bg-background-dark flex h-32 items-center justify-center rounded-lg p-4">
		<p class="text-foreground-muted text-lg">Ingen drinker registrert ennå</p>
	</div>
{/if}
