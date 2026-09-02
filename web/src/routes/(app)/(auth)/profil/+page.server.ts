import { fail, redirect, type Actions } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { events, attendees, users, passkeys } from '$lib/db/schema';
import { and, count, countDistinct, eq } from 'drizzle-orm';

export const load: PageServerLoad = async ({ locals }) => {
	if (!locals.user) {
		redirect(302, '/logg-inn');
	}

	const [createdEvents, userPasskeys] = await Promise.all([
		locals.db
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
			.orderBy(events.createdAt),
		locals.db
			.select({
				id: passkeys.id,
				name: passkeys.name,
				createdAt: passkeys.createdAt
			})
			.from(passkeys)
			.where(eq(passkeys.userId, locals.user.id))
			.orderBy(passkeys.createdAt)
	]);

	return {
		user: locals.user,
		createdEvents,
		passkeys: userPasskeys
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
	},
	deletePasskey: async ({ locals, request }) => {
		const user = locals.user;
		if (!user) {
			return fail(401, { message: 'Du er ikke logget inn' });
		}

		const formData = await request.formData();
		const id = formData.get('id')?.toString();
		if (!id) {
			return fail(400, { message: 'Mangler passkey-id' });
		}

		await locals.db.delete(passkeys).where(and(eq(passkeys.id, id), eq(passkeys.userId, user.id)));

		return {
			success: true
		};
	},
	renamePasskey: async ({ locals, request }) => {
		const user = locals.user;
		if (!user) {
			return fail(401, { message: 'Du er ikke logget inn' });
		}

		const formData = await request.formData();
		const id = formData.get('id')?.toString();
		if (!id) {
			return fail(400, { message: 'Mangler passkey-id' });
		}

		const name = formData.get('name')?.toString().trim().slice(0, 64) || null;

		await locals.db
			.update(passkeys)
			.set({ name })
			.where(and(eq(passkeys.id, id), eq(passkeys.userId, user.id)));

		return {
			success: true
		};
	}
};
