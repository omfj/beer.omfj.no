import type {
	LoginRequest,
	RegisterRequest,
	UpdateProfileRequest,
	UserResponse,
	CreateEventRequest,
	UnlockEventRequest,
	Events,
	CreatedEvent,
	EventDetail,
	DrinkOptions,
	CreateDrinkRequest,
	CreatedDrink,
	Leaderboard,
	Health
} from './types';

export type * from './types';

export interface ApiClientOptions {
	/** Backend origin, optionally including a path prefix. */
	baseUrl: string;
	/** Override fetch for tests or alternate runtimes. */
	fetch?: typeof globalThis.fetch;
}

export interface RequestOptions {
	signal?: AbortSignal;
}

export class ApiError extends Error {
	constructor(
		public readonly status: number,
		public readonly code: string,
		message: string
	) {
		super(message);
		this.name = 'ApiError';
	}
}

export class ApiClient {
	private readonly baseUrl: string;
	private readonly fetcher: typeof globalThis.fetch;

	constructor(options: ApiClientOptions) {
		this.baseUrl = options.baseUrl.replace(/\/+$/, '');
		this.fetcher = options.fetch ?? globalThis.fetch.bind(globalThis);
	}

	async login(input: LoginRequest, options?: RequestOptions): Promise<UserResponse> {
		return this.json('/auth/login', input, options);
	}

	async register(input: RegisterRequest, options?: RequestOptions): Promise<UserResponse> {
		return this.json('/auth/register', input, options);
	}

	async logout(options?: RequestOptions): Promise<void> {
		await this.request('/auth/logout', { method: 'POST', ...options });
	}

	async me(options?: RequestOptions): Promise<UserResponse> {
		return this.get('/auth/me', options);
	}

	async updateMe(input: UpdateProfileRequest, options?: RequestOptions): Promise<UserResponse> {
		const response = await this.request('/auth/me', {
			method: 'PATCH',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify(input),
			...options
		});
		return response.json();
	}

	async listEvents(options?: RequestOptions): Promise<Events> {
		return this.get('/events', options);
	}

	async createEvent(input: CreateEventRequest, options?: RequestOptions): Promise<CreatedEvent> {
		return this.json('/events', input, options);
	}

	async getEvent(id: string, options?: RequestOptions): Promise<EventDetail> {
		return this.get(`/event/${encodeURIComponent(id)}`, options);
	}

	async unlockEvent(
		id: string,
		input: UnlockEventRequest,
		options?: RequestOptions
	): Promise<void> {
		await this.request(`/event/${encodeURIComponent(id)}/unlock`, {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify(input),
			...options
		});
	}

	async getDrinkOptions(options?: RequestOptions): Promise<DrinkOptions> {
		return this.get('/drinks/options', options);
	}

	async createDrink(
		id: string,
		input: CreateDrinkRequest,
		options?: RequestOptions
	): Promise<CreatedDrink> {
		const body = new FormData();
		body.set('image', input.image);
		if (input.drinkTypeId != null) body.set('drinkTypeId', input.drinkTypeId);
		if (input.drinkSizeId != null) body.set('drinkSizeId', input.drinkSizeId);
		if (input.abv != null) body.set('abv', String(input.abv));
		const response = await this.request(`/event/${encodeURIComponent(id)}/drinks`, {
			method: 'POST',
			body,
			...options
		});
		return response.json();
	}

	async deleteDrink(eventId: string, drinkId: string, options?: RequestOptions): Promise<void> {
		await this.request(
			`/event/${encodeURIComponent(eventId)}/drinks/${encodeURIComponent(drinkId)}`,
			{
				method: 'DELETE',
				...options
			}
		);
	}

	async getLeaderboard(year: number, options?: RequestOptions): Promise<Leaderboard> {
		return this.get(`/leaderboard?${new URLSearchParams({ year: String(year) })}`, options);
	}

	async getHealth(options?: RequestOptions): Promise<Health> {
		return this.get('/health', options);
	}

	imageUrl(id: string): string {
		return `${this.baseUrl}/images/${encodeURIComponent(id)}`;
	}

	private async get<T>(path: string, options?: RequestOptions): Promise<T> {
		const response = await this.request(path, { method: 'GET', ...options });
		return response.json();
	}

	private async json<T>(path: string, input: unknown, options?: RequestOptions): Promise<T> {
		const response = await this.request(path, {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify(input),
			...options
		});
		return response.json();
	}

	private async request(path: string, init: RequestInit): Promise<Response> {
		const headers = new Headers(init.headers);
		headers.set('Accept', 'application/json');
		const response = await this.fetcher(`${this.baseUrl}${path}`, {
			...init,
			headers,
			credentials: 'include'
		});
		if (!response.ok) {
			// Axum extractors and proxies may return non-JSON error responses.
			const body: unknown = await response.json().catch(() => null);
			let code = 'http_error';
			let message = response.statusText || `HTTP ${response.status}`;
			if (body && typeof body === 'object' && 'error' in body) {
				const error = body.error;
				if (error && typeof error === 'object') {
					if ('code' in error && typeof error.code === 'string') code = error.code;
					if ('message' in error && typeof error.message === 'string') message = error.message;
				}
			}
			throw new ApiError(response.status, code, message);
		}
		return response;
	}
}
