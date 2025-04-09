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
	login: async (event) => {
		const { db, sessionService } = event.locals;

		const formData = await event.request.formData();
		const username = formData.get('username');
		const password = formData.get('password');

		if (!validateUsername(username)) {
			return fail(400, {
				message: 'Ugyldig brukernavn (min 3, max 31 karakterer, bare bokstaver)'
			});
		}
		if (!validatePassword(password)) {
			return fail(400, { message: 'Ugyldig passord (min 6, max 255 karakterer)' });
		}

		const user = await db.query.users.findFirst({
			where: (row, { eq }) => eq(row.username, username),
			with: {
				password: true
			}
		});

		if (!user) {
			return fail(400, { message: 'Ugyldig brukernavn eller passord' });
		}

		const validPassword = await bcrypt.compare(password, user.password.passwordHash);

		if (!validPassword) {
			return fail(400, { message: 'Ugyldig brukernavn eller passord' });
		}

		const sessionToken = sessionService.generateSessionToken();
		const session = await sessionService.createSession(sessionToken, user.id);
		sessionService.setSessionTokenCookie(event, sessionToken, session.expiresAt);

		return redirect(302, '/');
	},
	register: async (event) => {
		const { db, sessionService } = event.locals;

		const formData = await event.request.formData();
		const username = formData.get('username');
		const password = formData.get('password');

		if (!validateUsername(username)) {
			return fail(400, { message: 'Ugyldig brukernavn' });
		}
		if (!validatePassword(password)) {
			return fail(400, { message: 'Ugyldig passord' });
		}

		const userId = generateUserId();
		const passwordHash = await bcrypt.hash(password, 12);

		try {
			await db.insert(table.users).values({ id: userId, username });
			await db.insert(table.userPasswords).values({
				userId,
				passwordHash
			});

			const sessionToken = sessionService.generateSessionToken();
			const session = await sessionService.createSession(sessionToken, userId);
			sessionService.setSessionTokenCookie(event, sessionToken, session.expiresAt);
		} catch {
			return fail(500, { message: 'En feil har oppstått' });
		}
		return redirect(302, '/');
	}
};
