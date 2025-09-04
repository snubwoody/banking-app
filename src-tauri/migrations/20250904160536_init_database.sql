-- Add migration script here
CREATE TABLE accounts(
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL
);

CREATE TABLE categories(
    id INTEGER PRIMARY KEY NOT NULL,
    title TEXT NOT NULL UNIQUE
);

INSERT INTO categories(title)
VALUES 
    ('Groceries'),
    ('Phone'),
    ('Electricity'),
    ('Hospital'),
    ('Taxes'),
    ('Insurance'),
    ('Miscellaneous');


CREATE TABLE transactions(
    id INTEGER PRIMARY KEY NOT NULL,
    amount INTEGER NOT NULL,
    account INTEGER NOT NULL REFERENCES accounts(id),
    date TEXT NOT NULL,
    category INTEGER NOT NULL REFERENCES categories(id)
);