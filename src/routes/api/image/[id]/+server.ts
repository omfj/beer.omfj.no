import { error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

export const GET: RequestHandler = async ({ params, locals, setHeaders }) => {
	const imageId = params.id;

	if (!imageId) {
		throw error(400, 'Missing image ID');
	}

	try {
		// Get the image from R2 bucket
		const object = await locals.bucket.get(imageId);

		if (!object) {
			throw error(404, 'Image not found');
		}

		// Get the image data as array buffer
		const imageBuffer = await object.arrayBuffer();
		
		// Get content type from metadata or default to image/jpeg
		const contentType = object.httpMetadata?.contentType || 'image/jpeg';

		// Set cache headers for optimal performance
		setHeaders({
			'Content-Type': contentType,
			'Cache-Control': 'public, max-age=31536000, immutable', // Cache for 1 year
			'ETag': object.etag || '',
			'Last-Modified': object.uploaded?.toUTCString() || new Date().toUTCString(),
			'Content-Length': imageBuffer.byteLength.toString(),
		});

		// Return the image as a Response
		return new Response(imageBuffer, {
			status: 200,
			headers: {
				'Content-Type': contentType,
				'Cache-Control': 'public, max-age=31536000, immutable',
				'ETag': object.etag || '',
				'Last-Modified': object.uploaded?.toUTCString() || new Date().toUTCString(),
			}
		});

	} catch (fetchError) {
		console.error('Error fetching image from bucket:', fetchError);
		throw error(500, 'Failed to retrieve image');
	}
};