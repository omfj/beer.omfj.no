import { eq, and } from 'drizzle-orm';
import * as table from '$lib/db/schema';
import type { Database } from '$lib/db';

export async function hasEventAccess(
	db: Database,
	event: { id: string; password: string | null; createdBy: string | null },
	userId: string
): Promise<boolean> {
	if (!event.password) return true;
	if (event.createdBy === userId) return true;

	const access = await db.query.eventAccess.findFirst({
		where: and(eq(table.eventAccess.eventId, event.id), eq(table.eventAccess.userId, userId))
	});

	return !!access;
}

export async function grantEventAccess(db: Database, eventId: string, userId: string) {
	await db
		.insert(table.eventAccess)
		.values({ eventId, userId, grantedAt: new Date() })
		.onConflictDoNothing();
}
