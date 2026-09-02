import { error, json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { generateRegistrationOptions } from '@simplewebauthn/server';
import { eq } from 'drizzle-orm';
import * as table from '$lib/db/schema';
import { getRelyingParty, parseTransports, setChallengeCookie } from '$lib/passkey';

export const POST: RequestHandler = async (event) => {
	const { locals } = event;

	if (!locals.user) {
		throw error(401, 'Unauthorized');
	}

	const existingPasskeys = await locals.db.query.passkeys.findMany({
		where: eq(table.passkeys.userId, locals.user.id),
		columns: { id: true, transports: true }
	});

	const { rpName, rpID } = getRelyingParty(event);

	const options = await generateRegistrationOptions({
		rpName,
		rpID,
		userName: locals.user.username,
		userID: new TextEncoder().encode(locals.user.id),
		attestationType: 'none',
		excludeCredentials: existingPasskeys.map((passkey) => ({
			id: passkey.id,
			transports: parseTransports(passkey.transports)
		})),
		authenticatorSelection: {
			// Login only uses conditional UI with empty allowCredentials,
			// which can just surface discoverable credentials — so a
			// non-discoverable one would register fine but never work.
			residentKey: 'required',
			userVerification: 'preferred'
		}
	});

	setChallengeCookie(event, options.challenge);

	return json(options);
};
