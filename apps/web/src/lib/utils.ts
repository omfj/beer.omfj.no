import { encodeBase32LowerCase } from '@oslojs/encoding';

export function generateUserId() {
	// ID with 120 bits of entropy, or about the same as UUID v4.
	const bytes = crypto.getRandomValues(new Uint8Array(15));
	return encodeBase32LowerCase(bytes);
}

// Username must be a string, alphanumeric, and between 3 and 255 characters long.
export function validateUsername(username: unknown): username is string {
	return (
		typeof username === 'string' &&
		/^[a-zA-Z0-9]+$/.test(username) &&
		username.length >= 3 &&
		username.length <= 255
	);
}

export function validatePassword(password: unknown): password is string {
	return typeof password === 'string' && password.length >= 3 && password.length <= 255;
}
