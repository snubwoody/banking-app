SELECT 
    t.id,t.date,t.amount,
    c.id as category_id,
    c.title as category_title,
    a.id AS account_id,
    at.id AS account_type_id,
    at.title as account_type,
    a.name AS account_name,
    a.starting_balance AS account_starting_balance
FROM transactions as t
JOIN categories AS c ON c.id = t.category
JOIN accounts AS a ON a.id = t.account 
JOIN account_type AS at ON a.account_type = at.id
WHERE a.id = $1;