<script lang="ts">
	import { cn } from '$lib/cn';
	import type { HTMLInputAttributes } from 'svelte/elements';

	type Props = Omit<HTMLInputAttributes, 'type'> & {
		checked?: boolean;
		label?: string;
	};

	let { checked = $bindable(false), label, class: className, ...props }: Props = $props();
</script>

<label class={cn('flex cursor-pointer items-center gap-3', className)}>
	<div class="relative">
		<input type="checkbox" bind:checked class="sr-only" {...props} />

		<div
			class={cn(
				'focus-within:ring-primary h-6 w-6 border-2 border-gray-400 transition-colors duration-200 focus-within:ring-2 focus-within:ring-offset-2',
				{
					'hover:border-background-darkest bg-white': !checked
				}
			)}
		>
			{#if checked}
				<div
					class="absolute top-1/2 left-1/2 h-3 w-3 -translate-x-1/2 -translate-y-1/2 transform bg-neutral-600 transition-transform duration-200"
				></div>
			{/if}
		</div>
	</div>

	{#if label}
		<span class="text-sm select-none">{label}</span>
	{/if}
</label>
