
export type Account = {
    id: string,
    name: string
}

export type Transaction = {
    id: string,
    account: string,
    amount: number
}