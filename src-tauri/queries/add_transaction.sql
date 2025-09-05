INSERT INTO transactions(amount,account,category,date) 
VALUES($1,$2,$3,$4) 
RETURNING id;