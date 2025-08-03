import bcrypt from 'bcryptjs';
import { fail, redirect } from '@sveltejs/kit';
import * as table from '$lib/db/schema';
import type { Actions, PageServerLoad } from './$types';
import { validateUsername, validatePassword, generateUserId } from '$lib/utils';

export const load: PageServerLoad = async (event) => {
	if (event.locals.user) {
		return redirect(302, '/');
	}
};

export const actions: Actions = {
	register: async (event) => {
		const { db, sessionService } = event.locals;

		const formData = await event.request.formData();
		const username = formData.get('username');
		const password = formData.get('password');
		const termsAccepted = formData.get('terms') === 'on';

		if (!validateUsername(username)) {
			return fail(400, { message: 'Ugyldig brukernavn' });
		}
		if (!validatePassword(password)) {
			return fail(400, { message: 'Ugyldig passord' });
		}

		if (!termsAccepted) {
			return fail(400, { message: 'Du må godta vilkårene' });
		}

		const userId = generateUserId();
		const passwordHash = await bcrypt.hash(password, 12);

		try {
			await db.insert(table.users).values({ id: userId, username, hasAgreedToTerms: true });
			await db.insert(table.userPasswords).values({
				userId,
				passwordHash
			});

			const sessionToken = sessionService.generateSessionToken();
			const session = await sessionService.createSession(sessionToken, userId);
			sessionService.setSessionTokenCookie(event, sessionToken, session.expiresAt);
		} catch (e) {
			console.log('Error:', e);

			if (e instanceof Error && e.message.includes('UNIQUE constraint')) {
				return fail(400, { message: 'Brukernavnet er allerede tatt' });
			}

			return fail(500, { message: 'En feil har oppstått' });
		}

		return redirect(302, '/');
	}
};
