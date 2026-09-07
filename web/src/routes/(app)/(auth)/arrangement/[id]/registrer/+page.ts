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
		const [eventDetail, drinkOptions] = await Promise.all([
			api.getEvent(params.id),
			api.getDrinkOptions()
		]);
		return { event: eventDetail.event, drinkOptions };
	} catch (cause) {
		if (cause instanceof ApiError && cause.status === 401) {
			redirect(303, `/logg-inn?event=${encodeURIComponent(params.id)}`);
		}
		if (cause instanceof ApiError && cause.status === 403) {
			redirect(303, `/arrangement/${encodeURIComponent(params.id)}/unlock`);
		}
		throw error(cause instanceof ApiError ? cause.status : 500, {
			message: cause instanceof Error ? cause.message : 'Kunne ikke laste siden'
		});
	}
};
