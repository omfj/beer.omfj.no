import type { User } from '$lib/api';
import { getContext, setContext } from 'svelte';

export type AuthState =
	| { status: 'loading'; user: null }
	| { status: 'anonymous'; user: null }
	| { status: 'authenticated'; user: User };

export class UserContext {
	state = $state<AuthState>({ status: 'loading', user: null });

	setUser(user: User) {
		this.state = { status: 'authenticated', user };
	}

	clear() {
		this.state = { status: 'anonymous', user: null };
	}
}

const AUTH_CONTEXT_KEY = '__auth';

export function createUserContext() {
	return new UserContext();
}

export function setUserContext(value: UserContext) {
	return setContext<UserContext>(AUTH_CONTEXT_KEY, value);
}

export function getUser() {
	return getContext<UserContext>(AUTH_CONTEXT_KEY);
}
