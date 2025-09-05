INSERT INTO accounts(name,account_type,starting_balance)
VALUES ($1,$2,$3)
RETURNING id;