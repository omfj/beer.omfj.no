<script lang="ts">
	import ButtonLink from '$lib/components/button-link.svelte';
	import Button from '$lib/components/button.svelte';
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import { ArrowLeft, Share, QrCode } from '@lucide/svelte';
	import SEO from '$lib/components/seo.svelte';
	import { ApiError } from '$lib/api';
	import { api } from '$lib/api/client';
	import { getUser } from '$lib/context/user.svelte';
	import { calculateDrinkPoints, calculateBac, alcoholGrams, weightToKg } from '$lib/scoring';
	import { SvelteMap, SvelteSet } from 'svelte/reactivity';
	import EventScoreboard from '$lib/components/event-scoreboard.svelte';
	import EventUserFilter from '$lib/components/event-user-filter.svelte';
	import DrinkImageCard from '$lib/components/drink-image-card.svelte';
	import ErrorAlert from '$lib/components/error-alert.svelte';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
	const auth = getUser();
	const eventId = page.params.id!;
	let eventDetail = $derived(data.eventDetail);
	let deleteError = $state<string | null>(null);
	let deletingIds = $state(new Set<string>());
	let deletedIds = $state(new Set<string>());

	let event = $derived(eventDetail?.event ?? null);
	let attendees = $derived(
		eventDetail.attendees.filter((attendee) => !deletedIds.has(attendee.id))
	);
	let accessUsers = $derived(eventDetail?.accessUsers ?? []);
	let user = $derived(auth.state.status === 'authenticated' ? auth.state.user : null);

	function handleApiError(error: unknown) {
		if (error instanceof ApiError && error.status === 401) {
			auth.clear();
			return true;
		}
		if (error instanceof ApiError && error.status === 403) {
			void goto(
				resolve('/(app)/(auth)/arrangement/[id]/unlock', {
					id: eventId
				}),
				{ replaceState: true }
			);
			return true;
		}
		return false;
	}

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
				consumedAtMs: attendee.createdAt * 1000
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
		const sorted = [...attendees].sort((a, b) => a.createdAt - b.createdAt);

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
				result.set(attendee.id, calculateBac(drinks, weightToKg(w), g, attendee.createdAt * 1000));
			} else {
				result.set(attendee.id, null);
			}

			drinks.push({
				grams: alcoholGrams(attendee.drinkSize?.volumeML, attendee.abv),
				consumedAtMs: attendee.createdAt * 1000
			});
			priorDrinks.set(attendee.userId, drinks);
		}

		return result;
	});

	// User filter state
	let selectedUserIds = $state(new Set<string>());

	let filteredAttendees = $derived(
		selectedUserIds.size === 0 ? attendees : attendees.filter((a) => selectedUserIds.has(a.userId))
	);

	// Pagination state
	let imagesLimit = $state(12);

	const loadMoreImages = () => {
		imagesLimit += 12;
	};

	const shareEvent = async () => {
		const currentEvent = event;
		if (!currentEvent) return;
		const url = window.location.href;
		const title = `${currentEvent.name} - Beer Counter`;
		const text = `Bli med på arrangementet "${currentEvent.name}" på Beer Counter!`;

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

	const handleDelete = async (attendeeId: string) => {
		if (!confirm('Er du sikker på at du vil slette denne registreringen? Dette kan ikke angres.'))
			return;

		deleteError = null;
		deletingIds = new SvelteSet(deletingIds).add(attendeeId);
		try {
			await api.deleteDrink(eventId, attendeeId);
			deletedIds = new SvelteSet(deletedIds).add(attendeeId);
		} catch (error) {
			if (!handleApiError(error)) {
				deleteError = error instanceof Error ? error.message : 'Kunne ikke slette registreringen';
			}
		} finally {
			const next = new SvelteSet(deletingIds);
			next.delete(attendeeId);
			deletingIds = next;
		}
	};
</script>

{#if event}
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
					id: event.id
				})}>Registrer ny drink</ButtonLink
			>
		</div>

		<div class="flex flex-col gap-4 sm:flex-row">
			<ButtonLink
				variant="outline"
				class="w-full gap-2"
				href={resolve('/(app)/(auth)/arrangement/[id]/qr', {
					id: event.id
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

	{#if deleteError}
		<ErrorAlert message={deleteError} class="mb-4" />
	{/if}

	<EventScoreboard entries={scoreboard} currentUserId={user?.id} />

	<EventUserFilter entries={scoreboard} bind:selectedUserIds />

	{#if filteredAttendees.length > 0}
		<div class="space-y-4">
			<div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
				{#each filteredAttendees.slice(0, imagesLimit) as attendee (attendee.id)}
					<DrinkImageCard
						{attendee}
						currentUserId={user?.id}
						promille={attendeePromille.get(attendee.id)}
						deleting={deletingIds.has(attendee.id)}
						onDelete={handleDelete}
					/>
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
		<div class="bg-background-dark flex h-32 items-center justify-center p-4">
			<p class="text-foreground-muted text-lg">Ingen drinker å vise for valgte brukere</p>
		</div>
	{:else}
		<div class="bg-background-dark flex h-32 items-center justify-center p-4">
			<p class="text-foreground-muted text-lg">Ingen drinker registrert ennå</p>
		</div>
	{/if}
{/if}
