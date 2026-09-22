<script lang="ts">
	import Button from './button.svelte';
	import Checkbox from './checkbox.svelte';
	import { SvelteSet } from 'svelte/reactivity';

	type User = { id: string; username: string };
	let {
		entries,
		selectedUserIds = $bindable()
	}: {
		entries: { user: User }[];
		selectedUserIds: Set<string>;
	} = $props();
	let filterOpen = $state(false);
	const toggleUser = (id: string) => {
		const next = new SvelteSet(selectedUserIds);
		if (next.has(id)) next.delete(id);
		else next.add(id);
		selectedUserIds = next;
	};
	let filterLabel = $derived(
		selectedUserIds.size === 0
			? 'Alle deltakere'
			: selectedUserIds.size === 1
				? (entries.find((e) => selectedUserIds.has(e.user.id))?.user.username ?? '1 valgt')
				: `${selectedUserIds.size} valgt`
	);
</script>

{#if entries.length > 1}
	<div class="relative mb-4">
		<p class="text-foreground-muted mb-1 text-sm">Filtrer:</p>
		<button
			onclick={() => (filterOpen = !filterOpen)}
			class="bg-background-dark border-background-darkest focus:ring-primary focus:border-primary flex h-12 w-full items-center justify-between border px-4 text-lg text-gray-600 focus:ring-2 focus:outline-none"
		>
			<span>{filterLabel}</span>
			<svg
				class="h-5 w-5 text-gray-400 transition-transform {filterOpen ? 'rotate-180' : ''}"
				fill="none"
				viewBox="0 0 24 24"
				stroke="currentColor"
				stroke-width="2"
			>
				<path stroke-linecap="round" stroke-linejoin="round" d="M19 9l-7 7-7-7" />
			</svg>
		</button>

		{#if filterOpen}
			<div
				class="bg-background-dark border-background-darkest absolute z-10 mt-1 w-full border shadow-lg"
			>
				{#each entries as { user: u } (u.id)}
					<div class="hover:bg-background-darker px-4 py-3">
						<Checkbox
							checked={selectedUserIds.has(u.id)}
							onchange={() => toggleUser(u.id)}
							label={u.username}
						/>
					</div>
				{/each}
				{#if selectedUserIds.size > 0}
					<div class="border-background-darkest border-t px-4 py-3">
						<Button variant="ghost" size="sm" onclick={() => (selectedUserIds = new Set())}>
							Nullstill filter
						</Button>
					</div>
				{/if}
			</div>
		{/if}
	</div>
{/if}
