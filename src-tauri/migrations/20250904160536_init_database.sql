CREATE TABLE account_type(
    id TEXT PRIMARY KEY NOT NULL DEFAULT (hex(randomblob(8))),
    title TEXT UNIQUE NOT NULL
);

INSERT INTO account_type(title)
VALUES 
    ('Chequing'),
    ('Savings');

CREATE TABLE accounts(
    id TEXT PRIMARY KEY NOT NULL DEFAULT (hex(randomBlob(8))),
    name TEXT NOT NULL,
    starting_balance INTEGER NOT NULL DEFAULT 0,
    account_type TEXT NOT NULL,
    Foreign Key (account_type) REFERENCES account_type(id)
);

CREATE TABLE categories(
    id TEXT PRIMARY KEY NOT NULL DEFAULT (hex(randomBlob(8))),
    title TEXT NOT NULL UNIQUE
);

INSERT INTO categories(title)
VALUES 
    ('Groceries'),
    ('Phone'),
    ('Electricity'),
    ('Hospital'),
    ('Rent'),
    ('Entertainment'),
    ('Taxes'),
    ('Insurance'),
    ('Correction'),
    ('Miscellaneous');


CREATE TABLE transactions(
    id TEXT PRIMARY KEY NOT NULL DEFAULT (hex(randomBlob(8))),
    amount INTEGER NOT NULL,
    account TEXT NOT NULL REFERENCES accounts(id),
    date TEXT NOT NULL,
    category TEXT NOT NULL REFERENCES categories(id)
);