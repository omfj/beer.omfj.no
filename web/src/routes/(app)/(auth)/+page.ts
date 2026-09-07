import { redirect } from '@sveltejs/kit';
import { ApiError } from '$lib/api';
import { api } from '$lib/api/client';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent }) => {
	const { authState } = await parent();
	if (authState.status === 'anonymous') redirect(303, '/logg-inn');

	try {
		const { events } = await api.listEvents();
		return { events };
	} catch (cause) {
		if (cause instanceof ApiError && cause.status === 401) redirect(303, '/logg-inn');
		throw cause;
	}
};
