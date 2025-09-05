-- Add migration script here
CREATE TABLE account_type(
    id INTEGER PRIMARY KEY NOT NULL,
    title TEXT UNIQUE NOT NULL
);

INSERT INTO account_type(title)
VALUES 
    ('Chequing'),
    ('Savings');

CREATE TABLE accounts(
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    starting_balance INTEGER NOT NULL DEFAULT 0,
    account_type INTEGER NOT NULL REFERENCES account_type(id) DEFAULT 1
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
    ('Correction'),
    ('Miscellaneous');


CREATE TABLE transactions(
    id INTEGER PRIMARY KEY NOT NULL,
    amount INTEGER NOT NULL,
    account INTEGER NOT NULL REFERENCES accounts(id),
    date TEXT NOT NULL,
    category INTEGER NOT NULL REFERENCES categories(id)
);