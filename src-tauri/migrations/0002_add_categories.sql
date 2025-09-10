CREATE TABLE categories(
    id TEXT PRIMARY KEY NOT NULL DEFAULT (hex(randomBlob(8))),
    title TEXT NOT NULL UNIQUE,
    CHECK(length(title) >= 2)
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


