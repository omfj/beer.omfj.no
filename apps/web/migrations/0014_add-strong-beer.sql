-- Custom SQL migration file, put your code below! --

-- Insert strong beer drink type
INSERT INTO drink_type (id, name, description, abv, created_at) VALUES 
('strong_beer', 'Sterk øl', 'Sterkøl og kraftige øltyper', 8, strftime('%s', 'now') * 1000);

-- Link strong beer with existing beer sizes
INSERT INTO drink_type_size (id, drink_type_id, drink_size_id, created_at) VALUES 
('strong_beer_small_combo', 'strong_beer', 'beer_small', strftime('%s', 'now') * 1000),
('strong_beer_medium_combo', 'strong_beer', 'beer_medium', strftime('%s', 'now') * 1000),
('strong_beer_large_combo', 'strong_beer', 'beer_large', strftime('%s', 'now') * 1000),
('strong_beer_pint_combo', 'strong_beer', 'beer_pint', strftime('%s', 'now') * 1000),
('strong_beer_huge_combo', 'strong_beer', 'beer_huge', strftime('%s', 'now') * 1000),
('strong_beer_jug_combo', 'strong_beer', 'beer_jug', strftime('%s', 'now') * 1000);
