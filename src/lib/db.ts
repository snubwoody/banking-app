
export type Account = {
    id: number,
    name: string,
    starting_balance: number,
    account_type: AccountType
};

export type Transaction = {
    id: number,
    amount: number,
    category: Category,
    account: Account,
    date: string
};

export type Category = {
    id: number,
    title: string
};

export type AccountType = {
    id: number,
    title: string
};