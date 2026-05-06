-- Insert drink types
INSERT INTO drink_type (id, name, description, created_at) VALUES 
('beer', 'Øl', 'Alle typer øl og ølbaserte drikker', strftime('%s', 'now') * 1000),
('wine', 'Vin', 'Rødvin, hvitvin og musserende', strftime('%s', 'now') * 1000),
('cocktail', 'Cocktail', 'Blandede drinker og cocktails', strftime('%s', 'now') * 1000),
('shot', 'Shot', 'Shots og snaps', strftime('%s', 'now') * 1000),
('liquor', 'Brennevin', 'Whisky, vodka, gin og andre sterke drikker', strftime('%s', 'now') * 1000),
('cider', 'Cider', 'Eple- og pærecider', strftime('%s', 'now') * 1000);

-- Insert drink sizes
INSERT INTO drink_size (id, name, volume_ml, description, created_at) VALUES 
-- Beer sizes
('beer_small', '0,33L', 330, 'Liten øl', strftime('%s', 'now') * 1000),
('beer_medium', '0,4L', 400, 'Medium øl', strftime('%s', 'now') * 1000),
('beer_large', '0,5L', 500, 'Stor øl', strftime('%s', 'now') * 1000),
('beer_pint', 'Pint', 568, 'Engelsk pint', strftime('%s', 'now') * 1000),
-- Wine sizes
('wine_glass', 'Et glass', 150, 'Standard vinglass', strftime('%s', 'now') * 1000),
('wine_large_glass', 'Stort glass', 200, 'Stort vinglass', strftime('%s', 'now') * 1000),
('wine_bottle', 'Flaske', 750, 'Hel flaske vin', strftime('%s', 'now') * 1000),
-- Shot sizes
('shot_standard', 'Shot', 40, 'Standard shot', strftime('%s', 'now') * 1000),
('shot_double', 'Dobbel shot', 80, 'Dobbel shot', strftime('%s', 'now') * 1000),
-- Cocktail sizes
('cocktail_small', 'Liten drink', 200, 'Liten cocktail', strftime('%s', 'now') * 1000),
('cocktail_standard', 'Standard drink', 300, 'Standard cocktail', strftime('%s', 'now') * 1000),
-- Liquor sizes
('liquor_shot', 'Shot', 40, 'Shot brennevin', strftime('%s', 'now') * 1000),
('liquor_double', 'Dobbel', 80, 'Dobbel brennevin', strftime('%s', 'now') * 1000),
('liquor_glass', 'Glass', 100, 'Glass brennevin', strftime('%s', 'now') * 1000),
-- Cider sizes
('cider_small', '0,33L', 330, 'Liten cider', strftime('%s', 'now') * 1000),
('cider_large', '0,5L', 500, 'Stor cider', strftime('%s', 'now') * 1000);

-- Insert valid drink type and size combinations
INSERT INTO drink_type_size (id, drink_type_id, drink_size_id, created_at) VALUES 
-- Beer combinations
('beer_small_combo', 'beer', 'beer_small', strftime('%s', 'now') * 1000),
('beer_large_combo', 'beer', 'beer_large', strftime('%s', 'now') * 1000),
('beer_pint_combo', 'beer', 'beer_pint', strftime('%s', 'now') * 1000),
-- Wine combinations
('wine_glass_combo', 'wine', 'wine_glass', strftime('%s', 'now') * 1000),
('wine_large_glass_combo', 'wine', 'wine_large_glass', strftime('%s', 'now') * 1000),
('wine_bottle_combo', 'wine', 'wine_bottle', strftime('%s', 'now') * 1000),
-- Shot combinations
('shot_standard_combo', 'shot', 'shot_standard', strftime('%s', 'now') * 1000),
('shot_double_combo', 'shot', 'shot_double', strftime('%s', 'now') * 1000),
-- Cocktail combinations
('cocktail_small_combo', 'cocktail', 'cocktail_small', strftime('%s', 'now') * 1000),
('cocktail_standard_combo', 'cocktail', 'cocktail_standard', strftime('%s', 'now') * 1000),
-- Liquor combinations
('liquor_shot_combo', 'liquor', 'liquor_shot', strftime('%s', 'now') * 1000),
('liquor_double_combo', 'liquor', 'liquor_double', strftime('%s', 'now') * 1000),
('liquor_glass_combo', 'liquor', 'liquor_glass', strftime('%s', 'now') * 1000),
-- Cider combinations
('cider_small_combo', 'cider', 'cider_small', strftime('%s', 'now') * 1000),
('cider_large_combo', 'cider', 'cider_large', strftime('%s', 'now') * 1000);