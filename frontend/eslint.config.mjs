import tseslint from "typescript-eslint";
import baseEslintConfig from "./configs/eslint/eslint.config.base.mjs";

/** @type {import('eslint').Linter.Config[]} */
export default tseslint.config(...baseEslintConfig, {
  rules: {
    "@typescript-eslint/ban-ts-comment": "warn",
  },
});
