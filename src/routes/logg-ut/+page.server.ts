import { fail, redirect } from '@sveltejs/kit';
import type { Actions } from './$types';

export const actions: Actions = {
	default: async (event) => {
		const { locals } = event;

		if (!locals.session) {
			return fail(401);
		}
		await locals.sessionService.invalidateSession(locals.session.id);
		locals.sessionService.deleteSessionTokenCookie(event);

		return redirect(302, '/logg-inn');
	}
};
