import { invoke } from "@tauri-apps/api/core";
import type { Account, AccountType, Category, Transaction } from "./db";

export class AccountStore{
    accounts: Account[] = $state([]);
    transactions: Transaction[] = $state([]);
    categories: Category[] = $state([]);
    private _accountTypes: AccountType[] = $state([]);

    constructor(){
        this.fetch();
        console.log(this);
    }

    private async fetch() {
        this.accounts = await invoke<Account[]>("fetch_accounts");
        this._accountTypes = await invoke<AccountType[]>("get_account_types");
        this.transactions = await invoke<Transaction[]>("get_all_transactions");
        this.categories = await invoke<Category[]>("get_categories");
    }

    get accountTypes(){
        return this._accountTypes;
    }

    async addAccount(name: string, accountType: string, startingBalance: number){
        await invoke("create_account", {name, accountType, startingBalance});
        await this.fetch();
    }

    async addTransaction(amount: number, account: Account, category: Category, date: string){
        await invoke("add_transaction", {amount, account: account.id, category: category.id, date});
        await this.fetch();
    }

    async addCategory(title: string){
        await invoke("add_category", {title});
        await this.fetch();
    }
}

/**
 * Global account store
*/
export const accountStore = new AccountStore();
