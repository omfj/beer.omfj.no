import type { Handle } from '@sveltejs/kit';
import { createDatabase } from '$lib/db';
import { sessionCookieName, SessionService } from '$lib/auth';

const handleAuth: Handle = async ({ event, resolve }) => {
	const db = createDatabase(event.platform!.env.DB);
	event.locals.db = db;

	const sessionService = new SessionService(db);
	event.locals.sessionService = sessionService;

	const sessionToken = event.cookies.get(sessionCookieName);
	if (!sessionToken) {
		event.locals.user = null;
		event.locals.session = null;
		return resolve(event);
	}

	const { session, user } = await sessionService.validateSessionToken(sessionToken);
	if (session) {
		sessionService.setSessionTokenCookie(event, sessionToken, session.expiresAt);
	} else {
		sessionService.deleteSessionTokenCookie(event);
	}

	event.locals.user = user;
	event.locals.session = session;

	return resolve(event);
};

export const handle: Handle = handleAuth;
