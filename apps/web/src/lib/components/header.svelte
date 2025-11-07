<script lang="ts">
	import { enhance } from '$app/forms';
	import { resolve } from '$app/paths';
	import { Menu, Moon, Sun, X } from '@lucide/svelte';
	import { getThemeContext } from '$lib/theme.svelte';

	type Props = {
		isMenuOpen?: boolean;
		onMenuToggle?: () => void;
	};

	let { isMenuOpen = false, onMenuToggle }: Props = $props();

	let theme = getThemeContext();
</script>

<header class="mb-6 flex items-center justify-between border-b-2 pb-4">
	<h1 class="text-2xl font-medium">
		<a href={resolve('/')} class="hover:underline">Beer Counter</a>
	</h1>

	<div class="flex items-center">
		<button onclick={() => theme.toggle()} class="mr-4">
			{#if theme.current === 'dark'}
				<Sun class="size-7" />
			{:else}
				<Moon class="size-7" />
			{/if}
		</button>

		{#if onMenuToggle}
			<button onclick={onMenuToggle} class="relative z-50">
				{#if isMenuOpen}
					<X class="size-8" />
				{:else}
					<Menu class="size-8" />
				{/if}
			</button>
		{/if}
	</div>
</header>

{#if isMenuOpen && onMenuToggle}
	<div class="bg-background fixed top-0 left-0 z-50 h-full w-full">
		<div class="mx-auto flex h-full max-w-2xl flex-col p-8">
			<header class="mb-6 flex items-center justify-between border-b-2 pb-4">
				<h1 class="text-2xl font-medium">
					<a href={resolve('/')} class="hover:underline">Beer Counter</a>
				</h1>

				<div class="flex items-center">
					<button onclick={() => theme.toggle()} class="mr-4">
						{#if theme.current === 'dark'}
							<Sun class="size-7" />
						{:else}
							<Moon class="size-7" />
						{/if}
					</button>

					<button onclick={onMenuToggle} class="relative z-50">
						<X class="size-8" />
					</button>
				</div>
			</header>

			<nav>
				<menu class="flex flex-col gap-4">
					<li>
						<a href={resolve('/')} class="text-2xl font-light hover:underline">Hjem</a>
					</li>
					<li>
						<a href={resolve('/profil')} class="text-2xl font-light hover:underline">Profil</a>
					</li>
					<li>
						<a href={resolve('/arrangementer/ny')} class="text-2xl font-light hover:underline"
							>Nytt arrangement</a
						>
					</li>
					<li>
						<form action={resolve('/logg-ut')} method="post" use:enhance>
							<button class="text-2xl font-light hover:underline">Logg ut</button>
						</form>
					</li>
				</menu>
			</nav>
		</div>
	</div>
{/if}
