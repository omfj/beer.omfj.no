-- Custom SQL migration file, put your code below! --

INSERT INTO drink_size (id, name, volume_ml, description, created_at) VALUES 
-- Beer sizes
('beer_huge', '1L', 1000, 'Liter øl', strftime('%s', 'now') * 1000),
('beer_jug', 'Mugge', 2400, 'Mugge med øl', strftime('%s', 'now') * 1000);

INSERT INTO drink_type_size (id, drink_type_id, drink_size_id, created_at) VALUES 
('beer_huge_combo', 'beer', 'beer_huge', strftime('%s', 'now') * 1000),
('beer_jug_combo', 'beer', 'beer_jug', strftime('%s', 'now') * 1000);