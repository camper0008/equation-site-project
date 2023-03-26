DROP TABLE IF EXISTS users;
CREATE TABLE users (
    id VARCHAR(8) PRIMARY KEY NOT NULL,
    username TEXT NOT NULL,
    permission TEXT NOT NULL,
    date_created TEXT NOT NULL,
    password TEXT NOT NULL
);

DROP TABLE IF EXISTS equations;
CREATE TABLE equations (
    id VARCHAR(8) PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    date_created TEXT NOT NULL,
    creator_id VARCHAR(64) NOT NULL
);

DROP TABLE IF EXISTS sessions;
CREATE TABLE sessions (
    token VARCHAR(64) PRIMARY KEY NOT NULL,
    user_id TEXT NOT NULL,
    date_created TEXT NOT NULL
);
