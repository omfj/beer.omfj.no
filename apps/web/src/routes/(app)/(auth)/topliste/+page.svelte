<script lang="ts">
	import SEO from '$lib/components/seo.svelte';
	import { Trophy, Medal, TrendingUp } from '@lucide/svelte';
	import { goto } from '$app/navigation';

	let { data } = $props();
	let leaderboard = $derived(data.leaderboard);
	let selectedYear = $derived(data.selectedYear);
	let availableYears = $derived(data.availableYears);

	function handleYearChange(event: Event) {
		const select = event.target as HTMLSelectElement;
		const year = select.value;
		if (year === 'all') {
			goto('/topliste');
		} else {
			goto(`/topliste?year=${year}`);
		}
	}

	function getRankIcon(position: number) {
		if (position === 1) return Trophy;
		if (position === 2 || position === 3) return Medal;
		return TrendingUp;
	}

	function getRankColor(position: number) {
		if (position === 1) return 'text-yellow-500';
		if (position === 2) return 'text-gray-400';
		if (position === 3) return 'text-amber-700';
		return 'text-foreground-muted';
	}
</script>

<SEO
	title="Topliste"
	description="Se hvem som har drukket mest og samlet flest poeng. Konkurrer med venner og klatre på topplisten."
/>

<div class="mb-8">
	<h1 class="mb-3 text-3xl font-medium">Topliste</h1>

	<p class="mb-4 text-xl font-light">
		Her finner du de 10 beste drikkerne rangert etter poeng. Filtrer etter år eller se totalen for
		alle tider.
	</p>

	<div class="flex items-center gap-4">
		<label for="year-filter" class="text-sm font-medium">Filtrer etter år:</label>
		<select
			id="year-filter"
			class="bg-background-dark hover:bg-background-darker rounded border border-gray-700 px-4 py-2 transition-colors"
			value={selectedYear?.toString() ?? 'all'}
			onchange={handleYearChange}
		>
			<option value="all">Alle tider</option>
			{#each availableYears as year}
				<option value={year}>{year}</option>
			{/each}
		</select>
	</div>
</div>

<section>
	<h2 class="mb-4 text-xl font-medium">
		{selectedYear ? `Topp 10 - ${selectedYear}` : 'Topp 10 - Alle tider'}
	</h2>

	{#if leaderboard.length > 0}
		<ol class="flex flex-col gap-3">
			{#each leaderboard as entry, index (entry.userId)}
				{@const position = index + 1}
				{@const RankIcon = getRankIcon(position)}
				{@const rankColor = getRankColor(position)}
				<li>
					<div
						class="bg-background-dark hover:bg-background-darker group flex items-center justify-between p-4 transition-colors"
					>
						<div class="flex items-center gap-4">
							<div class="flex w-12 items-center justify-center">
								<RankIcon class="{rankColor} h-6 w-6" />
							</div>
							<div class="flex flex-col gap-1">
								<span class="text-xl font-medium">{entry.username}</span>
								<span class="text-foreground-muted text-sm"
									>{entry.drinkCount} enheter registrert</span
								>
							</div>
						</div>
						<div class="flex flex-col items-end gap-1">
							<span class="text-primary text-2xl font-bold">{entry.points}</span>
							<span class="text-foreground-muted text-sm">poeng</span>
						</div>
					</div>
				</li>
			{/each}
		</ol>
	{:else}
		<div class="bg-background-dark flex h-32 items-center justify-center p-4">
			<p class="text-foreground-muted">
				{selectedYear
					? `Ingen registreringer funnet for ${selectedYear}.`
					: 'Ingen registreringer funnet enda.'}
			</p>
		</div>
	{/if}
</section>
