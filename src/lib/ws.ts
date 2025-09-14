import { dev } from '$app/environment';

export function createWsUrl(url: string, roomId: string) {
	const protocol = dev ? 'ws' : 'wss';
	return `${protocol}://${url}/event/${roomId}`;
}

export const createHttpUrl = (url: string, roomId: string) => {
	const protocol = dev ? 'http' : 'https';
	return `${protocol}://${url}/event/${roomId}`;
};
