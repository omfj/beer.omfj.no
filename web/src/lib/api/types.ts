type Weight = 'light' | 'medium' | 'heavy';

type Gender = 'male' | 'female' | 'other';

export interface User {
	id: string;
	username: string;
	hasAgreedToTerms: boolean;
	weight: Weight | null;
	gender: Gender | null;
	createdAt: number | null;
}

export interface LoginRequest {
	username: string;
	password: string;
}

export interface RegisterRequest extends LoginRequest {
	termsAccepted: boolean;
}

export interface UserResponse {
	user: User;
}

export interface UpdateProfileRequest {
	weight: Weight | null;
	gender: Gender | null;
}

export interface CreateEventRequest {
	name: string;
	password?: string | null;
}

export interface UnlockEventRequest {
	password: string;
}

export interface EventSummary {
	id: string;
	name: string;
	totalAttendees: number;
	distinctUsers: number;
}

export interface Events {
	events: EventSummary[];
}

export interface Event {
	id: string;
	name: string;
	color: string;
	createdAt: number;
	createdBy: string | null;
}

export interface CreatedEvent {
	event: Event;
}

export interface DrinkType {
	id: string;
	name: string;
	description: string | null;
	abv: number | null;
	multiplier: number;
}

export interface DrinkSize {
	id: string;
	name: string;
	volumeML: number;
	description: string | null;
}

export interface Attendee {
	id: string;
	userId: string;
	username: string;
	createdAt: number;
	imageId: string | null;
	abv: number | null;
	drinkType: DrinkType | null;
	drinkSize: DrinkSize | null;
	userWeight: string | null;
	userGender: string | null;
}

export interface EventUser {
	id: string;
	username: string;
	weight: string | null;
	gender: string | null;
}

export interface EventDetail {
	event: Event;
	attendees: Attendee[];
	accessUsers: EventUser[];
}

export interface DrinkTypeSize {
	id: string;
	drinkTypeId: string;
	drinkSizeId: string;
}

export interface DrinkOptions {
	drinkTypes: DrinkType[];
	drinkSizes: DrinkSize[];
	drinkTypeSizes: DrinkTypeSize[];
}

export interface CreateDrinkRequest {
	image: File;
	drinkTypeId?: string | null;
	drinkSizeId?: string | null;
	abv?: number | null;
}

export interface CreatedDrink {
	id: string;
	eventId: string;
	userId: string;
	imageId: string;
	createdAt: number;
	drinkTypeId: string | null;
	drinkSizeId: string | null;
	abv: number | null;
}

export interface LeaderboardEntry {
	userId: string;
	username: string;
	points: number;
	drinkCount: number;
}

export interface Leaderboard {
	leaderboard: LeaderboardEntry[];
	selectedYear: number;
	availableYears: number[];
}

export interface Health {
	status: 'ok';
}
