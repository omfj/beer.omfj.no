import type { RequestEvent } from '@sveltejs/kit';
import { eq } from 'drizzle-orm';
import { sha256 } from '@oslojs/crypto/sha2';
import { encodeBase64url, encodeHexLowerCase } from '@oslojs/encoding';
import * as table from '$lib/db/schema';
import type { Database } from './db';

const DAY_IN_MS = 1000 * 60 * 60 * 24;

export const sessionCookieName = 'auth-session';

export type SessionValidationResult = Awaited<
	ReturnType<typeof SessionService.prototype.validateSessionToken>
>;

export class SessionService {
	#db: Database;

	constructor(db: Database) {
		this.#db = db;
	}

	generateSessionToken() {
		const bytes = crypto.getRandomValues(new Uint8Array(18));
		const token = encodeBase64url(bytes);
		return token;
	}

	async createSession(token: string, userId: string) {
		const sessionId = encodeHexLowerCase(sha256(new TextEncoder().encode(token)));
		const session: table.Session = {
			id: sessionId,
			userId,
			expiresAt: new Date(Date.now() + DAY_IN_MS * 30)
		};
		await this.#db.insert(table.sessions).values(session);
		return session;
	}

	async validateSessionToken(token: string) {
		const sessionId = encodeHexLowerCase(sha256(new TextEncoder().encode(token)));
		const [result] = await this.#db
			.select({
				// Adjust user table here to tweak returned data
				user: {
					id: table.users.id,
					username: table.users.username,
					hasAgreedToTerms: table.users.hasAgreedToTerms
				},
				session: table.sessions
			})
			.from(table.sessions)
			.innerJoin(table.users, eq(table.sessions.userId, table.users.id))
			.where(eq(table.sessions.id, sessionId));

		if (!result) {
			return { session: null, user: null };
		}
		const { session, user } = result;

		const sessionExpired = Date.now() >= session.expiresAt.getTime();
		if (sessionExpired) {
			await this.#db.delete(table.sessions).where(eq(table.sessions.id, session.id));
			return { session: null, user: null };
		}

		const renewSession = Date.now() >= session.expiresAt.getTime() - DAY_IN_MS * 15;
		if (renewSession) {
			session.expiresAt = new Date(Date.now() + DAY_IN_MS * 30);
			await this.#db
				.update(table.sessions)
				.set({ expiresAt: session.expiresAt })
				.where(eq(table.sessions.id, session.id));
		}

		return { session, user };
	}

	async invalidateSession(sessionId: string) {
		await this.#db.delete(table.sessions).where(eq(table.sessions.id, sessionId));
	}

	setSessionTokenCookie(event: RequestEvent, token: string, expiresAt: Date) {
		event.cookies.set(sessionCookieName, token, {
			expires: expiresAt,
			path: '/'
		});
	}

	deleteSessionTokenCookie(event: RequestEvent) {
		event.cookies.delete(sessionCookieName, {
			path: '/'
		});
	}
}
