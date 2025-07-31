import { error, redirect } from '@sveltejs/kit';
import * as table from '$lib/db/schema';
import { generateUserId } from '$lib/utils';
import type { PageServerLoad, Actions } from './$types';

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

	return { event };
};

export const actions: Actions = {
	default: async ({ request, locals, params }) => {
		if (!locals.user) {
			throw error(401, 'Ikke logget inn');
		}

		const eventId = params.id;
		const formData = await request.formData();

		const file = formData.get('image') as File;

		if (!file || file.size === 0) {
			throw error(400, 'Ingen fil ble lastet opp');
		}

		// Validate file type
		if (!file.type.startsWith('image/')) {
			throw error(400, 'Kun bildefiler er tillatt');
		}

		// Validate file size (max 10MB)
		const maxSize = 10 * 1024 * 1024; // 10MB
		if (file.size > maxSize) {
			throw error(400, 'Filen er for stor. Maksimal størrelse er 10MB');
		}

		try {
			// Generate unique filename
			const timestamp = Date.now();
			const extension = file.name.split('.').pop() || 'jpg';
			const fileName = `beer-${eventId}-${locals.user.id}-${timestamp}.${extension}`;

			// Upload to R2 bucket
			const arrayBuffer = await file.arrayBuffer();
			await locals.bucket.put(fileName, arrayBuffer, {
				httpMetadata: {
					contentType: file.type
				},
				customMetadata: {
					userId: locals.user.id,
					eventId: eventId,
					originalName: file.name,
					uploadedAt: new Date().toISOString()
				}
			});

			// Insert attendee record with image URL
			const attendeeId = generateUserId();
			await locals.db.insert(table.attendees).values({
				id: attendeeId,
				eventId: eventId,
				userId: locals.user.id,
				imageId: fileName,
				createdAt: new Date()
			});

			console.log(
				`Beer registered successfully: ${fileName} for user ${locals.user.id} in event ${eventId}`
			);
		} catch (uploadError) {
			console.error('Error uploading file:', uploadError);
			throw error(500, 'Feil ved opplasting av fil');
		}

		// Redirect back to the event page
		throw redirect(303, `/arrangement/${eventId}`);
	}
};
