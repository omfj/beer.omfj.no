<script lang="ts">
	import { enhance } from '$app/forms';
	import { ChevronRight, TriangleAlert } from '@lucide/svelte';
	import Button from '$lib/components/button.svelte';
	import ButtonLink from '$lib/components/button-link.svelte';
	import SEO from '$lib/components/seo.svelte';
	import { resolve } from '$app/paths';

	let { data } = $props();
	let joinedEvents = $derived(data.joinedEvents);
</script>

<SEO
	title="Arrangementer"
	description="Se alle dine arrangementer og opprett nye for å telle øl med venner. Konkurrer om hvem som drikker mest på fest og arrangementer."
/>

<div class="mb-8">
	<h1 class="mb-3 text-3xl font-medium">Arrangementer</h1>

	<p class="mb-4 text-xl font-light">
		Her finner du en oversikt over arrangementene våre. Trykk på et arrangement for å se mer
		informasjon og melde deg på.
	</p>

	<div class="mb-4 border-l-4 border-amber-200 bg-amber-50 p-4">
		<div class="flex items-center gap-2">
			<span class="text-2xl">🍺</span>
			<div>
				<p class="text-sm font-medium text-gray-900">Visste du? Øl gir 10% bonuspoeng!</p>
				<p class="text-xs text-gray-700">
					Vi feirer ølkulturen med en liten bonus, men alle drikketyper er velkomne.
				</p>
			</div>
		</div>
	</div>

	<div class="flex items-center justify-between">
		<ButtonLink href={resolve('/arrangementer/ny')}>Lag nytt arrangement</ButtonLink>
	</div>
</div>

<section>
	<h2 class="mb-2 text-xl font-medium">Dine arrangementer</h2>

	<ul class="flex flex-col gap-5">
		{#each joinedEvents as event (event.id)}
			<li>
				<a
					class="bg-background-dark hover:bg-background-darker group flex h-24 items-center justify-between p-4 transition-colors"
					href={resolve('/arrangement/[id]', {
						id: event.id
					})}
				>
					<div class="flex flex-col gap-2">
						<span class="text-3xl group-hover:underline">{event.name}</span>
						<ul class="text-foreground-muted flex items-center gap-4 text-xs">
							<li>{event.distinctUsers} deltakere</li>
							<li>{event.totalAttendees} enheter registrert</li>
						</ul>
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
					Du har ikke meldt deg på noen arrangementer enda.
				</p>
			</li>
		{/each}
	</ul>
</section>

{#if data.showTermsPopup}
	<div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm">
		<div class="bg-background mx-4 max-w-lg rounded-lg p-6 shadow-2xl">
			<div class="mb-6 flex items-center gap-3">
				<TriangleAlert class="h-8 w-8 text-orange-500" />
				<h2 class="text-2xl font-medium">Godta vilkår</h2>
			</div>

			<div class="mb-6 space-y-4">
				<p class="text-gray-700">
					Du må godta våre vilkår for å fortsette å bruke Beer Counter. Vilkårene beskriver hvordan
					vi håndterer opplastede filer og deling av innhold. <a
						class="text-primary hover:underline"
						href={resolve('/vilkar')}>Les vilkårene her</a
					>.
				</p>

				<p class="text-sm text-gray-700">
					Om du ikke godtar vilkårene, kan du ikke bruke Beer Counter.
				</p>
			</div>

			<div class="flex flex-col gap-3 sm:flex-row">
				<form method="post" action="?/acceptTerms" use:enhance class="flex-1">
					<Button class="w-full bg-green-600 text-white hover:bg-green-700">Godta vilkår</Button>
				</form>

				<form method="post" action="?/deleteAccount" use:enhance>
					<Button class="w-full bg-red-600 px-4 text-white hover:bg-red-700">Slett konto</Button>
				</form>
			</div>
		</div>
	</div>
{/if}
