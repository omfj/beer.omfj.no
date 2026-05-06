import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { attendees, events, users, drinkTypes, drinkSizes } from '$lib/db/schema';
import { and, gte, isNull, lt, sql, eq } from 'drizzle-orm';
import { calculateDrinkPoints } from '$lib/scoring';

export const load: PageServerLoad = async ({ locals, url }) => {
	if (!locals.user) {
		redirect(302, '/logg-inn');
	}

	const yearParam = url.searchParams.get('year');
	const selectedYear = yearParam && /^\d{4}$/.test(yearParam) ? Number(yearParam) : null;
	const selectedYearValue = selectedYear ? selectedYear.toString() : 'all';

	// Build date filters for the selected year
	let startDate: Date | null = null;
	let endDate: Date | null = null;

	if (selectedYear) {
		startDate = new Date(selectedYear, 0, 1); // January 1st
		endDate = new Date(selectedYear + 1, 0, 1); // January 1st next year
	}

	const dateFilter = selectedYear
		? and(gte(attendees.createdAt, startDate!), lt(attendees.createdAt, endDate!))
		: undefined;

	// Single JOIN query — only open events, no post-filter needed
	const attendeeRecords = await locals.db
		.select({
			userId: attendees.userId,
			username: users.username,
			volumeML: drinkSizes.volumeML,
			abv: drinkTypes.abv
		})
		.from(attendees)
		.innerJoin(events, eq(attendees.eventId, events.id))
		.innerJoin(users, eq(attendees.userId, users.id))
		.leftJoin(drinkTypes, eq(attendees.drinkTypeId, drinkTypes.id))
		.leftJoin(drinkSizes, eq(attendees.drinkSizeId, drinkSizes.id))
		.where(dateFilter ? and(isNull(events.password), dateFilter) : isNull(events.password));

	// Calculate points for each user
	const userPointsMap = new Map<string, { username: string; points: number; drinkCount: number }>();

	for (const record of attendeeRecords) {
		const userId = record.userId;
		const username = record.username;
		const volumeML = record.volumeML ?? null;
		const abv = record.abv ?? null;

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

	// Get list of available years from open events only
	const yearResults = await locals.db
		.selectDistinct({
			year: sql<number>`CAST(strftime('%Y', ${attendees.createdAt}, 'unixepoch') AS INTEGER)`
		})
		.from(attendees)
		.innerJoin(events, eq(attendees.eventId, events.id))
		.where(isNull(events.password))
		.orderBy(sql`CAST(strftime('%Y', ${attendees.createdAt}, 'unixepoch') AS INTEGER) DESC`);

	const availableYears = yearResults.map((r) => r.year);

	return {
		leaderboard,
		selectedYear,
		selectedYearValue,
		availableYears
	};
};
