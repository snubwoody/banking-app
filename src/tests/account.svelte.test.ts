import {test, expect} from "vitest";
import { accountStore } from "$lib/account.svelte";

test("Create account store", () => {
    console.log("Hello");
    console.log(accountStore);
});