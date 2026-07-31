<script lang="ts">
	import '../app.css';

	import { writable } from 'svelte/store';
	import NProgress from 'nprogress';
	import { setUserContext } from '$lib/context/user';
	import type { UserContext } from '$lib/context/user';
	import { afterNavigate, beforeNavigate } from '$app/navigation';
	import { pwaInfo } from 'virtual:pwa-info';
	import { createThemeContext } from '$lib/theme.svelte';

	let webManifestLink = $derived(pwaInfo ? pwaInfo.webManifest.linkTag : '');

	// Initialize theme context
	createThemeContext();

	const { data, children } = $props();

	const user: UserContext = writable(null);
	$effect.pre(() => {
		user.set(data.user);
	});
	setUserContext(user);

	NProgress.configure({
		showSpinner: false,
		minimum: 0.16
	});

	beforeNavigate(async () => {
		NProgress.start();
	});

	afterNavigate(async () => {
		NProgress.done();
	});
</script>

<svelte:head>
	{@html webManifestLink}
</svelte:head>

<div
	class="fixed inset-x-0 top-0 z-50 bg-yellow-300 px-4 py-3 text-center font-semibold text-black"
	role="note"
>
	Husk å drikke vann! 💦💦
</div>

<div class="pt-12">
	{@render children()}
</div>
