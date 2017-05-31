CREATE TABLE gallery_images(
  gallery_id INTEGER NOT NULL,
  image_id INTEGER NOT NULL,
  FOREIGN KEY(gallery_id) REFERENCES galleries(id),
  FOREIGN KEY(image_id) REFERENCES image_locations(id),
  PRIMARY KEY(gallery_id, image_id)
);
