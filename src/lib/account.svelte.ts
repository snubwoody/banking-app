import { invoke } from "@tauri-apps/api/core";
import type { Account, AccountType, Transaction } from "./db";

export class AccountStore{
    accounts: Account[] = $state([]);
    transactions: Transaction[] = $state([]);
    private _accountTypes: AccountType[] = [];

    constructor(){
        this.fetch();
    }

    private async fetch() {
        invoke<Account[]>("fetch_accounts")
            .then(val => { this.accounts = val; });
        invoke<AccountType[]>("get_account_types")
            .then(val => this._accountTypes = val);
    }

    get accountTypes(){
        return this._accountTypes;
    }

    async addAccount(name: string, accountType: number, startingBalance: number){
        await invoke("create_account", {name, accountType, startingBalance});
        await this.fetch();
    }
}

/**
 * Global account store
*/
export const accountStore = new AccountStore();
