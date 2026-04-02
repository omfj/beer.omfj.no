import { fail, redirect, type Actions } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { events, attendees, users } from '$lib/db/schema';
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

function parseWeight(value: FormDataEntryValue | null): 'light' | 'medium' | 'heavy' | null {
	if (!value) return null;
	const w = value.toString();
	if (!['light', 'medium', 'heavy'].includes(w)) return null;
	return w as 'light' | 'medium' | 'heavy';
}

function parseGender(value: FormDataEntryValue | null): 'male' | 'female' | 'other' | null {
	if (!value) return null;
	const gender = value.toString();
	if (!['male', 'female', 'other'].includes(gender)) return null;
	return gender as 'male' | 'female' | 'other';
}

export const actions: Actions = {
	saveProfile: async ({ locals, request }) => {
		const user = locals.user;
		if (!user) {
			return fail(401, { message: 'Du er ikke logget inn' });
		}

		const formData = await request.formData();

		const weight = parseWeight(formData.get('weight'));
		const gender = parseGender(formData.get('gender'));

		await locals.db.update(users).set({ weight, gender }).where(eq(users.id, user.id));

		return {
			success: true
		};
	}
};
