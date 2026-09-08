import { redirect } from '@sveltejs/kit';
import { ApiError } from '$lib/api';
import { createApiClient } from '$lib/api/client';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch, parent, url }) => {
	const { authState } = await parent();
	if (authState.status === 'anonymous') redirect(303, '/logg-inn');

	const currentYear = new Date().getFullYear();
	const value = url.searchParams.get('year');
	const year = value && /^\d{4}$/.test(value) && Number(value) > 0 ? Number(value) : currentYear;
	try {
		const api = createApiClient(fetch);
		const result = await api.getLeaderboard(year);
		return {
			year,
			leaderboard: result.leaderboard,
			availableYears: Array.from(new Set([year, ...result.availableYears])).sort((a, b) => b - a)
		};
	} catch (cause) {
		if (cause instanceof ApiError && cause.status === 401) redirect(303, '/logg-inn');
		throw cause;
	}
};
