-- Storing users.
CREATE TABLE users(
    id INTEGER NOT NULL,
    username TEXT NOT NULL UNIQUE,
    hash TEXT NOT NULL,
    salt TEXT NOT NULL,
    PRIMARY KEY(id DESC) );