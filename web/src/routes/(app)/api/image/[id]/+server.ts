import { error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { eq } from 'drizzle-orm';
import * as table from '$lib/db/schema';
import { hasEventAccess } from '$lib/event-access';

export const GET: RequestHandler = async ({ params, locals, setHeaders }) => {
	if (!locals.user) {
		throw error(401, 'Unauthorized');
	}

	const imageId = params.id;

	if (!imageId) {
		throw error(400, 'Missing image ID');
	}

	// Check the user has access to the event this image belongs to
	const attendee = await locals.db.query.attendees.findFirst({
		where: eq(table.attendees.imageId, imageId),
		with: { event: true }
	});

	if (attendee?.event) {
		const access = await hasEventAccess(locals.db, attendee.event, locals.user.id);
		if (!access) {
			throw error(403, 'Forbidden');
		}
	}

	try {
		const object = await locals.bucket.get(imageId);

		if (!object) {
			throw error(404, 'Image not found');
		}

		const imageBuffer = await object.arrayBuffer();
		const contentType = object.httpMetadata?.contentType || 'image/jpeg';

		setHeaders({
			'Content-Type': contentType,
			'Cache-Control': 'private, max-age=31536000, immutable',
			ETag: object.etag || '',
			'Last-Modified': object.uploaded?.toUTCString() || new Date().toUTCString(),
			'Content-Length': imageBuffer.byteLength.toString()
		});

		return new Response(imageBuffer, {
			status: 200,
			headers: {
				'Content-Type': contentType,
				'Cache-Control': 'private, max-age=31536000, immutable',
				ETag: object.etag || '',
				'Last-Modified': object.uploaded?.toUTCString() || new Date().toUTCString()
			}
		});
	} catch (fetchError) {
		console.error('Error fetching image from bucket:', fetchError);
		throw error(500, 'Failed to retrieve image');
	}
};
