-- Custom SQL migration file, put your code below! --

-- Update drink types with typical ABV values
UPDATE drink_type SET abv = 5 WHERE id = 'beer';        -- Beer: 5% ABV
UPDATE drink_type SET abv = 12 WHERE id = 'wine';       -- Wine: 12% ABV
UPDATE drink_type SET abv = 15 WHERE id = 'cocktail';   -- Cocktail: 15% ABV (average)
UPDATE drink_type SET abv = 40 WHERE id = 'shot';       -- Shot: 40% ABV
UPDATE drink_type SET abv = 40 WHERE id = 'liquor';     -- Liquor: 40% ABV
UPDATE drink_type SET abv = 5 WHERE id = 'cider';       -- Cider: 5% ABV