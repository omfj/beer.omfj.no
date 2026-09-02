import { error, redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { hasEventAccess } from '$lib/event-access';

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

	const access = await hasEventAccess(locals.db, event, locals.user.id);
	if (!access) {
		throw redirect(302, `/arrangement/${id}/unlock`);
	}

	return { event };
};
