import { error, redirect } from '@sveltejs/kit';
import { ApiError } from '$lib/api';
import { api } from '$lib/api/client';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent, params }) => {
	const { authState } = await parent();
	if (authState.status === 'anonymous') {
		redirect(303, `/logg-inn?event=${encodeURIComponent(params.id)}`);
	}

	try {
		await api.getEvent(params.id);
	} catch (cause) {
		if (cause instanceof ApiError && cause.status === 403) return {};
		if (cause instanceof ApiError && cause.status === 401) {
			redirect(303, `/logg-inn?event=${encodeURIComponent(params.id)}`);
		}
		throw error(cause instanceof ApiError ? cause.status : 500, {
			message: cause instanceof Error ? cause.message : 'Kunne ikke finne arrangementet'
		});
	}

	redirect(303, `/arrangement/${encodeURIComponent(params.id)}`);
};
