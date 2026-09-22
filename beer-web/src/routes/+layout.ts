import type { AuthState } from '$lib/context/user.svelte';
import { createApiClient } from '$lib/api/client';
import type { ApiClient } from '$lib/api';
import type { LayoutLoad } from './$types';

export const ssr = false;

export const load: LayoutLoad = async ({
	fetch
}): Promise<{ api: ApiClient; authState: AuthState }> => {
	const api = createApiClient(fetch);
	try {
		const { user } = await api.me();
		return { api, authState: { status: 'authenticated', user } };
	} catch {
		return { api, authState: { status: 'anonymous', user: null } };
	}
};
