CREATE TABLE accounts(
    id TEXT PRIMARY KEY NOT NULL DEFAULT (hex(randomBlob(8))),
    name TEXT NOT NULL,
    starting_balance REAL NOT NULL DEFAULT 0,
    account_type TEXT NOT NULL,
    Foreign Key (account_type) REFERENCES account_type(id)
);
