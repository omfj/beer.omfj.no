<script lang="ts">
	import { ChevronRight, ExternalLink } from '@lucide/svelte';
	import ButtonLink from '$lib/components/button-link.svelte';
	import SEO from '$lib/components/seo.svelte';
	import { resolve } from '$app/paths';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
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

	<div class="flex items-center justify-between">
		<ButtonLink href={resolve('/arrangementer/ny')}>Lag nytt arrangement</ButtonLink>
		<a
			href={resolve('/endringer')}
			class="text-foreground-muted flex items-center gap-1 text-sm underline"
		>
			Endringslogg
			<ExternalLink class="size-3" />
		</a>
	</div>
</div>

<section>
	<h2 class="mb-2 text-xl font-medium">Dine arrangementer</h2>

	<ul class="flex flex-col gap-5">
		{#if data.events.length === 0}
			<li>
				<p class="bg-background-dark flex h-20 items-center justify-center p-4 transition-colors">
					Du har ikke meldt deg på noen arrangementer enda.
				</p>
			</li>
		{:else}
			{#each data.events as event (event.id)}
				<li>
					<a
						class="bg-background-dark hover:bg-background-darker group flex h-24 items-center justify-between p-4 transition-colors"
						href={`/arrangement/${encodeURIComponent(event.id)}`}
					>
						<div class="flex flex-col gap-2">
							<span class="text-3xl group-hover:underline">{event.name}</span>
							<ul class="text-foreground-muted flex items-center gap-4 text-xs">
								<li>{event.distinctUsers} deltakere</li>
								<li>
									{event.totalAttendees} enheter registrert ({event.distinctUsers > 0
										? (event.totalAttendees / event.distinctUsers).toFixed(1)
										: '0.0'})
								</li>
							</ul>
						</div>
						<div>
							<ChevronRight
								class="text-foreground-muted group-hover:text-primary h-6 w-6 transition-colors"
							/>
						</div>
					</a>
				</li>
			{/each}
		{/if}
	</ul>
</section>
