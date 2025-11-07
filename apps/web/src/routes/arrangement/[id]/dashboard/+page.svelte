<script lang="ts">
	import { invalidateAll } from '$app/navigation';
	import qrcode from 'qrcode-generator';
	import { calculateDrinkPoints } from '$lib/scoring';
	import { SvelteMap } from 'svelte/reactivity';

	let { data } = $props();

	let event = $derived(data.event);
	let latestRegistrations = $derived(data.latestRegistrations);
	let attendeesForStats = $derived(data.attendeesForStats);
	let qrCanvas: HTMLCanvasElement;
	let imageCacheBuster = $state(Date.now());

	// Calculate stats
	let totalRegistrations = $derived(attendeesForStats.length);

	let uniqueParticipants = $derived(new Set(attendeesForStats.map((a) => a.userId)).size);

	let topUsers = $derived.by(() => {
		const userStats = new SvelteMap<
			string,
			{
				username: string;
				points: number;
				count: number;
			}
		>();

		attendeesForStats.forEach((attendee) => {
			const userId = attendee.userId;
			const points = calculateDrinkPoints(
				attendee.drinkSize?.volumeML || null,
				attendee.drinkType?.abv || null
			);

			if (userStats.has(userId)) {
				const stats = userStats.get(userId)!;
				stats.points += points;
				stats.count++;
			} else {
				userStats.set(userId, {
					username: attendee.username,
					points: points,
					count: 1
				});
			}
		});

		return Array.from(userStats.values())
			.map((entry) => ({
				...entry,
				points: Math.round(entry.points * 10) / 10
			}))
			.sort((a, b) => b.points - a.points)
			.slice(0, 4);
	});

	let totalPoints = $derived.by(() => {
		return attendeesForStats.reduce((sum, attendee) => {
			return (
				sum +
				calculateDrinkPoints(attendee.drinkSize?.volumeML || null, attendee.drinkType?.abv || null)
			);
		}, 0);
	});

	const eventUrl = $derived.by(() => {
		if (typeof window !== 'undefined') {
			const baseUrl = window.location.origin;
			return `${baseUrl}/arrangement/${event.id}`;
		}
		return '';
	});

	$effect(() => {
		if (qrCanvas && eventUrl) {
			generateQRCode();
		}
	});

	const ws = new WebSocket(data.wsUrl);

	$effect(() => {
		ws.onmessage = (event) => {
			const message = event.data;
			if (message === 'UPDATE') {
				imageCacheBuster = Date.now();
				invalidateAll();
			}
		};

		return () => {
			ws.close();
		};
	});

	function generateQRCode() {
		const qr = qrcode(0, 'M');
		qr.addData(eventUrl);
		qr.make();

		const ctx = qrCanvas.getContext('2d');
		if (!ctx) return;

		const modules = qr.getModuleCount();
		const cellSize = 6;
		const margin = cellSize * 2;
		const canvasSize = modules * cellSize + 2 * margin;

		qrCanvas.width = canvasSize;
		qrCanvas.height = canvasSize;

		// Background color
		ctx.fillStyle =
			getComputedStyle(document.documentElement).getPropertyValue('--color-qr-background').trim() ||
			'#ffffff';
		ctx.fillRect(0, 0, canvasSize, canvasSize);

		// Foreground color
		ctx.fillStyle =
			getComputedStyle(document.documentElement).getPropertyValue('--color-qr-foreground').trim() ||
			'#1a1a1a';
		for (let row = 0; row < modules; row++) {
			for (let col = 0; col < modules; col++) {
				if (qr.isDark(row, col)) {
					ctx.fillRect(margin + col * cellSize, margin + row * cellSize, cellSize, cellSize);
				}
			}
		}
	}
</script>

<svelte:head>
	<title>Dashboard - {event.name} - Beer Counter</title>
</svelte:head>

<div class="flex h-screen flex-col p-4">
	<div class="mb-4">
		<h1 class="text-5xl font-medium">{event.name}</h1>
	</div>

	<div class="flex flex-1 gap-4 overflow-hidden">
		<!-- Latest Registrations -->
		<div class="flex-1 overflow-hidden">
			{#if latestRegistrations.length === 0}
				<div class="bg-empty-state-background flex h-full items-center justify-center rounded-lg">
					<p class="text-foreground-muted">Ingen registreringer ennå</p>
				</div>
			{:else}
				<div class="grid h-full grid-cols-2 grid-rows-2 gap-3">
					{#each latestRegistrations.slice(0, 4) as registration (registration.id)}
						<div
							class="border-border bg-card-background flex min-h-0 flex-col overflow-hidden rounded-lg border"
						>
							<div class="relative min-h-0 flex-1 overflow-hidden">
								{#if registration.imageId}
									<img
										src="/api/image/{registration.imageId}?v={imageCacheBuster}"
										alt="Drink registration"
										class="h-full w-full object-cover"
									/>
								{:else}
									<div
										class="flex h-full w-full items-center justify-center"
										style="background: linear-gradient(to bottom right, var(--color-accent-start), var(--color-accent-end));"
									>
										<span class="text-foreground text-4xl font-medium">
											{registration.username.charAt(0).toUpperCase()}
										</span>
									</div>
								{/if}
							</div>
							<div
								class="bg-card-background border-border flex h-10 min-h-10 shrink-0 items-center justify-center border-t px-2"
							>
								<h3 class="text-foreground w-full truncate text-center text-sm font-medium">
									{registration.username}
								</h3>
							</div>
						</div>
					{/each}
				</div>
			{/if}
		</div>

		<!-- QR Code and Stats -->
		<div class="flex w-80 flex-col gap-3">
			<div class="flex items-center justify-center">
				<canvas bind:this={qrCanvas} class="border-border-light rounded-lg border-2"></canvas>
			</div>

			<!-- Stats -->
			<div class="border-border bg-card-background flex flex-col gap-3 rounded-lg border p-3">
				<div class="border-border-light border-b pb-3">
					<h3 class="text-foreground text-lg font-medium">Statistikk</h3>
				</div>

				<div class="space-y-3">
					<div class="flex items-center justify-between">
						<span class="text-foreground-muted text-sm">Totalt registrert</span>
						<span class="text-foreground text-lg font-medium">{totalRegistrations}</span>
					</div>

					<div class="flex items-center justify-between">
						<span class="text-foreground-muted text-sm">Deltakere</span>
						<span class="text-foreground text-lg font-medium">{uniqueParticipants}</span>
					</div>

					<div class="flex items-center justify-between">
						<span class="text-foreground-muted text-sm">Totale poeng</span>
						<span class="text-foreground text-lg font-medium"
							>{Math.round(totalPoints * 10) / 10}</span
						>
					</div>
				</div>

				{#if topUsers.length > 0}
					<div class="border-border-light mt-2 border-t pt-3">
						<h4 class="text-foreground-muted mb-3 text-sm font-medium">Toppliste</h4>
						<div class="space-y-2">
							{#each topUsers as { username, points, count }, index (username)}
								<div class="flex items-center justify-between">
									<div class="flex items-center gap-2">
										<span class="text-foreground-muted text-xs font-medium">
											{index === 0
												? '🥇'
												: index === 1
													? '🥈'
													: index === 2
														? '🥉'
														: `${index + 1}.`}
										</span>
										<span class="text-foreground text-sm">{username}</span>
									</div>
									<div class="flex items-center gap-2">
										<span class="text-foreground-muted text-xs">({count})</span>
										<span class="text-foreground text-sm font-medium">{points}</span>
									</div>
								</div>
							{/each}
						</div>
					</div>
				{/if}
			</div>
		</div>
	</div>
</div>
