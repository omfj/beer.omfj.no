import { error, fail, redirect } from '@sveltejs/kit';
import * as table from '$lib/db/schema';
import { generateUserId } from '$lib/utils';
import type { PageServerLoad, Actions } from './$types';
import { randomUUID } from 'crypto';
import { createHttpUrl } from '$lib/ws';
import ky from 'ky';

/**
 * Helper function to create structured log context
 */
function logContext(operation: string, eventId?: string, userId?: string) {
	return {
		operation,
		eventId,
		userId,
		timestamp: new Date().toISOString()
	};
}

export const load: PageServerLoad = async ({ locals, params }) => {
	const id = params.id;
	const startTime = Date.now();

	console.log('Load registration page:', logContext('load_registration_page', id, locals.user?.id));

	if (!locals.user) {
		console.warn('Unauthorized access attempt:', logContext('load_unauthorized', id));
		throw error(401, 'Ikke logget inn');
	}

	try {
		// Fetch event with error handling
		const event = await locals.db.query.events.findFirst({
			where: (events, { eq }) => eq(events.id, id)
		});

		if (!event) {
			console.warn('Event not found:', logContext('event_not_found', id, locals.user.id));
			throw error(404, 'Arrangementet ble ikke funnet');
		}

		console.log('Event found:', logContext('event_found', id, locals.user.id));

		// Load drink types and sizes with their valid combinations
		const [drinkTypes, drinkSizes, drinkTypeSizes] = await Promise.all([
			locals.db.query.drinkTypes.findMany({
				orderBy: (drinkTypes, { asc }) => [asc(drinkTypes.name)]
			}),
			locals.db.query.drinkSizes.findMany({
				orderBy: (drinkSizes, { asc }) => [asc(drinkSizes.volumeML)]
			}),
			locals.db.query.drinkTypeSizes.findMany()
		]);

		const duration = Date.now() - startTime;
		console.log('Registration page loaded successfully:', {
			...logContext('load_success', id, locals.user.id),
			duration_ms: duration,
			drink_types_count: drinkTypes.length,
			drink_sizes_count: drinkSizes.length,
			valid_combinations_count: drinkTypeSizes.length
		});

		return {
			event,
			drinkTypes,
			drinkSizes,
			drinkTypeSizes
		};
	} catch (err) {
		// Re-throw SvelteKit errors (401, 404, etc.)
		if (err && typeof err === 'object' && 'status' in err) {
			throw err;
		}

		// Log and handle unexpected database errors
		console.error('Database error loading registration page:', {
			...logContext('load_db_error', id, locals.user?.id),
			error: err instanceof Error ? err.message : String(err),
			stack: err instanceof Error ? err.stack : undefined
		});
		throw error(500, 'Kunne ikke laste inn registreringssiden');
	}
};

