import { ApiClient } from '.';

const baseUrl = import.meta.env.VITE_API_BASE_URL || '/api';

export const api = new ApiClient({ baseUrl });

export function createApiClient(fetch: typeof globalThis.fetch): ApiClient {
	return new ApiClient({ baseUrl, fetch });
}
