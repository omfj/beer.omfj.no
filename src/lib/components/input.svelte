<script lang="ts">
	import { cn } from '$lib/cn';
	import type { HTMLInputAttributes } from 'svelte/elements';

	type Props = HTMLInputAttributes;

	let { value = $bindable(), class: className, ...rest }: Props = $props();
</script>

<input bind:value class={cn('bg-background-dark h-14 p-4 text-3xl', className)} {...rest} />
