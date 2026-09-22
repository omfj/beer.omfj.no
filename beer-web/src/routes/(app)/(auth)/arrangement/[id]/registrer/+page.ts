import { error, redirect } from '@sveltejs/kit';
import { ApiError } from '$lib/api';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent, params }) => {
	const { api, authState } = await parent();
	if (authState.status === 'anonymous') {
		redirect(303, `/logg-inn?event=${encodeURIComponent(params.id)}`);
	}

	try {
		const [eventDetail, drinkOptions] = await Promise.all([
			api.getEvent(params.id),
			api.getDrinkOptions()
		]);
		const previousDrink = eventDetail.attendees
			.filter((drink) => drink.userId === authState.user?.id)
			.reduce<
				(typeof eventDetail.attendees)[number] | null
			>((latest, drink) => (!latest || drink.createdAt > latest.createdAt ? drink : latest), null);
		return { event: eventDetail.event, drinkOptions, previousDrink };
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
