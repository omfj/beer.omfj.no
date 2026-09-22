<script lang="ts">
	let { message, class: className = '' }: { message: string; class?: string } = $props();
</script>

<div class="border-l-4 border-red-500 bg-red-50 p-4 text-red-700 {className}" role="alert">
	<p class="font-medium">{message}</p>
</div>
