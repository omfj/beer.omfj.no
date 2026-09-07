import { ApiClient } from '.';

export const api = new ApiClient({
	baseUrl: import.meta.env.PUBLIC_API_BASE_URL || '/api'
});
