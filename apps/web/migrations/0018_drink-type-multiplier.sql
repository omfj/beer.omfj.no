-- Custom SQL migration file, put your code below! --
ALTER TABLE drink_type ADD COLUMN multiplier REAL NOT NULL DEFAULT 1.0;

UPDATE drink_type SET multiplier = 0.9 WHERE id = 'wine';
UPDATE drink_type SET multiplier = 0.75 WHERE id = 'cocktail';
