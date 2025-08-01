<script lang="ts">
	import { ArrowLeft, Trophy } from '@lucide/svelte';
	import { fly } from 'svelte/transition';

	let { data } = $props();

	let event = $derived(data.event);
	let user = $derived(data.user);

	// Filter attendees who have registered beers (have imageId)
	let beersRegistered = $derived(event.attendees.filter((attendee) => attendee.imageId));

	// Create scoreboard: count beers per user
	let scoreboard = $derived.by(() => {
		const userCounts = new Map<string, { user: any; count: number }>();

		beersRegistered.toReversed().forEach((attendee) => {
			const userId = attendee.user.id;
			if (userCounts.has(userId)) {
				userCounts.get(userId)!.count++;
			} else {
				userCounts.set(userId, {
					user: attendee.user,
					count: 1
				});
			}
		});

		// Convert to array and sort by count (highest first)
		return Array.from(userCounts.values()).sort((a, b) => b.count - a.count);
	});
</script>

<svelte:head>
	<title>{event.name} - Beer Counter</title>
</svelte:head>

<a href="/" class="my-4 flex items-center gap-4 text-2xl font-light hover:underline">
	<ArrowLeft class="h-6 w-6" /> Tilbake hjem
</a>

<div class="mb-4">
	<h1 class="mb-3 text-3xl font-medium">{event.name}</h1>

	<p class="mb-4 text-xl font-light">Registrer en enhet for å bli med på arrangementet!</p>

	{#if beersRegistered.length > 0}
		<p class="text-lg text-gray-600">{beersRegistered.length} enheter registrert totalt</p>
	{/if}
</div>

<div class="my-4 flex items-center gap-4">
	<a
		class="bg-background-dark hover:bg-background-darker flex h-14 w-full items-center justify-center text-center text-lg"
		href="/arrangement/{data.event.id}/registrer">Registrer ny enhet</a
	>
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
			{#each scoreboard as { user: scoreUser, count }, index (scoreUser.id)}
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
							<p class="font-medium">{scoreUser.username}</p>
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
										: 'text-gray-600'}">{count}</span
						>
					</div>
				</div>
			{/each}
		</div>
	</div>

	<div class="bg-background-darkest my-4 h-0.5"></div>
{/if}

{#if beersRegistered.length > 0}
	<div class="space-y-4">
		<h2 class="text-2xl font-medium">Registrerte enheter</h2>

		<div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
			{#each beersRegistered as attendee (attendee.id)}
				<div class="bg-background-dark overflow-hidden rounded-lg">
					{#if attendee.imageId}
						<div class="aspect-square">
							<a
								href="/api/image/{attendee.imageId}"
								target="_blank"
								rel="noopener noreferrer"
								class="block h-full w-full transition-transform hover:scale-105"
								aria-label="Se fullstørrelse bilde fra {attendee.user.username}"
							>
								<img
									src="/api/image/{attendee.imageId}"
									alt="Enhet fra {attendee.user.username}"
									class="h-full w-full cursor-pointer object-cover"
									loading="lazy"
								/>
							</a>
						</div>
					{/if}

					<!-- Beer info -->
					<div class="p-4">
						<div class="flex items-center justify-between gap-2">
							<p class="truncate font-medium">{attendee.user.username}</p>
							<p class="flex-shrink-0 text-xs whitespace-nowrap text-gray-600">
								{attendee.createdAt.toLocaleDateString('no-NO', {
									day: 'numeric',
									month: 'short',
									hour: '2-digit',
									minute: '2-digit'
								})}
							</p>
						</div>
					</div>
				</div>
			{/each}
		</div>
	</div>
{:else}
	<div class="bg-background-dark flex h-32 items-center justify-center rounded-lg p-4">
		<p class="text-lg text-gray-600">Ingen enheter registrert ennå</p>
	</div>
{/if}
