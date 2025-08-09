CREATE TABLE `drink_size` (
	`id` text PRIMARY KEY NOT NULL,
	`name` text NOT NULL,
	`volume_ml` integer NOT NULL,
	`description` text,
	`created_at` integer
);
--> statement-breakpoint
CREATE TABLE `drink_type_size` (
	`id` text PRIMARY KEY NOT NULL,
	`drink_type_id` text NOT NULL,
	`drink_size_id` text NOT NULL,
	`created_at` integer,
	FOREIGN KEY (`drink_type_id`) REFERENCES `drink_type`(`id`) ON UPDATE no action ON DELETE cascade,
	FOREIGN KEY (`drink_size_id`) REFERENCES `drink_size`(`id`) ON UPDATE no action ON DELETE cascade
);
--> statement-breakpoint
CREATE INDEX `drink_type_size_type_idx` ON `drink_type_size` (`drink_type_id`);--> statement-breakpoint
CREATE INDEX `drink_type_size_size_idx` ON `drink_type_size` (`drink_size_id`);--> statement-breakpoint
CREATE TABLE `drink_type` (
	`id` text PRIMARY KEY NOT NULL,
	`name` text NOT NULL,
	`description` text,
	`created_at` integer
);
--> statement-breakpoint
CREATE UNIQUE INDEX `drink_type_name_unique` ON `drink_type` (`name`);