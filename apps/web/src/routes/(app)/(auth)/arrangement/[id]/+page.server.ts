import { error, fail, redirect } from '@sveltejs/kit';
import type { PageServerLoad, Actions } from './$types';
import * as table from '$lib/db/schema';
import { eq, and } from 'drizzle-orm';
import { hasEventAccess } from '$lib/event-access';

export const load: PageServerLoad = async ({ locals, params }) => {
	const id = params.id;

	if (!locals.user) {
		throw redirect(307, `/logg-inn?event=${id}`);
	}

	const event = await locals.db.query.events.findFirst({
		where: (events, { eq }) => eq(events.id, id)
	});

	if (!event) {
		throw error(404, 'Arrangementet ble ikke funnet');
	}

	const access = await hasEventAccess(locals.db, event, locals.user.id);
	if (!access) {
		throw redirect(302, `/arrangement/${id}/unlock`);
	}

	const [rawAttendees, accessRecords] = await Promise.all([
		locals.db.query.attendees.findMany({
			where: (attendees, { eq }) => eq(attendees.eventId, id),
			with: {
				user: true,
				drinkType: true,
				drinkSize: true
			},
			orderBy: (attendees, { desc }) => [desc(attendees.createdAt)]
		}),
		locals.db.query.eventAccess.findMany({
			where: (ea, { eq }) => eq(ea.eventId, id),
			with: { user: true }
		})
	]);

	const attendees = rawAttendees.map((attendee) => ({
		id: attendee.id,
		userId: attendee.userId,
		username: attendee.user.username,
		createdAt: attendee.createdAt,
		imageId: attendee.user.hasAgreedToTerms ? attendee.imageId : null,
		drinkType: attendee.drinkType,
		drinkSize: attendee.drinkSize,
		userWeight: attendee.user.weight,
		userGender: attendee.user.gender
	}));

	const accessUsers = accessRecords.map((r) => ({
		id: r.user.id,
		username: r.user.username,
		weight: r.user.weight,
		gender: r.user.gender
	}));

	// Include creator if not already present
	if (event.createdBy && !accessUsers.find((u) => u.id === event.createdBy)) {
		const creator = await locals.db.query.users.findFirst({
			where: (u, { eq }) => eq(u.id, event.createdBy!)
		});
		if (creator) {
			accessUsers.push({
				id: creator.id,
				username: creator.username,
				weight: creator.weight,
				gender: creator.gender
			});
		}
	}

	return { event, attendees, accessUsers };
};

export const actions: Actions = {
	delete: async ({ request, locals, params }) => {
		if (!locals.user) {
			return fail(401, {
				message: 'Ikke logget inn'
			});
		}

		const eventId = params.id;
		const formData = await request.formData();
		const attendeeId = formData.get('attendeeId') as string;

		if (!attendeeId) {
			return fail(400, {
				message: 'Attendee ID er påkrevd'
			});
		}

		// Find the attendee record to verify ownership
		const attendee = await locals.db.query.attendees.findFirst({
			where: and(
				eq(table.attendees.id, attendeeId),
				eq(table.attendees.eventId, eventId),
				eq(table.attendees.userId, locals.user.id)
			)
		});

		if (!attendee) {
			return fail(400, {
				message: 'Registrering ikke funnet eller du har ikke tilgang til å slette den'
			});
		}

		// Delete the image from R2 if it exists
		if (attendee.imageId) {
			try {
				await locals.bucket.delete(attendee.imageId);
			} catch (deleteError) {
				console.error('Error deleting image from R2:', deleteError);
				// Continue with database deletion even if image deletion fails
			}
		}

		// Delete the attendee record from database
		await locals.db
			.delete(table.attendees)
			.where(
				and(
					eq(table.attendees.id, attendeeId),
					eq(table.attendees.eventId, eventId),
					eq(table.attendees.userId, locals.user.id)
				)
			);

		return { success: true };
	}
};
