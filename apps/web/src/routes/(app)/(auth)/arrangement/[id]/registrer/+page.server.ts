import { error, redirect } from '@sveltejs/kit';
import * as table from '$lib/db/schema';
import { generateUserId } from '$lib/utils';
import type { PageServerLoad, Actions } from './$types';
import { randomUUID } from 'crypto';
import { createHttpUrl } from '$lib/ws';
import ky from 'ky';

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

	// Load drink types and sizes with their valid combinations
	const drinkTypes = await locals.db.query.drinkTypes.findMany({
		orderBy: (drinkTypes, { asc }) => [asc(drinkTypes.name)]
	});

	const drinkSizes = await locals.db.query.drinkSizes.findMany({
		orderBy: (drinkSizes, { asc }) => [asc(drinkSizes.volumeML)]
	});

	const drinkTypeSizes = await locals.db.query.drinkTypeSizes.findMany();

	return {
		event,
		drinkTypes,
		drinkSizes,
		drinkTypeSizes
	};
};

export const actions: Actions = {
	default: async ({ request, locals, params, platform }) => {
		if (!locals.user) {
			throw error(401, 'Ikke logget inn');
		}

		const eventId = params.id;
		const formData = await request.formData();

		const file = formData.get('image') as File;
		const drinkTypeId = formData.get('drinkTypeId') as string;
		const drinkSizeId = formData.get('drinkSizeId') as string;

		if (!file || file.size === 0) {
			throw error(400, 'Ingen fil ble lastet opp');
		}

		// Validate that if both are provided, the combination exists
		if (drinkTypeId && drinkSizeId) {
			const validCombination = await locals.db.query.drinkTypeSizes.findFirst({
				where: (drinkTypeSizes, { and, eq }) =>
					and(
						eq(drinkTypeSizes.drinkTypeId, drinkTypeId),
						eq(drinkTypeSizes.drinkSizeId, drinkSizeId)
					)
			});

			if (!validCombination) {
				throw error(400, 'Ugyldig kombinasjon av drikketype og størrelse');
			}
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
			const extension = file.name.split('.').pop() || 'jpg';
			const imageId = randomUUID();
			const fileName = `${imageId}.${extension}`;

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

			// Insert attendee record with image URL and drink info
			const attendeeId = generateUserId();
			await locals.db.insert(table.attendees).values({
				id: attendeeId,
				eventId: eventId,
				userId: locals.user.id,
				drinkTypeId: drinkTypeId || null,
				drinkSizeId: drinkSizeId || null,
				imageId: fileName,
				createdAt: new Date()
			});

			console.log(
				`Beer registered successfully: ${fileName} for user ${locals.user.id} in event ${eventId}`
			);
		} catch (uploadError) {
			console.error('Error uploading file:', uploadError);
			error(500, 'Feil ved opplasting av fil');
		}

		// Broadcast new registration via WebSocket (if applicable)
		if (platform?.env.WS_HOST && platform?.env.API_KEY) {
			await ky
				.post(createHttpUrl(platform.env.WS_HOST, eventId), {
					headers: {
						Authorization: `Bearer ${platform.env.API_KEY}`
					},
					timeout: 5000
				})
				.catch(() => {
					console.warn('Failed to notify WebSocket server');
				});
		}

		// Redirect back to the event page
		throw redirect(303, `/arrangement/${eventId}`);
	}
};
