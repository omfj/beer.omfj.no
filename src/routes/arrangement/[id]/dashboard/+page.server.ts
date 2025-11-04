import { error } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { createWsUrl } from '$lib/ws';

export const load: PageServerLoad = async ({ platform, locals, params }) => {
	const id = params.id;

	const event = await locals.db.query.events.findFirst({
		where: (events, { eq }) => eq(events.id, id)
	});

	if (!event) {
		throw error(404, 'Arrangementet ble ikke funnet');
	}

	// Get the 5 latest registrations for this event
	const latestRegistrations = await locals.db.query.attendees
		.findMany({
			where: (attendees, { eq }) => eq(attendees.eventId, id),
			with: {
				user: true,
				drinkType: true,
				drinkSize: true
			},
			orderBy: (attendees, { desc }) => [desc(attendees.createdAt)],
			limit: 9
		})
		.then((attendees) => {
			return attendees.map((attendee) => ({
				id: attendee.id,
				userId: attendee.userId,
				username: attendee.user.username,
				createdAt: attendee.createdAt,
				imageId: attendee.user.hasAgreedToTerms ? attendee.imageId : null,
				drinkType: attendee.drinkType,
				drinkSize: attendee.drinkSize
			}));
		});

	const wsUrl = createWsUrl(platform!.env.WS_HOST, event.id);

	return { event, latestRegistrations, wsUrl };
};
