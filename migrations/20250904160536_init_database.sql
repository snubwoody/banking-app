-- Add migration script here
CREATE TABLE accounts(
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL
);


CREATE TABLE transactions(
    id TEXT PRIMARY KEY,
    amount INTEGER NOT NULL,
    account TEXT NOT NULL REFERENCES accounts(id)
);