export const actions: Actions = {
	default: async ({ request, locals, params, platform }) => {
		const eventId = params.id;
		const startTime = Date.now();
		const operationId = randomUUID();

		console.log('Starting drink registration:', {
			...logContext('register_start', eventId, locals.user?.id),
			operationId
		});

		if (!locals.user) {
			console.warn('Unauthorized registration attempt:', {
				...logContext('register_unauthorized', eventId),
				operationId
			});
			return fail(401, { message: 'Ikke logget inn' });
		}

		const waitUntil = platform?.ctx.waitUntil;

		let formData: FormData;
		try {
			formData = await request.formData();
		} catch (err) {
			console.error('Failed to parse form data:', {
				...logContext('form_parse_error', eventId, locals.user.id),
				operationId,
				error: err instanceof Error ? err.message : String(err)
			});
			return fail(400, { message: 'Ugyldig skjemadata' });
		}

		const file = formData.get('image') as File;
		const drinkTypeId = formData.get('drinkTypeId') as string;
		const drinkSizeId = formData.get('drinkSizeId') as string;

		console.log('Form data received:', {
			...logContext('form_received', eventId, locals.user.id),
			operationId,
			has_file: !!file,
			file_size: file?.size,
			file_type: file?.type,
			file_name: file?.name,
			drink_type_id: drinkTypeId,
			drink_size_id: drinkSizeId
		});

		// Validate file presence
		if (!file || file.size === 0) {
			console.warn('No file uploaded:', {
				...logContext('validation_no_file', eventId, locals.user.id),
				operationId
			});
			return fail(400, { message: 'Ingen fil ble lastet opp' });
		}

		// Validate file type
		if (!file.type.startsWith('image/')) {
			console.warn('Invalid file type:', {
				...logContext('validation_invalid_type', eventId, locals.user.id),
				operationId,
				file_type: file.type
			});
			return fail(400, { message: 'Kun bildefiler er tillatt' });
		}

		// Validate file size (max 10MB)
		const maxSize = 10 * 1024 * 1024; // 10MB
		if (file.size > maxSize) {
			console.warn('File too large:', {
				...logContext('validation_file_size', eventId, locals.user.id),
				operationId,
				file_size: file.size,
				max_size: maxSize
			});
			return fail(400, { message: 'Filen er for stor. Maksimal størrelse er 10MB' });
		}

		// Validate that if both are provided, the combination exists
		if (drinkTypeId && drinkSizeId) {
			try {
				const validCombination = await locals.db.query.drinkTypeSizes.findFirst({
					where: (drinkTypeSizes, { and, eq }) =>
						and(
							eq(drinkTypeSizes.drinkTypeId, drinkTypeId),
							eq(drinkTypeSizes.drinkSizeId, drinkSizeId)
						)
				});

				if (!validCombination) {
					console.warn('Invalid drink type/size combination:', {
						...logContext('validation_invalid_combination', eventId, locals.user.id),
						operationId,
						drink_type_id: drinkTypeId,
						drink_size_id: drinkSizeId
					});
					return fail(400, { message: 'Ugyldig kombinasjon av drikketype og størrelse' });
				}

				console.log('Valid drink combination:', {
					...logContext('validation_combination_ok', eventId, locals.user.id),
					operationId,
					drink_type_id: drinkTypeId,
					drink_size_id: drinkSizeId
				});
			} catch (err) {
				console.error('Database error validating drink combination:', {
					...logContext('db_validation_error', eventId, locals.user.id),
					operationId,
					error: err instanceof Error ? err.message : String(err),
					stack: err instanceof Error ? err.stack : undefined
				});
				return fail(500, { message: 'Feil ved validering av drikkeinformasjon' });
			}
		}

		// Generate unique filename
		const extension = file.name.split('.').pop() || 'jpg';
		const imageId = randomUUID();
		const fileName = `${imageId}.${extension}`;

		console.log('Generated filename:', {
			...logContext('filename_generated', eventId, locals.user.id),
			operationId,
			filename: fileName,
			original_name: file.name
		});

		// Upload to R2 bucket
		let arrayBuffer: ArrayBuffer;
		try {
			const conversionStart = Date.now();
			arrayBuffer = await file.arrayBuffer();
			console.log('File converted to array buffer:', {
				...logContext('file_converted', eventId, locals.user.id),
				operationId,
				duration_ms: Date.now() - conversionStart,
				buffer_size: arrayBuffer.byteLength
			});
		} catch (err) {
			console.error('Failed to convert file to array buffer:', {
				...logContext('file_conversion_error', eventId, locals.user.id),
				operationId,
				error: err instanceof Error ? err.message : String(err)
			});
			return fail(500, { message: 'Feil ved behandling av fil' });
		}

		try {
			const uploadStart = Date.now();
			const uploadPromise = locals.bucket.put(fileName, arrayBuffer, {
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

			// Add 30 second timeout for R2 upload
			const timeoutPromise = new Promise<never>((_, reject) =>
				setTimeout(() => reject(new Error('R2 upload timeout')), 30000)
			);

			await Promise.race([uploadPromise, timeoutPromise]);

			const uploadDuration = Date.now() - uploadStart;
			console.log('File uploaded to R2:', {
				...logContext('r2_upload_success', eventId, locals.user.id),
				operationId,
				filename: fileName,
				duration_ms: uploadDuration,
				file_size: arrayBuffer.byteLength
			});
		} catch (uploadError) {
			const isTimeout = uploadError instanceof Error && uploadError.message === 'R2 upload timeout';
			console.error('R2 upload failed:', {
				...logContext('r2_upload_error', eventId, locals.user.id),
				operationId,
				filename: fileName,
				is_timeout: isTimeout,
				error: uploadError instanceof Error ? uploadError.message : String(uploadError),
				stack: uploadError instanceof Error ? uploadError.stack : undefined
			});

			if (isTimeout) {
				return fail(500, { message: 'Opplasting tok for lang tid. Prøv igjen.' });
			}
			return fail(500, { message: 'Feil ved opplasting av fil' });
		}

		// Insert attendee record with image URL and drink info
		const attendeeId = generateUserId();
		try {
			const dbStart = Date.now();
			await locals.db.insert(table.attendees).values({
				id: attendeeId,
				eventId: eventId,
				userId: locals.user.id,
				drinkTypeId: drinkTypeId || null,
				drinkSizeId: drinkSizeId || null,
				imageId: fileName,
				createdAt: new Date()
			});

			const dbDuration = Date.now() - dbStart;
			console.log('Drink registration saved to database:', {
				...logContext('db_insert_success', eventId, locals.user.id),
				operationId,
				attendee_id: attendeeId,
				filename: fileName,
				duration_ms: dbDuration
			});
		} catch (dbError) {
			console.error('Database insert failed:', {
				...logContext('db_insert_error', eventId, locals.user.id),
				operationId,
				attendee_id: attendeeId,
				filename: fileName,
				error: dbError instanceof Error ? dbError.message : String(dbError),
				stack: dbError instanceof Error ? dbError.stack : undefined
			});

			// TODO: Consider implementing R2 cleanup here to remove orphaned image
			console.warn('Orphaned image in R2 (db insert failed):', {
				...logContext('orphaned_image', eventId, locals.user.id),
				operationId,
				filename: fileName
			});

			return fail(500, { message: 'Feil ved lagring av registrering' });
		}

		// Broadcast new registration via WebSocket (if applicable) - use waitUntil for background execution
		if (platform?.env.WS_HOST && platform?.env.API_KEY && waitUntil) {
			console.log('Broadcasting registration to WebSocket:', {
				...logContext('ws_broadcast_start', eventId, locals.user.id),
				operationId,
				ws_host: platform.env.WS_HOST
			});

			waitUntil(
				ky
					.post(createHttpUrl(platform.env.WS_HOST, eventId), {
						headers: {
							Authorization: `Bearer ${platform.env.API_KEY}`
						},
						timeout: 5000
					})
					.then(() => {
						console.log('WebSocket notification sent:', {
							...logContext('ws_broadcast_success', eventId, locals.user.id),
							operationId
						});
					})
					.catch((wsError) => {
						console.error('Failed to notify WebSocket server:', {
							...logContext('ws_broadcast_error', eventId, locals.user.id),
							operationId,
							error: wsError instanceof Error ? wsError.message : String(wsError)
						});
					})
			);
		} else {
			console.warn('WebSocket broadcast skipped:', {
				...logContext('ws_broadcast_skipped', eventId, locals.user.id),
				operationId,
				has_ws_host: !!platform?.env.WS_HOST,
				has_api_key: !!platform?.env.API_KEY,
				has_wait_until: !!waitUntil
			});
		}

		const totalDuration = Date.now() - startTime;
		console.log('Drink registration completed successfully:', {
			...logContext('register_success', eventId, locals.user.id),
			operationId,
			attendee_id: attendeeId,
			filename: fileName,
			total_duration_ms: totalDuration
		});

		// Redirect back to the event page
		redirect(303, `/arrangement/${eventId}`);
	}
};
