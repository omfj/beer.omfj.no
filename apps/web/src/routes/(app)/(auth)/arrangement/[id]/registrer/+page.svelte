<script lang="ts">
	import { enhance } from '$app/forms';
	import { ArrowLeft, Camera, Upload, X, Loader, SwitchCamera } from '@lucide/svelte';
	import Select from '$lib/components/select.svelte';
	import { calculateDrinkPoints } from '$lib/scoring';
	import { resolve } from '$app/paths';

	let { data, form } = $props();
	let files = $state<FileList | null>(null);
	let fileInput = $state<HTMLInputElement | null>(null);
	let isDragOver = $state(false);
	let isUploading = $state(false);
	let videoElement = $state<HTMLVideoElement | null>(null);
	let canvasElement = $state<HTMLCanvasElement | null>(null);
	let stream = $state<MediaStream | null>(null);
	let showCamera = $state(false);
	let facingMode = $state<'user' | 'environment'>('environment');
	let hasMultipleCameras = $state(false);
	let selectedDrinkType = $state<string>('');
	let selectedDrinkSize = $state<string>('');
	let fileError = $state<string | null>(null);

	const MAX_FILE_SIZE = 10 * 1024 * 1024; // 10MB in bytes

	// Computed available sizes based on selected drink type
	let availableSizes = $derived.by(() => {
		if (!selectedDrinkType) return [];
		return data.drinkTypeSizes
			.filter((combo) => combo.drinkTypeId === selectedDrinkType)
			.map((combo) => data.drinkSizes.find((size) => size.id === combo.drinkSizeId)!)
			.sort((a, b) => {
				if (!a.volumeML || !b.volumeML) return 0;
				return a.volumeML - b.volumeML;
			});
	});

	// Reset size when drink type changes
	$effect(() => {
		if (selectedDrinkType) {
			if (
				availableSizes.length > 0 &&
				!availableSizes
					.filter((size) => size !== undefined)
					.some((size) => size.id === selectedDrinkSize)
			) {
				selectedDrinkSize = '';
			}
		}
	});

	// Calculate preview points based on selected type and size
	let previewPoints = $derived.by(() => {
		if (!selectedDrinkType && !selectedDrinkSize) {
			return 0.5; // Show default when nothing selected
		}

		if (!selectedDrinkType || !selectedDrinkSize) {
			return 0.5; // Show default when only one selected
		}

		const selectedTypeData = data.drinkTypes.find((type) => type.id === selectedDrinkType);
		const selectedSize = availableSizes.find((size) => size?.id === selectedDrinkSize);
		if (!selectedTypeData || !selectedSize) return 0.5;

		return calculateDrinkPoints(selectedSize.volumeML, selectedTypeData.abv);
	});

	// Reactive effect to set up video stream when elements are ready
	$effect(() => {
		if (stream && videoElement && showCamera) {
			videoElement.srcObject = stream;
			videoElement.play().catch(console.error);
		}
	});

	async function checkMultipleCameras() {
		try {
			const devices = await navigator.mediaDevices.enumerateDevices();
			const videoDevices = devices.filter((device) => device.kind === 'videoinput');
			hasMultipleCameras = videoDevices.length > 1;
		} catch (error) {
			console.error('Error checking cameras:', error);
			hasMultipleCameras = false;
		}
	}

	function validateFileSize(file: File): boolean {
		fileError = null;

		if (file.size > MAX_FILE_SIZE) {
			const sizeMB = (file.size / (1024 * 1024)).toFixed(1);
			fileError = `Filen er for stor (${sizeMB} MB). Maksimal filstørrelse er 10 MB.`;
			return false;
		}

		return true;
	}

	async function startCamera() {
		try {
			stream = await navigator.mediaDevices.getUserMedia({
				video: { facingMode: facingMode }
			});
			showCamera = true;

			// Check for multiple cameras after successfully starting
			await checkMultipleCameras();
		} catch (error) {
			console.error('Error accessing camera:', error);
			alert('Kunne ikke få tilgang til kameraet. Kontroller at du har gitt tillatelse.');
		}
	}

	async function flipCamera() {
		if (!stream) return;

		// Stop current stream
		stream.getTracks().forEach((track) => track.stop());

		// Switch facing mode
		facingMode = facingMode === 'environment' ? 'user' : 'environment';

		try {
			// Start new stream with flipped camera
			stream = await navigator.mediaDevices.getUserMedia({
				video: { facingMode: facingMode }
			});
		} catch (error) {
			console.error('Error flipping camera:', error);
			// If flip fails, try to restart with original mode
			facingMode = facingMode === 'environment' ? 'user' : 'environment';
			try {
				stream = await navigator.mediaDevices.getUserMedia({
					video: { facingMode: facingMode }
				});
			} catch (fallbackError) {
				console.error('Error with fallback camera:', fallbackError);
				alert('Kunne ikke bytte kamera.');
			}
		}
	}

	function stopCamera() {
		if (stream) {
			stream.getTracks().forEach((track) => track.stop());
			stream = null;
		}
		showCamera = false;
	}

	function capturePhoto() {
		if (!videoElement || !canvasElement) return;

		const canvas = canvasElement;
		const video = videoElement;

		canvas.width = video.videoWidth;
		canvas.height = video.videoHeight;

		const ctx = canvas.getContext('2d');
		if (!ctx) return;

		ctx.drawImage(video, 0, 0);

		canvas.toBlob(
			(blob) => {
				if (!blob) return;

				// Create a File object from the blob
				const file = new File([blob], `beer-photo-${Date.now()}.jpg`, { type: 'image/jpeg' });

				// Validate file size
				if (!validateFileSize(file)) {
					stopCamera();
					return;
				}

				// Create a FileList-like object
				const dataTransfer = new DataTransfer();
				dataTransfer.items.add(file);
				files = dataTransfer.files;

				if (fileInput) {
					fileInput.files = dataTransfer.files;
				}

				stopCamera();
			},
			'image/jpeg',
			0.9
		);
	}

	// File upload handlers
	const handleDragOver = (e: DragEvent) => {
		e.preventDefault();
		isDragOver = true;
	};

	const handleDragLeave = (e: DragEvent) => {
		e.preventDefault();
		isDragOver = false;
	};

	const handleDrop = (e: DragEvent) => {
		e.preventDefault();
		isDragOver = false;

		if (e.dataTransfer?.files && e.dataTransfer.files.length > 0) {
			const file = e.dataTransfer.files[0];
			if (!validateFileSize(file)) {
				return;
			}

			files = e.dataTransfer.files;
			if (fileInput) {
				fileInput.files = e.dataTransfer.files;
			}
		}
	};

	const openFileDialog = () => {
		fileInput?.click();
	};

	const handleFileSelect = (e: Event) => {
		const target = e.target as HTMLInputElement;
		if (target.files && target.files.length > 0) {
			const file = target.files[0];
			if (!validateFileSize(file)) {
				target.value = ''; // Clear the input
				files = null;
				return;
			}
			files = target.files;
		}
	};

	const removeFile = () => {
		files = null;
		fileError = null;
		if (fileInput) {
			fileInput.value = '';
		}
	};
