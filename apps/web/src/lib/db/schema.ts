import { relations } from 'drizzle-orm';
import { sqliteTable, text, integer, index } from 'drizzle-orm/sqlite-core';

export const users = sqliteTable('user', {
	id: text().primaryKey(),
	username: text().notNull().unique(),
	type: text(),
	hasAgreedToTerms: integer({ mode: 'boolean' }).notNull().default(false),
	// Users with null are created before 2. aug 2025
	createdAt: integer({ mode: 'timestamp' }).$defaultFn(() => new Date())
});

export const usersRelations = relations(users, ({ one, many }) => ({
	password: one(userPasswords, {
		fields: [users.id],
		references: [userPasswords.userId]
	}),
	session: many(sessions),
	attendances: many(attendees)
}));

export const userPasswords = sqliteTable('user_password', {
	userId: text()
		.primaryKey()
		.references(() => users.id, { onDelete: 'cascade' }),
	passwordHash: text().notNull()
});

export const userPasswordsRelations = relations(userPasswords, ({ one }) => ({
	user: one(users, {
		fields: [userPasswords.userId],
		references: [users.id]
	})
}));

export const sessions = sqliteTable('session', {
	id: text().primaryKey(),
	userId: text()
		.notNull()
		.references(() => users.id),
	expiresAt: integer({ mode: 'timestamp' }).notNull()
});

export const sessionsRelations = relations(sessions, ({ one }) => ({
	user: one(users, {
		fields: [sessions.userId],
		references: [users.id]
	})
}));

export const events = sqliteTable('event', {
	id: text().primaryKey(),
	name: text().notNull(),
	color: text().notNull(),
	createdAt: integer({ mode: 'timestamp' }).notNull(),
	createdBy: text().references(() => users.id, {
		onDelete: 'set null'
	})
});

export const eventsRelations = relations(events, ({ many }) => ({
	attendees: many(attendees)
}));

// Should be renamed to registrations or something similar
// as it is not really an attendee in the sense of a person attending an event,
// but rather a record of a user registering a drink for an event.
export const attendees = sqliteTable(
	'attendee',
	{
		id: text().primaryKey(),
		eventId: text()
			.notNull()
			.references(() => events.id, {
				onDelete: 'cascade'
			}),
		userId: text()
			.notNull()
			.references(() => users.id, {
				onDelete: 'cascade'
			}),
		drinkTypeId: text().references(() => drinkTypes.id, {
			onDelete: 'set null'
		}),
		drinkSizeId: text().references(() => drinkSizes.id, {
			onDelete: 'set null'
		}),
		imageId: text(),
		createdAt: integer({ mode: 'timestamp' }).notNull()
	},
	(t) => [index('attendee_event_idx').on(t.eventId), index('attendee_user_idx').on(t.userId)]
);

export const attendeesRelations = relations(attendees, ({ one }) => ({
	event: one(events, {
		fields: [attendees.eventId],
		references: [events.id]
	}),
	user: one(users, {
		fields: [attendees.userId],
		references: [users.id]
	}),
	drinkType: one(drinkTypes, {
		fields: [attendees.drinkTypeId],
		references: [drinkTypes.id]
	}),
	drinkSize: one(drinkSizes, {
		fields: [attendees.drinkSizeId],
		references: [drinkSizes.id]
	})
}));

export const drinkTypes = sqliteTable('drink_type', {
	id: text().primaryKey(),
	name: text().notNull().unique(), // e.g., 'Beer', 'Wine', 'Cocktail', 'Shot'
	description: text(),
	abv: integer(), // Alcohol by volume percentage (nullable)
	createdAt: integer({ mode: 'timestamp' }).$defaultFn(() => new Date())
});

export const drinkTypesRelations = relations(drinkTypes, ({ many }) => ({
	drinkTypeSizes: many(drinkTypeSizes),
	attendees: many(attendees)
}));

export const drinkSizes = sqliteTable('drink_size', {
	id: text().primaryKey(),
	name: text().notNull(),
	volumeML: integer(), // Volume in milliliters (nullable for "Other" size)
	description: text(),
	createdAt: integer({ mode: 'timestamp' }).$defaultFn(() => new Date())
});

export const drinkSizesRelations = relations(drinkSizes, ({ many }) => ({
	drinkTypeSizes: many(drinkTypeSizes),
	attendees: many(attendees)
}));

export const drinkTypeSizes = sqliteTable(
	'drink_type_size',
	{
		id: text().primaryKey(),
		drinkTypeId: text()
			.notNull()
			.references(() => drinkTypes.id, {
				onDelete: 'cascade'
			}),
		drinkSizeId: text()
			.notNull()
			.references(() => drinkSizes.id, {
				onDelete: 'cascade'
			}),
		createdAt: integer({ mode: 'timestamp' }).$defaultFn(() => new Date())
	},
	(t) => [
		index('drink_type_size_type_idx').on(t.drinkTypeId),
		index('drink_type_size_size_idx').on(t.drinkSizeId)
	]
);

export const drinkTypeSizesRelations = relations(drinkTypeSizes, ({ one }) => ({
	drinkType: one(drinkTypes, {
		fields: [drinkTypeSizes.drinkTypeId],
		references: [drinkTypes.id]
	}),
	drinkSize: one(drinkSizes, {
		fields: [drinkTypeSizes.drinkSizeId],
		references: [drinkSizes.id]
	})
}));

export type Session = typeof sessions.$inferSelect;

export type User = typeof users.$inferSelect;

export type DrinkType = typeof drinkTypes.$inferSelect;

export type DrinkSize = typeof drinkSizes.$inferSelect;

export type DrinkTypeSize = typeof drinkTypeSizes.$inferSelect;
