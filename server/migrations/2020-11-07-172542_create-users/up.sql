CREATE TABLE USERS (
    id uuid NOT NULL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    active boolean NOT NULL DEFAULT 'f',
    password TEXT NOT NULL,
    status TEXT NOT NULL,
    created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP
);
