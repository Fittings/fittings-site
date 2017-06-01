-- Storing images with ids is

drop table image_locations;

CREATE TABLE image_locations(
  id INTEGER NOT NULL,
  name TEXT NOT NULL UNIQUE,
  url TEXT NOT NULL,
  PRIMARY KEY(id DESC)
);

