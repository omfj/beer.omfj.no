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
	})
}));

export type Session = typeof sessions.$inferSelect;

export type User = typeof users.$inferSelect;
