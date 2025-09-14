<script lang="ts">
	import { resolve } from '$app/paths';
	import Button from '$lib/components/button.svelte';
	import { ArrowLeft, Download, Copy, Check } from '@lucide/svelte';
	import qrcode from 'qrcode-generator';

	let { data } = $props();

	let event = $derived(data.event);
	let qrCanvas: HTMLCanvasElement;
	let copySuccess = $state(false);

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

	function generateQRCode() {
		// Create QR code using qrcode-generator library
		const qr = qrcode(0, 'M');
		qr.addData(eventUrl);
		qr.make();

		// Get canvas context
		const ctx = qrCanvas.getContext('2d');
		if (!ctx) return;

		const modules = qr.getModuleCount();
		const cellSize = 8;
		const margin = cellSize * 4;
		const canvasSize = modules * cellSize + 2 * margin;

		qrCanvas.width = canvasSize;
		qrCanvas.height = canvasSize;

		// White background
		ctx.fillStyle = '#ffffff';
		ctx.fillRect(0, 0, canvasSize, canvasSize);

		// Black modules
		ctx.fillStyle = '#1a1a1a';
		for (let row = 0; row < modules; row++) {
			for (let col = 0; col < modules; col++) {
				if (qr.isDark(row, col)) {
					ctx.fillRect(margin + col * cellSize, margin + row * cellSize, cellSize, cellSize);
				}
			}
		}
	}

	const copyUrl = async () => {
		await navigator.clipboard.writeText(eventUrl);
		copySuccess = true;
		setTimeout(() => {
			copySuccess = false;
		}, 2000);
	};

	const downloadQR = () => {
		if (qrCanvas) {
			const link = document.createElement('a');
			link.download = `${event.name}-qr.png`;
			link.href = qrCanvas.toDataURL();
			link.click();
		}
	};
</script>

<svelte:head>
	<title>QR-kode for {event.name} - Beer Counter</title>
</svelte:head>

<a
	href={resolve('/arrangement/[id]', {
		id: event.id
	})}
	class="my-4 flex items-center gap-4 text-2xl font-light hover:underline"
>
	<ArrowLeft class="h-6 w-6" /> Tilbake til arrangement
</a>

<div class="mb-8">
	<h1 class="mb-3 text-3xl font-medium">QR-kode for {event.name}</h1>
	<p class="text-xl font-light text-gray-600">
		Del denne QR-koden for enkel tilgang til arrangementet
	</p>
</div>

<div class="mx-auto max-w-md space-y-6">
	<!-- QR Code Container -->
	<div>
		<div class="mb-4 flex justify-center">
			<canvas bind:this={qrCanvas} class="mx-auto border-2"></canvas>
		</div>

		<p class="text-sm break-all text-gray-600">{eventUrl}</p>
	</div>

	<!-- Action Buttons -->
	<div class="flex flex-col gap-3">
		<Button onclick={copyUrl} class="w-full gap-2" variant={copySuccess ? 'default' : 'outline'}>
			{#if copySuccess}
				<Check class="size-5" />
				Kopiert!
			{:else}
				<Copy class="size-5" />
				Kopier lenke
			{/if}
		</Button>

		<Button onclick={downloadQR} class="w-full gap-2" variant="outline">
			<Download class="size-5" />
			Last ned QR-kode
		</Button>
	</div>
</div>
