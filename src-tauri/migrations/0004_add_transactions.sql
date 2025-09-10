CREATE TABLE transactions(
    id TEXT PRIMARY KEY NOT NULL DEFAULT (hex(randomBlob(8))),
    amount REAL NOT NULL,
    account TEXT NOT NULL REFERENCES accounts(id),
    to_account TEXT NULL REFERENCES accounts(id), -- for transfers
    date TEXT NOT NULL,
    category TEXT NOT NULL REFERENCES categories(id)
);
