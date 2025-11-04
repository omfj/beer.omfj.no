import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { events, attendees } from '$lib/db/schema';
import { count, countDistinct, eq } from 'drizzle-orm';

export const load: PageServerLoad = async ({ locals }) => {
	if (!locals.user) {
		redirect(302, '/logg-inn');
	}

	const createdEvents = await locals.db
		.select({
			id: events.id,
			name: events.name,
			color: events.color,
			createdAt: events.createdAt,
			totalAttendees: count(attendees.id),
			distinctUsers: countDistinct(attendees.userId)
		})
		.from(events)
		.where(eq(events.createdBy, locals.user.id))
		.leftJoin(attendees, eq(events.id, attendees.eventId))
		.groupBy(events.id, events.name, events.color, events.createdAt)
		.orderBy(events.createdAt);

	return {
		user: locals.user,
		createdEvents
	};
};
