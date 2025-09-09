
export type Account = {
    id: string,
    name: string,
    starting_balance: number,
    account_type: AccountType
};

export type Transaction = {
    id: string,
    amount: number,
    category: Category,
    account: Account,
    date: string
};

export type Category = {
    id: string,
    title: string
};

export type AccountType = {
    id: string,
    title: string
};