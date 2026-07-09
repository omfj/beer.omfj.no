import { error, json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import {
	verifyAuthenticationResponse,
	type AuthenticationResponseJSON
} from '@simplewebauthn/server';
import { decodeBase64url } from '@oslojs/encoding';
import { eq } from 'drizzle-orm';
import * as table from '$lib/db/schema';
import { consumeChallengeCookie, getRelyingParty, parseTransports } from '$lib/passkey';

export const POST: RequestHandler = async (event) => {
	const { locals } = event;

	const expectedChallenge = consumeChallengeCookie(event);
	if (!expectedChallenge) {
		throw error(400, 'Utfordringen er utløpt, prøv igjen');
	}

	const response = (await event.request
		.json()
		.catch(() => null)) as AuthenticationResponseJSON | null;
	if (typeof response?.id !== 'string') {
		throw error(400, 'Ugyldig forespørsel');
	}

	const passkey = await locals.db.query.passkeys.findFirst({
		where: eq(table.passkeys.id, response.id)
	});

	if (!passkey) {
		throw error(400, 'Ukjent passkey');
	}

	const { rpID, origin } = getRelyingParty(event);

	let verification;
	try {
		verification = await verifyAuthenticationResponse({
			response,
			expectedChallenge,
			expectedOrigin: origin,
			expectedRPID: rpID,
			credential: {
				id: passkey.id,
				publicKey: new Uint8Array(decodeBase64url(passkey.publicKey)),
				counter: passkey.counter,
				transports: parseTransports(passkey.transports)
			}
		});
	} catch (e) {
		console.error('Passkey authentication verification failed:', e);
		throw error(400, 'Kunne ikke verifisere passkey');
	}

	if (!verification.verified) {
		throw error(400, 'Kunne ikke verifisere passkey');
	}

	await locals.db
		.update(table.passkeys)
		.set({ counter: verification.authenticationInfo.newCounter })
		.where(eq(table.passkeys.id, passkey.id));

	await locals.sessionService.startSession(event, passkey.userId);

	return json({ success: true });
};
