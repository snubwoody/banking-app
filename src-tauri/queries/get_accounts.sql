SELECT 
    a.id,a.name,a.starting_balance, 
    t.id AS account_type_id,
    t.title AS account_type
FROM accounts AS a
JOIN account_type AS t ON t.id = a.account_type;