</script>

<svelte:head>
	<title>Registrer drink - {data.event.name} - Beer Counter</title>
</svelte:head>

<a
	href={resolve('/arrangement/[id]', { id: data.event.id })}
	class="my-4 flex items-center gap-4 text-2xl font-light hover:underline"
>
	<ArrowLeft class="h-6 w-6" /> Tilbake til arrangement
</a>

<div class="mb-8">
	<h1 class="mb-3 text-3xl font-medium">Registrer ny drink</h1>
	<p class="text-xl font-light">Velg type, størrelse og last opp et bilde.</p>
</div>

{#if form?.message}
	<div class="mb-6 border-l-4 border-red-500 bg-red-50 p-4 text-red-700">
		<p class="font-medium">{form.message}</p>
	</div>
{/if}

{#if fileError}
	<div class="mb-6 border-l-4 border-red-500 bg-red-50 p-4 text-red-700">
		<p class="font-medium">{fileError}</p>
	</div>
{/if}

<form
	method="post"
	enctype="multipart/form-data"
	use:enhance={() => {
		isUploading = true;
		return async ({ update }) => {
			await update();
			isUploading = false;
		};
	}}
	class="space-y-6"
>
	<!-- Drink Type and Size Selection -->
	<div class="space-y-4">
		<div>
			<label for="drinkTypeId" class="mb-2 block text-lg font-medium"
				>Drikketype <span class="text-sm text-gray-500">(valgfritt)</span></label
			>
			<Select bind:value={selectedDrinkType} id="drinkTypeId" name="drinkTypeId">
				<option value="">Ikke oppgitt</option>
				{#each data.drinkTypes as drinkType (drinkType.id)}
					<option value={drinkType.id}>{drinkType.name}</option>
				{/each}
			</Select>
		</div>

		{#if selectedDrinkType}
			<div>
				<label for="drinkSizeId" class="mb-2 block text-lg font-medium"
					>Størrelse <span class="text-sm text-gray-500">(valgfritt)</span></label
				>
				<Select bind:value={selectedDrinkSize} id="drinkSizeId" name="drinkSizeId">
					<option value="">Ikke oppgitt</option>
					{#each availableSizes as size (size.id)}
						<option value={size.id}>{size.name} ({size.volumeML}ml)</option>
					{/each}
				</Select>
			</div>
		{/if}

		<div class="bg-primary/10 border-primary/20 border p-4">
			<div class="flex items-center justify-between">
				<span class="text-lg font-medium">Forventet poengsum:</span>
				<span class="text-primary text-2xl font-bold">{previewPoints} poeng</span>
			</div>
			<p class="mt-1 text-sm text-gray-600">
				{#if !selectedDrinkType && !selectedDrinkSize}
					Standard poeng (ingen type/størrelse valgt)
				{:else if !selectedDrinkType || !selectedDrinkSize}
					Standard poeng (mangler type eller størrelse)
				{:else}
					Basert på størrelse og type
				{/if}
			</p>
		</div>
	</div>

	<!-- Hidden file input - always present for form submission -->
	<input
		bind:this={fileInput}
		type="file"
		id="image"
		name="image"
		accept="image/*"
		onchange={handleFileSelect}
		class="sr-only"
		required
	/>

	<!-- svelte-ignore a11y_media_has_caption -->
	<div class="space-y-4">
		{#if showCamera}
			<!-- Camera Interface -->
			<div class="space-y-4">
				<div class="relative">
					<!-- svelte-ignore a11y_media_has_caption -->
					<video bind:this={videoElement} autoplay playsinline class="w-full rounded"></video>

					<!-- Camera Flip Button -->
					{#if hasMultipleCameras}
						<button
							type="button"
							onclick={flipCamera}
							class="absolute top-2 right-2 rounded-full bg-black/50 p-2 text-white transition-colors hover:bg-black/70"
							aria-label="Bytt kamera"
							title="Bytt mellom front- og bakkamera"
						>
							<SwitchCamera class="h-5 w-5" />
						</button>
					{/if}
				</div>

				<div class="flex gap-4">
					<button
						type="button"
						onclick={capturePhoto}
						class="bg-primary hover:bg-primary/90 flex-1 p-3 font-medium text-white"
					>
						Ta bilde
					</button>
					<button
						type="button"
						onclick={stopCamera}
						class="bg-background-darker hover:bg-background-darkest flex-1 p-3 font-medium"
					>
						Avbryt
					</button>
				</div>
			</div>
		{:else if files && files.length > 0}
			<!-- File Preview -->
			<div class="bg-background-dark border-primary space-y-4 border-2 border-dashed p-6">
				<div class="relative">
					<button
						type="button"
						onclick={removeFile}
						class="bg-background-darkest hover:bg-background-darker absolute top-2 right-2 z-10 rounded-full p-2 transition-colors"
						aria-label="Fjern fil"
					>
						<X class="h-4 w-4" />
					</button>

					{#each Array.from(files) as file (file.name)}
						{#if file.type.startsWith('image/')}
							<div class="space-y-4">
								<img
									src={URL.createObjectURL(file)}
									alt="Forhåndsvisning av øl"
									class="max-h-64 w-full rounded object-contain"
								/>
							</div>
						{/if}
					{/each}
				</div>
			</div>
		{:else}
			<!-- File Upload Area -->
			<div
				class="relative border-2 border-dashed transition-all duration-300 {isDragOver
					? 'border-primary bg-primary/5 scale-[1.02]'
					: 'border-background-darker hover:border-background-darkest hover:bg-background-dark/50'}"
				class:cursor-pointer={true}
				ondrop={handleDrop}
				ondragover={handleDragOver}
				ondragleave={handleDragLeave}
				onclick={openFileDialog}
				role="button"
				tabindex="0"
				onkeydown={(e) => {
					if (e.key === 'Enter' || e.key === ' ') {
						e.preventDefault();
						openFileDialog();
					}
				}}
			>
				<!-- Empty State -->
				<div class="flex min-h-48 flex-col items-center justify-center p-8 text-center">
					<div
						class="bg-background-darker mb-4 flex h-16 w-16 items-center justify-center rounded-full"
					>
						<Upload class="h-8 w-8 text-gray-500" />
					</div>
					<div class="space-y-2">
						<p class="text-lg font-medium">
							{isDragOver ? 'Slipp filen her' : 'Last opp bilde av øl'}
						</p>
						<p class="text-sm text-gray-600">Dra og slipp fil her, eller klikk for å velge</p>
						<p class="text-xs text-gray-500">PNG, JPG eller WEBP (maks 10MB)</p>
						<p class="text-xs text-gray-500 italic md:hidden">
							Tips: For best resultat, klikk opplastingsboksen og velg "Ta bilde"
						</p>
					</div>
				</div>
			</div>
		{/if}

		<!-- Camera Button -->
		{#if !showCamera}
			<button
				type="button"
				onclick={startCamera}
				class="bg-background-dark hover:bg-background-darker flex w-full items-center justify-center gap-3 p-4 text-lg font-medium transition-colors"
			>
				<Camera class="h-6 w-6" />
				Ta bilde
			</button>
		{/if}

		<!-- Hidden canvas for photo capture -->
		<canvas bind:this={canvasElement} class="hidden"></canvas>

		<!-- Submit Button -->
		<button
			type="submit"
			disabled={!files || isUploading}
			class="bg-primary hover:bg-primary/90 disabled:bg-background-darker flex w-full items-center justify-center gap-2 p-4 text-lg font-medium text-white transition-colors disabled:text-gray-500"
		>
			{#if isUploading}
				<Loader class="h-5 w-5 animate-spin" />
				Laster opp...
			{:else if !files}
				Velg et bilde først
			{:else}
				Registrer drink
			{/if}
		</button>
	</div>
</form>
