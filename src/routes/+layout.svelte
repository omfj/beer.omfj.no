<script lang="ts">
	import { writable } from 'svelte/store';
	import '../app.css';
	import { setUserContext } from '$lib/context/user';

	const { data, children } = $props();

	const user = writable(data.user);
	$effect.pre(() => {
		user.set(data.user);
	});
	setUserContext(user);
</script>

<div class="mx-auto w-full max-w-2xl p-8">
	{@render children()}
</div>
