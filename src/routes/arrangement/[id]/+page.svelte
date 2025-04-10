<script lang="ts">
	import { enhance } from '$app/forms';
	import { Plus, Minus, ArrowLeft } from 'lucide-svelte';
	import { fly } from 'svelte/transition';

	let { data } = $props();
	let event = $derived(data.event);
	let user = $derived(data.user);
	let isAttending = $derived(event.attendees.some((attendee) => attendee.user.id === user.id));
	let sortedAttendees = $derived(event.attendees.sort((a, b) => b.count - a.count));
</script>

<svelte:head>
	<title>{event.name} - Beer Counter</title>
</svelte:head>

<a href="/" class="my-4 flex items-center gap-4 text-2xl font-light hover:underline">
	<ArrowLeft class="h-6 w-6" /> Tilbake hjem
</a>

<div class="mb-4">
	<h1 class="mb-3 text-3xl font-medium">{event.name}</h1>

	<p class="mb-4 text-xl font-light">
		Her finner du informasjon om arrangementet. Trykk på knappen under for å melde deg på.
	</p>
	<p class="mb-4 text-xl font-light">
		Når du er meldt på kan du begynne å registrere antall øl du har drukket.
	</p>

	{#if !isAttending}
		<form method="post" action="?/join" use:enhance>
			<button
				class="bg-background-darker hover:bg-background-darkest flex h-14 w-fit items-center justify-center p-4 text-lg font-medium transition-colors"
			>
				Meld deg på
			</button>
		</form>
	{/if}
</div>

<ul class="flex flex-col gap-5">
	{#each sortedAttendees as attendee (attendee.userId)}
		{@const isUser = attendee.user.id === user.id}
		<li
			class="bg-background-dark relative flex h-20 w-full items-center justify-center p-4 text-3xl"
			transition:fly={{ duration: 200 }}
		>
			{#if isUser}
				<form method="post" action="?/count" use:enhance>
					<input type="hidden" name="action" value="decrement" />
					<button
						class="bg-background-darker absolute top-1/2 left-6 flex size-10 -translate-y-1/2 items-center justify-center rounded-lg"
					>
						<Minus class="h-6 w-6" />
					</button>
				</form>
			{/if}
			<p class="line-clamp-1">
				{attendee.user.username}: {attendee.count}
			</p>
			{#if isUser}
				<form method="post" action="?/count" use:enhance>
					<input type="hidden" name="action" value="increment" />
					<button
						class="bg-background-darker absolute top-1/2 right-6 flex size-10 -translate-y-1/2 items-center justify-center rounded-lg"
					>
						<Plus class="h-6 w-6" />
					</button>
				</form>
			{/if}
		</li>
	{:else}
		<li>
			<p class="bg-background-dark flex items-center justify-center h-20 p-4 transition-colors">
				Ingen er påmeldt
			</p>
		</li>
	{/each}
</ul>
