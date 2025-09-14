<script lang="ts">
	import Footer from '$lib/components/footer.svelte';

	const { children } = $props();
</script>

<div class="mx-auto w-full max-w-2xl p-8">
	{@render children()}
	<Footer />
</div>
