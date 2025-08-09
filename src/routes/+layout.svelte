<script lang="ts">
	import '../app.css';

	import { writable } from 'svelte/store';
	import NProgress from 'nprogress';
	import { setUserContext } from '$lib/context/user';
	import { afterNavigate, beforeNavigate } from '$app/navigation';
	import { pwaInfo } from 'virtual:pwa-info';
	import Footer from '$lib/components/footer.svelte';

	let webManifestLink = $derived(pwaInfo ? pwaInfo.webManifest.linkTag : '');

	const { data, children } = $props();

	const user = writable(data.user);
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

<div class="mx-auto w-full max-w-2xl p-8">
	{@render children()}
	<Footer />
</div>
