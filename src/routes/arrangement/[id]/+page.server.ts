import { error } from '@sveltejs/kit';
import type { PageServerLoad, Actions } from './$types';
import * as table from '$lib/db/schema';
import { eq } from 'drizzle-orm';

export const load: PageServerLoad = async ({ locals, params }) => {
	if (!locals.user) {
		throw error(401, 'Ikke logget inn');
	}

	const id = params.id;
	const event = await locals.db.query.events.findFirst({
		where: (events, { eq }) => eq(events.id, id),
		with: {
			attendees: {
				with: {
					user: true
				}
			}
		}
	});

	if (!event) {
		throw error(404, 'Arrangementet ble ikke funnet');
	}

	return { event };
};

export const actions: Actions = {
	join: async ({ locals, params }) => {
		if (!locals.user) {
			throw error(401, { message: 'Ikke logget inn' });
		}

		const id = params.id;

		const event = await locals.db.query.events.findFirst({
			where: (events, { eq }) => eq(events.id, id)
		});

		if (!event) {
			throw error(404, { message: 'Arrangementet ble ikke funnet' });
		}

		await locals.db
			.insert(table.attendees)
			.values({
				id: crypto.randomUUID(),
				eventId: event.id,
				userId: locals.user.id,
				createdAt: new Date(),
				count: 0
			})
			.returning();

		return { success: true };
	},
	count: async ({ locals, params, request }) => {
		if (!locals.user) {
			throw error(401, { message: 'Ikke logget inn' });
		}

		const id = params.id;

		const event = await locals.db.query.events.findFirst({
			where: (events, { eq }) => eq(events.id, id)
		});

		if (!event) {
			throw error(404, { message: 'Arrangementet ble ikke funnet' });
		}

		const attendee = await locals.db.query.attendees.findFirst({
			where: (attendees, { eq }) => eq(attendees.eventId, event.id)
		});

		if (!attendee) {
			throw error(404, { message: 'Du er ikke påmeldt arrangementet' });
		}

		const formData = await request.formData();
		const action = formData.get('action') as string;
		if (action !== 'increment' && action !== 'decrement') {
			throw error(400, { message: 'Ugyldig handling' });
		}

		const newCount = action === 'increment' ? attendee.count + 1 : Math.max(0, attendee.count - 1);

		await locals.db
			.update(table.attendees)
			.set({ count: newCount })
			.where(eq(table.attendees.id, attendee.id))
			.returning();

		return { success: true };
	}
};
