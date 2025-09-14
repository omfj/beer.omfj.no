import { fail, redirect } from '@sveltejs/kit';
import type { PageServerLoad, Actions } from './$types';
import * as table from '$lib/db/schema';
import { generateSoftColor } from '$lib/color';
import { nanoid } from 'nanoid';

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

		const [event] = await locals.db
			.insert(table.events)
			.values({
				id: nanoid(8),
				name,
				color: generateSoftColor(),
				createdAt: new Date(),
				createdBy: locals.user.id
			})
			.returning();

		if (!event) {
			return fail(500, { message: 'Noe gikk galt' });
		}

		throw redirect(302, `/arrangement/${event.id}`);
	}
};
