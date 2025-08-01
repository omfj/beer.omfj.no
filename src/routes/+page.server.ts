import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ locals }) => {
	if (!locals.user) {
		redirect(302, '/logg-inn');
	}

	const eventIds = await locals.db.query.attendees
		.findMany({
			where: (attendee, { eq }) => eq(attendee.userId, locals.user.id)
		})
		.then((attendees) => attendees.map((attendee) => attendee.eventId));

	const events = await locals.db.query.events.findMany({
		where: (event, { inArray }) => inArray(event.id, eventIds)
	});

	return { events };
};
