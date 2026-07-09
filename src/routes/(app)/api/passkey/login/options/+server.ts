import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { generateAuthenticationOptions } from '@simplewebauthn/server';
import { getRelyingParty, setChallengeCookie } from '$lib/passkey';

export const POST: RequestHandler = async (event) => {
	const { rpID } = getRelyingParty(event);

	// Empty allowCredentials lets the browser offer any discoverable
	// passkey for this RP, so login works without typing a username.
	const options = await generateAuthenticationOptions({
		rpID,
		userVerification: 'preferred',
		allowCredentials: []
	});

	setChallengeCookie(event, options.challenge);

	return json(options);
};
