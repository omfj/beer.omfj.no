-- Custom SQL migration file, put your code below! --
ALTER TABLE attendee ADD COLUMN abv REAL;

UPDATE attendee
SET abv = (
  SELECT dt.abv
  FROM drink_type dt
  WHERE dt.id = attendee.drink_type_id
)
WHERE drink_type_id IS NOT NULL;