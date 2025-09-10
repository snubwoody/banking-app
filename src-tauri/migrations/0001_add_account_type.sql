CREATE TABLE account_type(
    id TEXT PRIMARY KEY NOT NULL DEFAULT (hex(randomblob(8))),
    title TEXT UNIQUE NOT NULL
);

INSERT INTO account_type(title)
VALUES 
    ('Chequing'),
    ('Credit'),
    ('Savings');