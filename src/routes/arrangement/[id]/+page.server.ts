import { error } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ locals, params }) => {
	const id = params.id;

	if (!locals.user) {
		throw error(401, 'Ikke logget inn');
	}

	const event = await locals.db.query.events.findFirst({
		where: (events, { eq }) => eq(events.id, id)
	});

	if (!event) {
		throw error(404, 'Arrangementet ble ikke funnet');
	}

	const attendees = await locals.db.query.attendees
		.findMany({
			where: (attendees, { eq }) => eq(attendees.eventId, id),
			with: {
				user: true
			},
			orderBy: (attendees, { desc }) => [desc(attendees.createdAt)]
		})
		.then((attendees) => {
			return attendees.map((attendee) => ({
				id: attendee.id,
				userId: attendee.userId,
				username: attendee.user.username,
				createdAt: attendee.createdAt,
				imageId: attendee.user.hasAgreedToTerms ? attendee.imageId : null
			}));
		});

	return { event, attendees };
};
