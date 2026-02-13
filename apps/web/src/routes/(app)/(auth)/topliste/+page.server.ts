import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { attendees, users, drinkTypes, drinkSizes } from '$lib/db/schema';
import { eq, and, gte, lt, sql } from 'drizzle-orm';
import { calculateDrinkPoints } from '$lib/scoring';

export const load: PageServerLoad = async ({ locals, url }) => {
	if (!locals.user) {
		redirect(302, '/logg-inn');
	}

	const yearParam = url.searchParams.get('year');
	const selectedYear = yearParam ? parseInt(yearParam) : null;

	// Build date filters for the selected year
	let startDate: Date | null = null;
	let endDate: Date | null = null;

	if (selectedYear) {
		startDate = new Date(selectedYear, 0, 1); // January 1st
		endDate = new Date(selectedYear + 1, 0, 1); // January 1st next year
	}

	// Get all drink registrations with user, drink type and size info
	let attendeeRecords = await locals.db.query.attendees.findMany({
		with: {
			user: true,
			drinkType: true,
			drinkSize: true
		},
		where: selectedYear
			? and(gte(attendees.createdAt, startDate!), lt(attendees.createdAt, endDate!))
			: undefined
	});

	// Calculate points for each user
	const userPointsMap = new Map<string, { username: string; points: number; drinkCount: number }>();

	for (const record of attendeeRecords) {
		const userId = record.userId;
		const username = record.user.username;
		const volumeML = record.drinkSize?.volumeML ?? null;
		const abv = record.drinkType?.abv ?? null;

		const points = calculateDrinkPoints(volumeML, abv);

		if (!userPointsMap.has(userId)) {
			userPointsMap.set(userId, { username, points: 0, drinkCount: 0 });
		}

		const userStats = userPointsMap.get(userId)!;
		userStats.points += points;
		userStats.drinkCount += 1;
	}

	// Convert to array and sort by points (descending)
	const leaderboard = Array.from(userPointsMap.entries())
		.map(([userId, stats]) => ({
			userId,
			username: stats.username,
			points: Math.round(stats.points * 10) / 10, // Round to 1 decimal
			drinkCount: stats.drinkCount
		}))
		.sort((a, b) => b.points - a.points)
		.slice(0, 10); // Top 10

	// Get list of available years from the database
	const yearResults = await locals.db
		.selectDistinct({
			year: sql<number>`CAST(strftime('%Y', ${attendees.createdAt}, 'unixepoch') AS INTEGER)`
		})
		.from(attendees)
		.orderBy(sql`CAST(strftime('%Y', ${attendees.createdAt}, 'unixepoch') AS INTEGER) DESC`);

	const availableYears = yearResults.map((r) => r.year);

	return {
		leaderboard,
		selectedYear,
		availableYears
	};
};
