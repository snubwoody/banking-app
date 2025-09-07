import { invoke } from "@tauri-apps/api/core";
import type { Account, Transaction } from "./db";

export class AccountStore{
    accounts: Account[] = $state([]);
    transactions: Transaction[] = $state([]);

    constructor(){
        invoke<Account[]>("fetch_accounts")
            .then(val => { console.log(val); this.accounts = val; });
    }

}

/**
 * Global account store
*/
export const accountStore = new AccountStore();
