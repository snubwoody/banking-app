SELECT 
    t.id, t.date, t.amount,
    account.id AS account_id,
    account.name AS account_name,
    account.starting_balance as account_starting_balance,
    account_type.id AS account_type_id,
    account_type.title AS account_type_title,
    categories.id AS category_id,
    categories.title AS category_title
FROM transactions AS t
JOIN accounts AS account ON account.id = t.account
JOIN categories ON categories.id = t.category
JOIN account_type ON account_type.id = account.account_type
WHERE t.id = $1