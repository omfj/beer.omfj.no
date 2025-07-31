import { relations } from 'drizzle-orm';
import { sqliteTable, text, integer } from 'drizzle-orm/sqlite-core';

export const users = sqliteTable('user', {
	id: text('id').primaryKey(),
	username: text('username').notNull().unique(),
	type: text('type')
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
	userId: text('user_id')
		.primaryKey()
		.references(() => users.id, { onDelete: 'cascade' }),
	passwordHash: text('password_hash').notNull()
});

export const userPasswordsRelations = relations(userPasswords, ({ one }) => ({
	user: one(users, {
		fields: [userPasswords.userId],
		references: [users.id]
	})
}));

export const sessions = sqliteTable('session', {
	id: text('id').primaryKey(),
	userId: text('user_id')
		.notNull()
		.references(() => users.id),
	expiresAt: integer('expires_at', { mode: 'timestamp' }).notNull()
});

export const sessionsRelations = relations(sessions, ({ one }) => ({
	user: one(users, {
		fields: [sessions.userId],
		references: [users.id]
	})
}));

export const events = sqliteTable('event', {
	id: text('id').primaryKey(),
	name: text('name').notNull(),
	color: text('color').notNull(),
	createdAt: integer('created_at', { mode: 'timestamp' }).notNull()
});

export const eventsRelations = relations(events, ({ many }) => ({
	attendees: many(attendees)
}));

export const attendees = sqliteTable('attendee', {
	id: text('id').primaryKey(),
	eventId: text('event_id')
		.notNull()
		.references(() => events.id),
	userId: text('user_id')
		.notNull()
		.references(() => users.id),
	imageId: text('image_id'),
	createdAt: integer('created_at', { mode: 'timestamp' }).notNull()
});

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
