import { redirect, fail } from '@sveltejs/kit';
import type { PageServerLoad, Actions } from './$types';
import { users, sessions, userPasswords, events, attendees, eventAccess } from '$lib/db/schema';
import { count, countDistinct, desc, eq, inArray } from 'drizzle-orm';

export const load: PageServerLoad = async ({ locals }) => {
	if (!locals.user) {
		redirect(302, '/logg-inn');
	}

	const userId = locals.user.id;

	const [attendeeEventIds, accessEventIds, createdEventIds] = await Promise.all([
		locals.db
			.select({ eventId: attendees.eventId })
			.from(attendees)
			.where(eq(attendees.userId, userId))
			.then((rows) => rows.map((r) => r.eventId)),
		locals.db
			.select({ eventId: eventAccess.eventId })
			.from(eventAccess)
			.where(eq(eventAccess.userId, userId))
			.then((rows) => rows.map((r) => r.eventId)),
		locals.db
			.select({ id: events.id })
			.from(events)
			.where(eq(events.createdBy, userId))
			.then((rows) => rows.map((r) => r.id))
	]);

	const allEventIds = [...new Set([...attendeeEventIds, ...accessEventIds, ...createdEventIds])];

	if (allEventIds.length === 0) {
		return { joinedEvents: [], showTermsPopup: !locals.user.hasAgreedToTerms };
	}

	const joinedEvents = await locals.db
		.select({
			id: events.id,
			name: events.name,
			totalAttendees: count(attendees.id),
			distinctUsers: countDistinct(attendees.userId)
		})
		.from(events)
		.leftJoin(attendees, eq(events.id, attendees.eventId))
		.where(inArray(events.id, allEventIds))
		.groupBy(events.id, events.name)
		.orderBy(desc(events.createdAt));

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
