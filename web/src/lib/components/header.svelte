<script lang="ts">
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { Menu, Moon, Sun, X } from '@lucide/svelte';
	import { ApiError } from '$lib/api';
	import { api } from '$lib/api/client';
	import { getUser } from '$lib/context/user.svelte';
	import { getThemeContext } from '$lib/context/theme.svelte';

	type Props = {
		isMenuOpen?: boolean;
		onMenuToggle?: () => void;
	};

	let { isMenuOpen = false, onMenuToggle }: Props = $props();

	let theme = getThemeContext();
	const auth = getUser();
	let isLoggingOut = $state(false);
	let logoutError = $state<string | null>(null);

	async function logout() {
		isLoggingOut = true;
		logoutError = null;

		try {
			await api.logout();
			auth.clear();
			await goto('/logg-inn', { replaceState: true });
		} catch (error) {
			if (error instanceof ApiError && error.status === 401) {
				auth.clear();
				await goto('/logg-inn', { replaceState: true });
				return;
			}
			logoutError = error instanceof ApiError ? error.message : 'Kunne ikke logge ut. Prøv igjen.';
		} finally {
			isLoggingOut = false;
		}
	}
</script>

<header class="relative z-50 mb-6 flex items-center justify-between border-b-2 pb-4">
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
	<div class="bg-background fixed inset-0 z-40 min-h-dvh" aria-hidden="true"></div>
	<nav class="relative z-50">
		<menu class="flex flex-col gap-4">
			<li>
				<a href={resolve('/')} class="text-2xl font-light hover:underline">Hjem</a>
			</li>
			<li>
				<a href={resolve('/toppliste')} class="text-2xl font-light hover:underline">Toppliste</a>
			</li>
			<li>
				<a href={resolve('/endringer')} class="text-2xl font-light hover:underline">Endringslogg</a>
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
				<button
					type="button"
					class="text-2xl font-light hover:underline disabled:opacity-50"
					disabled={isLoggingOut}
					onclick={logout}>{isLoggingOut ? 'Logger ut...' : 'Logg ut'}</button
				>
			</li>
			{#if logoutError}
				<li><p class="text-red-500">{logoutError}</p></li>
			{/if}
		</menu>
	</nav>
{/if}
