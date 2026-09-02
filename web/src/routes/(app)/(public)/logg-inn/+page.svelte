<script lang="ts">
	import { onMount } from 'svelte';
	import { enhance } from '$app/forms';
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import Button from '$lib/components/button.svelte';
	import Input from '$lib/components/input.svelte';
	import SEO from '$lib/components/seo.svelte';
	import {
		loginWithPasskey,
		browserSupportsWebAuthnAutofill,
		genericPasskeyError,
		PasskeyCancelledError
	} from '$lib/passkey-client';

	let { form } = $props();

	let event = page.url.searchParams.get('event');
	let registerUrl = $derived(event ? `/registrer?event=${event}` : '/registrer');

	let passkeyError = $state<string | null>(null);

	// WebAuthn conditional UI: a pending passkey request lets the browser or
	// password manager offer passkeys in the username field's autofill
	// dropdown. Restart after a dismissed attempt so the suggestions keep
	// working without a page reload; stop on real errors.
	onMount(() => {
		let active = true;

		(async () => {
			if (!(await browserSupportsWebAuthnAutofill())) return;

			while (active) {
				const startedAt = Date.now();
				try {
					await loginWithPasskey(true);
					if (!active) return;
					await goto(event ? `/arrangement/${event}` : '/', { invalidateAll: true });
					return;
				} catch (e) {
					if (!active) return;
					if (e instanceof PasskeyCancelledError) {
						// A dismissal takes user interaction; an instant
						// rejection means something is wrong — don't re-arm
						// in a tight loop.
						if (Date.now() - startedAt < 1000) return;
						continue;
					}
					passkeyError = e instanceof Error ? e.message : genericPasskeyError;
					return;
				}
			}
		})();

		return () => {
			active = false;
		};
	});
</script>

<SEO
	title="Logg inn"
	description="Logg inn på Beer Counter for å begynne å telle øl på arrangementer med venner."
/>

<h1 class="mb-8 text-5xl">Logg inn</h1>

{#if form?.message}
	<p class="text-red-500">{form.message}</p>
{/if}

{#if passkeyError}
	<p class="text-red-500">{passkeyError}</p>
{/if}

<form class="flex flex-col gap-6" method="post" action="?/login" use:enhance>
	<input type="hidden" name="eventId" value={event} />

	<label class="flex flex-col gap-1 text-xl font-medium">
		Brukernavn
		<Input name="username" autocomplete="username webauthn" required />
	</label>

	<label class="flex flex-col gap-1 text-xl font-medium">
		Passord
		<Input type="password" name="password" autocomplete="current-password webauthn" required />
	</label>

	<Button>Logg inn</Button>
	<!-- eslint-disable-next-line svelte/no-navigation-without-resolve -->
	<a class="text-primary text-center hover:underline" href={registerUrl}>
		Har du ikke bruker? Registrer deg her.</a
	>
</form>
