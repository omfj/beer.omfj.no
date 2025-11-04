ALTER TABLE `attendee` ADD `drink_type_id` text REFERENCES drink_type(id);--> statement-breakpoint
ALTER TABLE `attendee` ADD `drink_size_id` text REFERENCES drink_size(id);