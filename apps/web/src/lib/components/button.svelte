<script lang="ts">
	import { buttonStyle, type ButtonStyleProps } from '$lib/cva';
	import type { HTMLButtonAttributes } from 'svelte/elements';

	type Props = HTMLButtonAttributes & ButtonStyleProps;

	let { class: className, variant, size, children, ...rest }: Props = $props();
</script>

<button class={buttonStyle({ variant, size, className })} {...rest}>{@render children?.()}</button>
