<script lang="ts">
	import { buttonStyle, type ButtonStyleProps } from '$lib/cva';
	import type { HTMLAnchorAttributes } from 'svelte/elements';

	type Props = HTMLAnchorAttributes & ButtonStyleProps;

	let { class: className, variant, size, link = true, children, ...rest }: Props = $props();
</script>

<a class={buttonStyle({ variant, size, link, className })} {...rest}>{@render children?.()}</a>
