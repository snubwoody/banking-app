import js from "@eslint/js";
import globals from "globals";
import tseslint from "typescript-eslint";
import json from "@eslint/json";
import css from "@eslint/css";
import svelte from "eslint-plugin-svelte";
import svelteConfig from "./svelte.config.js";
import stylistic from "@stylistic/eslint-plugin";
import { defineConfig, globalIgnores } from "eslint/config";

export default defineConfig([
    globalIgnores([
        "src-tauri",
        "build",
        ".svelte-kit",
        ".vscode"
    ]),
    { 
        files: ["**/*.{js,mjs,cjs,ts,mts,cts}"], 
        plugins: { js }, 
        extends: ["js/recommended"], 
        languageOptions: { globals: {...globals.browser, ...globals.node} } 
    },
    tseslint.configs.recommended,
    {
        files: ["**/*.svelte","**/*.svelte.{js,ts}"],
        extends: svelte.configs.recommended,
        languageOptions: {
            parserOptions:{
                parser: tseslint.parser,
                svelteConfig,
            }
        }
    },
    { files: ["**/*.json"], plugins: { json }, language: "json/json", extends: ["json/recommended"] },
    { files: ["**/*.css"], plugins: { css }, language: "css/css", extends: ["css/recommended"] },
    {
        files: ["**/*.{js,ts,svelte}"],
        plugins: {"@stylistic": stylistic},
        rules: {
            "@stylistic/semi": "error",
            "@stylistic/quotes": ["warn","double"],
            "@stylistic/indent": ["warn",4],
        }
    }
]);
