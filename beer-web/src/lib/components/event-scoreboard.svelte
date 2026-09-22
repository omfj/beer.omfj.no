<script lang="ts">
	import { Trophy } from '@lucide/svelte';
	import { fly } from 'svelte/transition';

	type ScoreEntry = {
		user: { id: string; username: string };
		points: number;
		count: number;
		promille: number | null;
	};

	let { entries, currentUserId }: { entries: ScoreEntry[]; currentUserId: string | undefined } =
		$props();
	let limit = $state(10);
	const loadMore = () => {
		limit += 10;
	};
</script>

{#if entries.length > 0}
	<!-- Scoreboard Section -->
	<div class="mb-8 space-y-4">
		<div class="flex items-center gap-3">
			<Trophy class="text-primary h-6 w-6" />
			<h2 class="text-2xl font-medium">Toppliste</h2>
		</div>
		<p class="text-foreground-muted text-sm">
			Promille er et estimat beregnet akkurat nå, basert på registrerte drinker siden første
			registrering. Krever at vekt og kjønn er satt i
			<a href="/profil" class="underline">profilen din</a>.
		</p>

		<div class="space-y-3">
			{#each entries.slice(0, limit) as { user: scoreUser, points, count, promille }, index (scoreUser.id)}
				<div
					class="flex items-center justify-between rounded p-3 transition-colors {index === 0
						? 'bg-primary/10 border-primary/20 border'
						: index === 1
							? 'border border-amber-500/20 bg-amber-500/10'
							: index === 2
								? 'border border-orange-600/20 bg-orange-600/10'
								: 'bg-background-darker'}"
					transition:fly={{ duration: 200, delay: index * 50 }}
				>
					<div class="flex items-center gap-4">
						<div
							class="flex h-8 w-8 items-center justify-center rounded-full text-sm font-bold {index ===
							0
								? 'bg-primary text-white'
								: index === 1
									? 'bg-amber-500 text-white'
									: index === 2
										? 'bg-orange-600 text-white'
										: 'bg-background-darkest text-gray-600'}"
						>
							{#if index < 3}
								{index === 0 ? '🥇' : index === 1 ? '🥈' : '🥉'}
							{:else}
								{index + 1}
							{/if}
						</div>
						<div>
							<p class="font-medium">
								{scoreUser.username} <span class="text-sm text-gray-500">({count} stk)</span>
							</p>
							{#if scoreUser.id === currentUserId}
								<p class="text-primary text-xs">Det er deg!</p>
							{/if}
						</div>
					</div>

					<div class="flex flex-col items-end gap-0.5">
						<div class="flex items-center gap-2">
							<span
								class="text-2xl font-bold {index === 0
									? 'text-primary'
									: index === 1
										? 'text-amber-500'
										: index === 2
											? 'text-orange-600'
											: 'text-gray-600'}">{points}</span
							>
							<span class="text-sm text-gray-500">poeng</span>
						</div>
						{#if promille !== null && count >= 2}
							<span class="text-xs text-gray-500">~{promille.toFixed(2)} ‰</span>
						{/if}
					</div>
				</div>
			{/each}

			{#if entries.length > limit}
				<button
					onclick={loadMore}
					class="hover:bg-background-darker w-full rounded p-3 text-center text-sm text-gray-600 transition-colors"
				>
					Vis 10 flere deltakere ({entries.length - limit} gjenstår)
				</button>
			{/if}
		</div>
	</div>

	<div class="bg-background-darkest my-4 h-0.5"></div>
{/if}
