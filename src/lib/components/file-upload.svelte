<script lang="ts">
	import { cn } from '$lib/cn';
	import type { HTMLInputAttributes } from 'svelte/elements';
	import { Upload } from '@lucide/svelte';

	let dragActive = $state(false);

	type Props = Omit<HTMLInputAttributes, 'type'>;

	let { files = $bindable(), class: className, ...props }: Props = $props();

	let fileInput = $state<HTMLInputElement | null>(null);

	function handleDragOver(e: DragEvent) {
		e.preventDefault();
		dragActive = true;
	}

	function handleDragLeave(e: DragEvent) {
		e.preventDefault();
		dragActive = false;
	}

	function handleDrop(e: DragEvent) {
		e.preventDefault();

		dragActive = false;

		if (e.dataTransfer?.files) {
			if (!fileInput) return;
			fileInput.files = e.dataTransfer.files;
			files = e.dataTransfer.files;
		}
	}

	function handleClick() {
		fileInput?.click();
	}

	function handleFileChange() {
		files = fileInput?.files;
	}
</script>

<div
	class={cn(
		'bg-background-dark flex cursor-pointer flex-col items-center justify-center border-dashed p-8 text-center transition-colors',
		{
			'border-primary bg-background-darker': dragActive,
			'border-gray-300 hover:border-gray-400': !dragActive
		},
		className
	)}
	onclick={handleClick}
	ondragover={handleDragOver}
	ondragleave={handleDragLeave}
	ondrop={handleDrop}
	role="button"
	tabindex="0"
	onkeydown={(e) => e.key === 'Enter' && handleClick()}
>
	<Upload class="mb-4 h-12 w-12 text-gray-400" />
	<p class="mb-2 text-lg font-medium text-gray-700">
		{dragActive ? 'Drop files here' : 'Upload files'}
	</p>
	<p class="text-sm text-gray-500">Drag and drop files here, or click to select files</p>

	{#if files && files.length > 1}
		<div class="mt-4 text-sm text-gray-600">
			{files.length} file{files.length === 1 ? '' : 's'} selected
		</div>
	{/if}

	{#if files && files.length === 1}
		<div class="mt-4 text-sm text-gray-600">
			{#each files as file (file.name)}
				<div class="flex items-center justify-between gap-2">
					<span>{file.name}</span>
					<span class="text-gray-500">({(file.size / 1024).toFixed(2)} KB)</span>
				</div>
			{/each}
		</div>
	{/if}
</div>

<input bind:this={fileInput} type="file" class="hidden" onchange={handleFileChange} {...props} />
