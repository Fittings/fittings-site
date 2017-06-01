-- Drop the new image_locations and return the image_locations table of old.
DROP TABLE image_locations;

CREATE TABLE image_locations(
  id INTEGER NOT NULL,
  url TEXT NOT NULL,
  PRIMARY KEY(id DESC)
);
