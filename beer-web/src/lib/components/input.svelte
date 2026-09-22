<script lang="ts">
	import { cn } from '$lib/cn';
	import type { HTMLInputAttributes } from 'svelte/elements';

	type Props = HTMLInputAttributes;

	let { value = $bindable(), class: className, ...rest }: Props = $props();
</script>

<input
	bind:value
	class={cn(
		'bg-background-dark border-background-darkest focus:ring-primary focus:border-primary h-14 border p-4 text-3xl focus:ring-2 focus:outline-none',
		className
	)}
	{...rest}
/>
