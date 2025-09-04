INSERT INTO transactions(amount,account,category) 
VALUES($1,$2,$3) 
RETURNING id;