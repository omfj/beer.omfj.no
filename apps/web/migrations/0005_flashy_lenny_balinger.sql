PRAGMA foreign_keys=OFF;--> statement-breakpoint
CREATE TABLE `__new_attendee` (
	`id` text PRIMARY KEY NOT NULL,
	`event_id` text NOT NULL,
	`user_id` text NOT NULL,
	`image_id` text,
	`created_at` integer NOT NULL,
	FOREIGN KEY (`event_id`) REFERENCES `event`(`id`) ON UPDATE no action ON DELETE cascade,
	FOREIGN KEY (`user_id`) REFERENCES `user`(`id`) ON UPDATE no action ON DELETE cascade
);
--> statement-breakpoint
INSERT INTO `__new_attendee`("id", "event_id", "user_id", "image_id", "created_at") SELECT "id", "event_id", "user_id", "image_id", "created_at" FROM `attendee`;--> statement-breakpoint
DROP TABLE `attendee`;--> statement-breakpoint
ALTER TABLE `__new_attendee` RENAME TO `attendee`;--> statement-breakpoint
PRAGMA foreign_keys=ON;--> statement-breakpoint
CREATE INDEX `attendee_event_idx` ON `attendee` (`event_id`);--> statement-breakpoint
CREATE INDEX `attendee_user_idx` ON `attendee` (`user_id`);