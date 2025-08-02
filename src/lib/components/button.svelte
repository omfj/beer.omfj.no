<script lang="ts">
	import { cn } from '$lib/cn';
	import type { HTMLButtonAttributes } from 'svelte/elements';

	type Props = HTMLButtonAttributes;

	let { class: className, children, ...rest }: Props = $props();
</script>

<button
	class={cn(
		'bg-background-darker hover:bg-background-darkest h-14 text-xl transition-colors disabled:cursor-not-allowed disabled:opacity-50',
		className
	)}
	{...rest}>{@render children?.()}</button
>
