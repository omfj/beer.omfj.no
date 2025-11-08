-- Custom SQL migration file, put your code below! --

-- Update cocktail ABV from 15% to 10% to balance scoring
-- This creates a more balanced system where beer is incentivized without making cocktails inviable
UPDATE drink_type SET abv = 10 WHERE id = 'cocktail';
