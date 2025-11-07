<script lang="ts">
	import { invalidateAll } from '$app/navigation';
	import qrcode from 'qrcode-generator';

	let { data } = $props();

	let event = $derived(data.event);
	let latestRegistrations = $derived(data.latestRegistrations);
	let qrCanvas: HTMLCanvasElement;
	let imageCacheBuster = $state(Date.now());

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

	function formatTime(date: Date): string {
		return date.toLocaleTimeString('no-NO', {
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	function formatDate(date: Date): string {
		return date.toLocaleDateString('no-NO', {
			day: 'numeric',
			month: 'short'
		});
	}
</script>

<svelte:head>
	<title>Dashboard - {event.name} - Beer Counter</title>
</svelte:head>

<div class="h-screen p-8">
	<div class="mb-8">
		<h1 class="mb-3 text-3xl font-medium">Dashboard for {event.name}</h1>
		<p class="text-foreground-muted text-xl font-light">
			Siste registreringer og QR-kode for å dele arrangementet
		</p>
	</div>

	<div class="grid grid-cols-1 gap-8 lg:grid-cols-3">
		<!-- Latest Registrations - Takes up 2/3 of the width -->
		<div class="lg:col-span-2">
			<h2 class="mb-6 text-2xl font-medium">Siste registreringer</h2>

			{#if latestRegistrations.length === 0}
				<div class="bg-empty-state-background rounded-lg p-8 text-center">
					<p class="text-foreground-muted text-lg">Ingen registreringer ennå</p>
					<p class="text-foreground-light mt-2">
						Registreringer vil vises her når folk begynner å registrere øl
					</p>
				</div>
			{:else}
				<div class="space-y-4">
					<div
						class="grid grid-cols-1 gap-4 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-3 xl:grid-cols-3 2xl:grid-cols-3"
					>
						{#each latestRegistrations.slice(0, 9) as registration (registration.id)}
							<div class="border-border bg-card-background overflow-hidden rounded-lg border">
								<div class="flex flex-col">
									<div class="shrink-0">
										{#if registration.imageId}
											<img
												src="/api/image/{registration.imageId}?v={imageCacheBuster}"
												alt="Drink registration"
												class="h-32 w-full object-cover"
											/>
										{:else}
											<div
												class="flex h-32 w-full items-center justify-center rounded-lg"
												style="background: linear-gradient(to bottom right, var(--color-accent-start), var(--color-accent-end));"
											>
												<span class="text-foreground text-2xl font-medium">
													{registration.username.charAt(0).toUpperCase()}
												</span>
											</div>
										{/if}
									</div>
									<div class="flex items-center justify-between p-4">
										<div>
											<h3 class="text-foreground font-medium">
												{registration.username}
											</h3>
											<p class="text-foreground-muted text-sm">
												{#if registration.drinkType && registration.drinkSize}
													{registration.drinkType.name} ({registration.drinkSize.name})
												{:else if registration.drinkType}
													{registration.drinkType.name}
												{:else}
													Registrering
												{/if}
											</p>
										</div>
										<div class="text-foreground-muted text-right text-sm">
											<div>{formatTime(registration.createdAt)}</div>
											<div>{formatDate(registration.createdAt)}</div>
										</div>
									</div>
								</div>
							</div>
						{/each}
					</div>
				</div>
			{/if}
		</div>

		<!-- QR Code - Takes up 1/3 of the width -->
		<div class="lg:col-span-1">
			<canvas bind:this={qrCanvas} class="border-border-light mx-auto my-auto rounded-lg border-2"
			></canvas>
		</div>
	</div>
</div>
