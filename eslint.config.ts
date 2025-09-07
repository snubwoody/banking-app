import js from "@eslint/js";
import globals from "globals";
import tseslint from "typescript-eslint";
import json from "@eslint/json";
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
        languageOptions: { globals: {...globals.browser, ...globals.node} },
        rules: {
            "prefer-const": ["warn"]
        }
    },
    tseslint.configs.recommended,
    {
        files: ["**/*.svelte", "**/*.svelte.{js,ts}"],
        extends: svelte.configs.recommended,
        languageOptions: {
            parserOptions:{
                parser: tseslint.parser,
                svelteConfig,
            }
        }
    },
    { files: ["**/*.json"], plugins: { json }, language: "json/json", extends: ["json/recommended"] },
    {
        files: ["**/*.{js,ts,svelte}"],
        plugins: {"@stylistic": stylistic},
        rules: {
            "@stylistic/semi": "error",
            "@stylistic/quotes": ["error", "double"],
            "@stylistic/indent": ["warn", 4],
            "@stylistic/arrow-spacing":["warn"],
            "@stylistic/block-spacing":["warn"],
            "@stylistic/comma-spacing":["warn", {"before": false, "after": true}],
            "@stylistic/new-parens": ["error"],
            "@typescript-eslint/no-unused-vars": ["warn"],
        },
    },
]);
