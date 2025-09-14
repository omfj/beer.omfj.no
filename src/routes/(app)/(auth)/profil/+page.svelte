<script lang="ts">
	import { ChevronRight } from '@lucide/svelte';
	import { formatDate } from '$lib/date';
	import { resolve } from '$app/paths';

	let { data } = $props();
	let user = $derived(data.user);
	let createdEvents = $derived(data.createdEvents);
</script>

<svelte:head>
	<title>Profil - Beer Counter</title>
</svelte:head>

<div class="mb-8">
	<h1 class="mb-3 text-3xl font-medium">Din profil</h1>

	<div class="bg-background-dark p-6">
		<div class="space-y-4">
			<div>
				<span class="text-foreground-muted text-sm">Brukernavn</span>
				<p class="text-xl font-medium">{user.username}</p>
			</div>

			{#if user.createdAt}
				<div>
					<span class="text-foreground-muted text-sm">Medlem siden</span>
					<p class="text-lg">{formatDate(user.createdAt)}</p>
				</div>
			{/if}
		</div>
	</div>
</div>

<section>
	<h2 class="mb-2 text-xl font-medium">Arrangementer du har laget ({createdEvents.length})</h2>

	<ul class="flex flex-col gap-5">
		{#each createdEvents as event (event.id)}
			<li>
				<a
					class="bg-background-dark hover:bg-background-darker group flex h-24 items-center justify-between p-4 transition-colors"
					href={resolve('/arrangement/[id]', { id: event.id })}
				>
					<div class="flex items-center gap-4">
						<div
							class="h-4 w-4 shrink-0 rounded-full border"
							style="background-color: {event.color}"
						></div>
						<div class="flex flex-col gap-2">
							<span class="text-2xl group-hover:underline">{event.name}</span>
							<ul class="text-foreground-muted flex items-center gap-4 text-xs">
								<li>{event.distinctUsers} deltakere</li>
								<li>{event.totalAttendees} enheter registrert</li>
								<li>Opprettet {formatDate(event.createdAt)}</li>
							</ul>
						</div>
					</div>
					<div>
						<ChevronRight
							class="text-foreground-muted group-hover:text-primary h-6 w-6 transition-colors"
						/>
					</div>
				</a>
			</li>
		{:else}
			<li>
				<p class="bg-background-dark flex items-center justify-center h-20 p-4 transition-colors">
					Du har enda ikke laget noen arrangementer
				</p>
			</li>
		{/each}
	</ul>
</section>
