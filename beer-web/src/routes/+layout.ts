import type { AuthState } from '$lib/context/user.svelte';
import { createApiClient } from '$lib/api/client';
import type { LayoutLoad } from './$types';

export const ssr = false;

export const load: LayoutLoad = async ({ fetch }): Promise<{ authState: AuthState }> => {
	try {
		const api = createApiClient(fetch);
		const { user } = await api.me();
		return { authState: { status: 'authenticated', user } };
	} catch {
		return { authState: { status: 'anonymous', user: null } };
	}
};
