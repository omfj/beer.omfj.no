<script lang="ts">
	import { cn } from '$lib/cn';
	import type { HTMLSelectAttributes } from 'svelte/elements';

	type Props = HTMLSelectAttributes;

	let { value = $bindable(), class: className, ...rest }: Props = $props();
</script>

<select bind:value class={cn('bg-background-dark h-14 p-4 text-3xl', className)} {...rest}>
	{@render children?.()}
</select>