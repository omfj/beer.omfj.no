import type { RequestEvent } from '@sveltejs/kit';
import { dev } from '$app/environment';
import type { AuthenticatorTransportFuture } from '@simplewebauthn/server';

export const passkeyChallengeCookieName = 'passkey-challenge';

// The challenge only needs to survive the round trip between the
// options and verify endpoints, but a conditional-UI login ceremony can
// stay pending for as long as the login page is open, so give the cookie
// some slack.
const CHALLENGE_TTL_SECONDS = 60 * 60;

// The rpID comes from the PASSKEY_RP_ID var: wrangler.jsonc in prod
// (beer.omfj.no), .dev.vars locally (localhost). It must match the domain
// the page is served from. The origin still comes from the request URL so
// a dev port is included.
export function getRelyingParty(event: RequestEvent) {
	return {
		rpName: 'Beer Counter',
		rpID: event.platform!.env.PASSKEY_RP_ID,
		origin: event.url.origin
	};
}

export function setChallengeCookie(event: RequestEvent, challenge: string) {
	event.cookies.set(passkeyChallengeCookieName, challenge, {
		path: '/',
		httpOnly: true,
		secure: !dev,
		sameSite: dev ? 'lax' : 'strict',
		maxAge: CHALLENGE_TTL_SECONDS
	});
}

export function consumeChallengeCookie(event: RequestEvent) {
	const challenge = event.cookies.get(passkeyChallengeCookieName);
	event.cookies.delete(passkeyChallengeCookieName, { path: '/' });
	return challenge ?? null;
}

export function parseTransports(transports: string | null): AuthenticatorTransportFuture[] {
	if (!transports) return [];
	try {
		return JSON.parse(transports);
	} catch {
		return [];
	}
}
