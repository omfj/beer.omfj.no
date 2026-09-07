import { error, redirect } from '@sveltejs/kit';
import { ApiError } from '$lib/api';
import { createApiClient } from '$lib/api/client';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch, parent, params }) => {
	const { authState } = await parent();
	if (authState.status === 'anonymous') {
		redirect(303, `/logg-inn?event=${encodeURIComponent(params.id)}`);
	}

	try {
		const api = createApiClient(fetch);
		const { event } = await api.getEvent(params.id);
		return { event };
	} catch (cause) {
		if (cause instanceof ApiError && cause.status === 401) {
			redirect(303, `/logg-inn?event=${encodeURIComponent(params.id)}`);
		}
		if (cause instanceof ApiError && cause.status === 403) {
			redirect(303, `/arrangement/${encodeURIComponent(params.id)}/unlock`);
		}
		throw error(cause instanceof ApiError ? cause.status : 500, {
			message: cause instanceof Error ? cause.message : 'Kunne ikke laste QR-koden'
		});
	}
};
