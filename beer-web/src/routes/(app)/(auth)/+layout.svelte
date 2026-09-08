<script lang="ts">
	import { goto, onNavigate } from '$app/navigation';
	import { page } from '$app/state';
	import { browser } from '$app/environment';
	import Header from '$lib/components/header.svelte';
	import { getUser } from '$lib/context/user.svelte';

	let { children } = $props();

	let isOpen = $state(false);
	const auth = getUser();

	$effect(() => {
		if (auth.state.status === 'anonymous') {
			const eventId = page.params.id;
			const destination = eventId ? `/logg-inn?event=${encodeURIComponent(eventId)}` : '/logg-inn';
			void goto(destination, { replaceState: true });
		}
	});

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

{#if auth.state.status === 'authenticated'}
	<Header isMenuOpen={isOpen} onMenuToggle={toggleMenu} />
	{@render children()}
{/if}
