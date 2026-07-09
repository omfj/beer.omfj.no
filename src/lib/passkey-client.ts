import {
	startRegistration,
	startAuthentication,
	browserSupportsWebAuthnAutofill,
	WebAuthnError,
	type PublicKeyCredentialCreationOptionsJSON,
	type PublicKeyCredentialRequestOptionsJSON
} from '@simplewebauthn/browser';

export { browserSupportsWebAuthnAutofill };

export const genericPasskeyError = 'Noe gikk galt, prøv igjen';

async function post<T>(url: string, body?: unknown): Promise<T> {
	const response = await fetch(url, {
		method: 'POST',
		headers: body ? { 'Content-Type': 'application/json' } : undefined,
		body: body ? JSON.stringify(body) : undefined
	});

	if (!response.ok) {
		const data = (await response.json().catch(() => null)) as { message?: string } | null;
		throw new Error(data?.message ?? genericPasskeyError);
	}

	return response.json() as Promise<T>;
}

function isCancellation(e: unknown): boolean {
	// @simplewebauthn wraps a dismissed ceremony (NotAllowedError) as
	// ERROR_PASSTHROUGH_SEE_CAUSE_PROPERTY with the original error as cause;
	// ERROR_CEREMONY_ABORTED is only used for abort-signal cancellations.
	if (e instanceof WebAuthnError) {
		return (
			e.code === 'ERROR_CEREMONY_ABORTED' ||
			(e.cause instanceof DOMException && e.cause.name === 'NotAllowedError')
		);
	}
	return e instanceof DOMException && e.name === 'NotAllowedError';
}

function toFriendlyError(e: unknown): Error {
	if (isCancellation(e)) {
		return new PasskeyCancelledError();
	}
	if (e instanceof Error) {
		return e;
	}
	return new Error(genericPasskeyError);
}

export class PasskeyCancelledError extends Error {
	constructor() {
		super('Passkey-handlingen ble avbrutt');
	}
}

export async function registerPasskey() {
	const optionsJSON = await post<PublicKeyCredentialCreationOptionsJSON>(
		'/api/passkey/register/options'
	);

	let registrationResponse;
	try {
		registrationResponse = await startRegistration({ optionsJSON });
	} catch (e) {
		throw toFriendlyError(e);
	}

	await post('/api/passkey/register/verify', registrationResponse);
}

/**
 * With `useBrowserAutofill` the promise stays pending until the user picks a
 * passkey from the browser/password manager autofill dropdown (WebAuthn
 * conditional UI).
 */
export async function loginWithPasskey(useBrowserAutofill = false) {
	const optionsJSON = await post<PublicKeyCredentialRequestOptionsJSON>(
		'/api/passkey/login/options'
	);

	let authenticationResponse;
	try {
		authenticationResponse = await startAuthentication({ optionsJSON, useBrowserAutofill });
	} catch (e) {
		throw toFriendlyError(e);
	}

	await post('/api/passkey/login/verify', authenticationResponse);
}
