-- Add migration script here
CREATE TABLE accounts(
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE categories(
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL UNIQUE
);

INSERT INTO categories(id,title)
VALUES 
    ('497dcba3-ecbf-4587-a2dd-5eb0665e6880','Groceries'),
    ('7472cba2-6037-488f-b5aa-53b1c39fe450','Phone'),
    ('5516e359-6c9c-4ebb-a409-52373d536d50','Electricity'),
    ('6e19051f-b126-4109-9b99-275fc3961439','Hospital'),
    ('643526bd-e93b-48fc-bc36-70f7b6de9573','Taxes'),
    ('1e0a5bee-5351-42c4-8053-2e02f37185c5','Insurance'),
    ('54f42a55-e3ea-451e-9f05-867487cbea58','WiFi');


CREATE TABLE transactions(
    id TEXT PRIMARY KEY,
    amount INTEGER NOT NULL,
    account TEXT NOT NULL REFERENCES accounts(id),
    date TEXT NOT NULL,
    category TEXT NOT NULL REFERENCES categories(id)
);