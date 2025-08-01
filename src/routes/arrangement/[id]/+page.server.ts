import { error } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ locals, params }) => {
	const id = params.id;

	if (!locals.user) {
		throw error(401, 'Ikke logget inn');
	}

	const event = await locals.db.query.events.findFirst({
		where: (events, { eq }) => eq(events.id, id),
		with: {
			attendees: {
				with: {
					user: true
				},
				orderBy: (attendees, { desc }) => [desc(attendees.createdAt)]
			}
		},
		orderBy: (events, { desc }) => [desc(events.createdAt)]
	});

	if (!event) {
		throw error(404, 'Arrangementet ble ikke funnet');
	}

	return { event };
};
