
export type Account = {
    id: number,
    name: string
}

export type Transaction = {
    id: number,
    amount: number,
    category: Category,
    account: Account
}

export type Category = {
    id: number,
    title: string
}

export type AccountType = {
    id: number,
    title: string
}