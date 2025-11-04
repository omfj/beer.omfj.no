<script lang="ts">
	import { cn } from '$lib/cn';
	import type { HTMLSelectAttributes } from 'svelte/elements';

	type Props = HTMLSelectAttributes;

	let { value = $bindable(), class: className, children, ...rest }: Props = $props();
</script>

<select
	bind:value
	class={cn(
		'bg-background-dark border-background-darkest focus:ring-primary focus:border-primary h-12 w-full border px-4 py-3 text-lg text-gray-600 focus:ring-2 focus:outline-none',
		className
	)}
	{...rest}
>
	{@render children?.()}
</select>
