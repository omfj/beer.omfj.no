import { redirect, fail } from '@sveltejs/kit';
import type { PageServerLoad, Actions } from './$types';
import { users, sessions, userPasswords, events, attendees } from '$lib/db/schema';
import { count, countDistinct, eq, inArray } from 'drizzle-orm';

export const load: PageServerLoad = async ({ locals }) => {
	if (!locals.user) {
		redirect(302, '/logg-inn');
	}

	const eventIds = await locals.db.query.attendees
		.findMany({
			where: (attendee, { eq }) => eq(attendee.userId, locals.user.id)
		})
		.then((attendees) => attendees.map((attendee) => attendee.eventId));

	const joinedEvents = await locals.db
		.select({
			id: events.id,
			name: events.name,
			totalAttendees: count(attendees.id),
			distinctUsers: countDistinct(attendees.userId)
		})
		.from(events)
		.leftJoin(attendees, eq(events.id, attendees.eventId))
		.where(inArray(events.id, eventIds))
		.groupBy(events.id, events.name)
		.orderBy(events.id);

	const showTermsPopup = !locals.user.hasAgreedToTerms;

	return { joinedEvents, showTermsPopup };
};

export const actions: Actions = {
	acceptTerms: async ({ locals }) => {
		if (!locals.user) {
			return fail(401, { message: 'Not authenticated' });
		}

		await locals.db
			.update(users)
			.set({ hasAgreedToTerms: true })
			.where(eq(users.id, locals.user.id));

		return { success: true };
	},

	deleteAccount: async ({ locals, cookies }) => {
		if (!locals.user) {
			return fail(401, { message: 'Not authenticated' });
		}

		await locals.db.delete(sessions).where(eq(sessions.userId, locals.user.id));
		await locals.db.delete(userPasswords).where(eq(userPasswords.userId, locals.user.id));
		await locals.db.delete(users).where(eq(users.id, locals.user.id));

		cookies.delete('session', { path: '/' });
		redirect(302, '/logg-inn');
	}
};
