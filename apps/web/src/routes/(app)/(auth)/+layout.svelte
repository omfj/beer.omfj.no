<script lang="ts">
	import { onNavigate } from '$app/navigation';
	import { browser } from '$app/environment';
	import Header from '$lib/components/header.svelte';

	let { children } = $props();

	let isOpen = $state(false);

	function toggleMenu() {
		isOpen = !isOpen;

		if (browser) {
			if (isOpen) {
				document.body.style.overflow = 'hidden';
			} else {
				document.body.style.overflow = '';
			}
		}
	}

	onNavigate(() => {
		isOpen = false;
		if (browser) {
			document.body.style.overflow = '';
		}
	});
</script>

<Header isMenuOpen={isOpen} onMenuToggle={toggleMenu} />

{@render children()}
