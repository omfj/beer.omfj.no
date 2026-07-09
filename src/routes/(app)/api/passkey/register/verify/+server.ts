import { error, json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { verifyRegistrationResponse, type RegistrationResponseJSON } from '@simplewebauthn/server';
import { encodeBase64url } from '@oslojs/encoding';
import * as table from '$lib/db/schema';
import { consumeChallengeCookie, getRelyingParty } from '$lib/passkey';

export const POST: RequestHandler = async (event) => {
	const { locals } = event;

	if (!locals.user) {
		throw error(401, 'Unauthorized');
	}

	const expectedChallenge = consumeChallengeCookie(event);
	if (!expectedChallenge) {
		throw error(400, 'Utfordringen er utløpt, prøv igjen');
	}

	const response = (await event.request
		.json()
		.catch(() => null)) as RegistrationResponseJSON | null;
	if (!response) {
		throw error(400, 'Ugyldig forespørsel');
	}

	const { rpID, origin } = getRelyingParty(event);

	let verification;
	try {
		verification = await verifyRegistrationResponse({
			response,
			expectedChallenge,
			expectedOrigin: origin,
			expectedRPID: rpID
		});
	} catch (e) {
		console.error('Passkey registration verification failed:', e);
		throw error(400, 'Kunne ikke verifisere passkey');
	}

	if (!verification.verified || !verification.registrationInfo) {
		throw error(400, 'Kunne ikke verifisere passkey');
	}

	const { credential } = verification.registrationInfo;

	const inserted = await locals.db
		.insert(table.passkeys)
		.values({
			id: credential.id,
			userId: locals.user.id,
			publicKey: encodeBase64url(credential.publicKey),
			counter: credential.counter,
			transports: credential.transports ? JSON.stringify(credential.transports) : null
		})
		.onConflictDoNothing()
		.returning({ id: table.passkeys.id });

	if (inserted.length === 0) {
		throw error(400, 'Passkeyen er allerede registrert');
	}

	return json({ success: true });
};
