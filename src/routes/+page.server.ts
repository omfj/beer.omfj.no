import { redirect, fail } from '@sveltejs/kit';
import type { PageServerLoad, Actions } from './$types';
import { users, sessions, userPasswords } from '$lib/db/schema';
import { eq } from 'drizzle-orm';

export const load: PageServerLoad = async ({ locals }) => {
	if (!locals.user) {
		redirect(302, '/logg-inn');
	}

	const eventIds = await locals.db.query.attendees
		.findMany({
			where: (attendee, { eq }) => eq(attendee.userId, locals.user.id)
		})
		.then((attendees) => attendees.map((attendee) => attendee.eventId));

	const joinedEvents = await locals.db.query.events.findMany({
		where: (event, { inArray }) => inArray(event.id, eventIds)
	});

	const yourEvents = await locals.db.query.events.findMany({
		where: (event, { eq }) => eq(event.createdBy, locals.user.id)
	});

	const showTermsPopup = !locals.user.hasAgreedToTerms;

	return { joinedEvents, yourEvents, showTermsPopup };
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
