<script lang="ts">
	import { enhance } from '$app/forms';
	import { onNavigate } from '$app/navigation';
	import { Menu, X } from '@lucide/svelte';
	import { browser } from '$app/environment';

	let { children } = $props();

	let isOpen = $state(false);

	function toggleMenu() {
		isOpen = !isOpen;

		if (browser) {
			if (isOpen) {
				document.body.style.overflow = 'hidden';
			} else {
				document.body.style.overflow = '';
			}
		}
	}

	onNavigate(() => {
		isOpen = false;
		if (browser) {
			document.body.style.overflow = '';
		}
	});
</script>

<header class="mb-6 flex items-center justify-end border-b-2 pb-4">
	<button onclick={toggleMenu} class="relative z-50">
		<Menu class="size-8" />
	</button>
</header>

{@render children()}

{#if isOpen}
	<div class="bg-background fixed top-0 left-0 z-50 h-full w-full">
		<div class="mx-auto flex h-full max-w-2xl flex-col p-8">
			<header class="mb-6 flex items-center justify-end border-b-2 pb-4">
				<button onclick={toggleMenu} class="relative z-50">
					<X class="size-8" />
				</button>
			</header>

			<nav>
				<menu class="flex flex-col gap-4">
					<li>
						<a href="/" class="text-2xl font-light hover:underline">Hjem</a>
					</li>
					<li>
						<a href="/profil" class="text-2xl font-light hover:underline">Profil</a>
					</li>
					<li>
						<a href="/arrangementer/ny" class="text-2xl font-light hover:underline"
							>Nytt arrangement</a
						>
					</li>
					<li>
						<form action="/logg-ut" method="post" use:enhance>
							<button class="text-2xl font-light hover:underline">Logg ut</button>
						</form>
					</li>
				</menu>
			</nav>
		</div>
	</div>
{/if}
