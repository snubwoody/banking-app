SELECT 
    t.id,t.date,t.amount,
    c.id as category_id,
    c.title as category_title,
    a.id AS account_id,
    a.name AS account_name
FROM transactions as t
JOIN categories AS c ON c.id = t.category
JOIN accounts as a ON a.id = t.account 
WHERE a.id = $1;