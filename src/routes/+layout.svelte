<script lang="ts">
	import '../app.css';

	import { writable } from 'svelte/store';
	import NProgress from 'nprogress';
	import { setUserContext } from '$lib/context/user';
	import { afterNavigate, beforeNavigate } from '$app/navigation';

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

<div class="mx-auto w-full max-w-2xl p-8">
	{@render children()}
</div>
