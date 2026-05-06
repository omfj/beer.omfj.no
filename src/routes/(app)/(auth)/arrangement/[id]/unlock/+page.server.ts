import bcrypt from 'bcryptjs';
import { error, fail, redirect } from '@sveltejs/kit';
import type { PageServerLoad, Actions } from './$types';
import { hasEventAccess, grantEventAccess } from '$lib/event-access';

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

	if (!event.password) {
		throw redirect(302, `/arrangement/${id}`);
	}

	const alreadyHasAccess = await hasEventAccess(locals.db, event, locals.user.id);
	if (alreadyHasAccess) {
		throw redirect(302, `/arrangement/${id}`);
	}

	return { eventName: event.name };
};

export const actions: Actions = {
	unlock: async ({ locals, params, request }) => {
		const id = params.id;

		if (!locals.user) {
			return fail(401, { message: 'Ikke logget inn' });
		}

		const event = await locals.db.query.events.findFirst({
			where: (events, { eq }) => eq(events.id, id)
		});

		if (!event) {
			return fail(404, { message: 'Arrangementet ble ikke funnet' });
		}

		if (!event.password) {
			throw redirect(302, `/arrangement/${id}`);
		}

		const formData = await request.formData();
		const password = formData.get('password') as string;

		if (!password) {
			return fail(400, { message: 'Passord er påkrevd' });
		}

		const valid = await bcrypt.compare(password, event.password);
		if (!valid) {
			return fail(400, { message: 'Feil passord' });
		}

		await grantEventAccess(locals.db, id, locals.user.id);

		throw redirect(302, `/arrangement/${id}`);
	}
};
