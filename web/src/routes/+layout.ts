import type { AuthState } from '$lib/context/user.svelte';
import { api } from '$lib/api/client';

export const ssr = false;

export async function load(): Promise<{ authState: AuthState }> {
	try {
		const { user } = await api.me();
		return { authState: { status: 'authenticated', user } };
	} catch {
		return { authState: { status: 'anonymous', user: null } };
	}
}
