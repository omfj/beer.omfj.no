<script lang="ts">
	import '../app.css';

	import NProgress from 'nprogress';
	import { createUserContext, setUserContext } from '$lib/context/user.svelte';
	import { afterNavigate, beforeNavigate } from '$app/navigation';
	import { pwaInfo } from 'virtual:pwa-info';
	import { createThemeContext } from '$lib/theme.svelte';

	let webManifestLink = $derived(pwaInfo ? pwaInfo.webManifest.linkTag : '');

	// Initialize theme context
	createThemeContext();

	const { data, children } = $props();

	const auth = createUserContext();
	setUserContext(auth);

	$effect(() => {
		auth.state = data.authState;
	});

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

<div class="pt-12">
	{@render children()}
</div>
