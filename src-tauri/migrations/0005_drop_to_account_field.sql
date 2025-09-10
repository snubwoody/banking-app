-- Drop to_account field on transactions
-- will create a dedicated table for transfers
ALTER TABLE transactions
DROP COLUMN to_account;