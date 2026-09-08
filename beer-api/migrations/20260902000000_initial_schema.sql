CREATE TABLE `event` (
	`id` text PRIMARY KEY NOT NULL,
	`name` text NOT NULL,
	`color` text NOT NULL,
	`created_at` integer NOT NULL,
	`created_by` text REFERENCES user(id),
	`password` text
);

CREATE TABLE `session` (
	`id` text PRIMARY KEY NOT NULL,
	`user_id` text NOT NULL,
	`expires_at` integer NOT NULL,
	FOREIGN KEY (`user_id`) REFERENCES `user`(`id`) ON UPDATE no action ON DELETE no action
);

CREATE TABLE `user_password` (
	`user_id` text PRIMARY KEY NOT NULL,
	`password_hash` text NOT NULL,
	FOREIGN KEY (`user_id`) REFERENCES `user`(`id`) ON UPDATE no action ON DELETE cascade
);

CREATE TABLE `user` (
	`id` text PRIMARY KEY NOT NULL,
	`username` text NOT NULL,
	`type` text,
	`has_agreed_to_terms` integer DEFAULT false NOT NULL,
	`created_at` integer,
	`gender` text,
	`weight` text
);

CREATE TABLE IF NOT EXISTS "attendee" (
	`id` text PRIMARY KEY NOT NULL,
	`event_id` text NOT NULL,
	`user_id` text NOT NULL,
	`image_id` text,
	`created_at` integer NOT NULL,
	`drink_type_id` text REFERENCES drink_type(id),
	`drink_size_id` text REFERENCES drink_size(id),
	abv REAL,
	FOREIGN KEY (`event_id`) REFERENCES `event`(`id`) ON UPDATE no action ON DELETE cascade,
	FOREIGN KEY (`user_id`) REFERENCES `user`(`id`) ON UPDATE no action ON DELETE cascade
);

CREATE TABLE `drink_size` (
	`id` text PRIMARY KEY NOT NULL,
	`name` text NOT NULL,
	`volume_ml` integer NOT NULL,
	`description` text,
	`created_at` integer
);

CREATE TABLE `drink_type_size` (
	`id` text PRIMARY KEY NOT NULL,
	`drink_type_id` text NOT NULL,
	`drink_size_id` text NOT NULL,
	`created_at` integer,
	FOREIGN KEY (`drink_type_id`) REFERENCES `drink_type`(`id`) ON UPDATE no action ON DELETE cascade,
	FOREIGN KEY (`drink_size_id`) REFERENCES `drink_size`(`id`) ON UPDATE no action ON DELETE cascade
);

CREATE TABLE `drink_type` (
	`id` text PRIMARY KEY NOT NULL,
	`name` text NOT NULL,
	`description` text,
	`created_at` integer,
	`abv` integer,
	multiplier REAL NOT NULL DEFAULT 1.0
);

CREATE TABLE `event_access` (
	`event_id` text NOT NULL,
	`user_id` text NOT NULL,
	`granted_at` integer NOT NULL,
	PRIMARY KEY(`event_id`, `user_id`),
	FOREIGN KEY (`event_id`) REFERENCES `event`(`id`) ON UPDATE no action ON DELETE cascade,
	FOREIGN KEY (`user_id`) REFERENCES `user`(`id`) ON UPDATE no action ON DELETE cascade
);

CREATE TABLE `passkey` (
	`id` text PRIMARY KEY NOT NULL,
	`user_id` text NOT NULL,
	`public_key` text NOT NULL,
	`counter` integer DEFAULT 0 NOT NULL,
	`transports` text,
	`created_at` integer NOT NULL, `name` text,
	FOREIGN KEY (`user_id`) REFERENCES `user`(`id`) ON UPDATE no action ON DELETE cascade
);

CREATE UNIQUE INDEX `user_username_unique` ON `user` (`username`);
CREATE INDEX `attendee_event_idx` ON `attendee` (`event_id`);
CREATE INDEX `attendee_user_idx` ON `attendee` (`user_id`);
CREATE INDEX `drink_type_size_type_idx` ON `drink_type_size` (`drink_type_id`);
CREATE INDEX `drink_type_size_size_idx` ON `drink_type_size` (`drink_size_id`);
CREATE UNIQUE INDEX `drink_type_name_unique` ON `drink_type` (`name`);
CREATE INDEX `passkey_user_idx` ON `passkey` (`user_id`);
