import { fail, redirect } from '@sveltejs/kit';
import type { PageServerLoad, Actions } from './$types';
import * as table from '$lib/db/schema';
import { generateSoftColor } from '$lib/color';

const generateEventId = () => {
	const letters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ';
	let id = '';
	for (let i = 0; i < 2; i++) {
		id += letters.charAt(Math.floor(Math.random() * letters.length));
	}
	for (let i = 0; i < 5; i++) {
		id += Math.floor(Math.random() * 10).toString();
	}
	return id;
};

export const load: PageServerLoad = async ({ locals }) => {
	if (!locals.user) {
		throw redirect(302, '/logg-inn');
	}
};

export const actions: Actions = {
	create: async ({ locals, request }) => {
		if (!locals.user) {
			throw redirect(302, '/logg-inn');
		}

		const formData = await request.formData();
		const name = formData.get('name') as string;

		if (!name) {
			return fail(400, { message: 'Navn er påkrevd' });
		}

		const maxRetries = 5;
		let attempts = 0;
		let event = null;

		while (attempts < maxRetries && !event) {
			const eventId = generateEventId();
			attempts++;

			try {
				const [insertedEvent] = await locals.db
					.insert(table.events)
					.values({
						id: eventId,
						name,
						color: generateSoftColor(),
						createdAt: new Date(),
						createdBy: locals.user.id
					})
					.returning();

				if (insertedEvent) {
					event = insertedEvent;
					break;
				}
			} catch (error) {
				// Check if it's a unique constraint violation (ID collision)
				if (
					error instanceof Error &&
					(error.message.includes('UNIQUE constraint') ||
						error.message.includes('unique constraint'))
				) {
					// ID collision, try again with a new ID
					if (attempts >= maxRetries) {
						return fail(500, {
							message: 'Kunne ikke generere unik event-ID. Prøv igjen.'
						});
					}
					continue;
				}
				// Some other error occurred
				console.error('Error creating event:', error);
				return fail(500, { message: 'Noe gikk galt ved opprettelse av arrangement' });
			}
		}

		if (!event) {
			return fail(500, { message: 'Kunne ikke opprette arrangement. Prøv igjen.' });
		}

		throw redirect(302, `/arrangement/${event.id}`);
	}
};
