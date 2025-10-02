import { FlatCompat } from '@eslint/eslintrc';
import { defineConfig } from 'eslint/config';

import drizzle from 'eslint-plugin-drizzle';
import javascript from '@eslint/js';
import perfectionist from 'eslint-plugin-perfectionist';
import prettyImport from '@kamiya4047/eslint-plugin-pretty-import';
import stylistic from '@stylistic/eslint-plugin';
import tailwindcss from 'eslint-plugin-better-tailwindcss';
import typescript from 'typescript-eslint';

const compat = new FlatCompat({
  baseDirectory: import.meta.dirname,
});

export default defineConfig(
  ...compat.extends('next/core-web-vitals', 'next/typescript'),
  {
    ignores: ['.next'],
  },
  {
    languageOptions: {
      parser: typescript.parser,
      parserOptions: {
        projectService: true,
        tsconfigRootDir: __dirname,
      },
    },
    linterOptions: {
      reportUnusedDisableDirectives: true,
    },
    name: 'parser',
  },
  javascript.configs.recommended,
  typescript.configs.recommendedTypeChecked,
  typescript.configs.strictTypeChecked,
  stylistic.configs.customize({ arrowParens: true, semi: true }),
  perfectionist.configs['recommended-alphabetical'],
  prettyImport.configs.warn,
  {
    name: 'better-tailwindcss',
    plugins: {
      'better-tailwindcss': tailwindcss,
    },
    rules: {
      ...tailwindcss.configs['recommended-warn'].rules,
    },
    settings: {
      'better-tailwindcss': {
        entryPoint: 'src/styles/globals.css',
      },
    },
  },
  {
    files: ['**/*.ts', '**/*.tsx'],
    plugins: {
      drizzle,
    },
    rules: {
      'drizzle/enforce-delete-with-where': [
        'error',
        { drizzleObjectName: ['db', 'ctx.db'] },
      ],
      'drizzle/enforce-update-with-where': [
        'error',
        { drizzleObjectName: ['db', 'ctx.db'] },
      ],
    },
  },
  {
    name: 'disables',
    rules: {
      '@typescript-eslint/restrict-template-expressions': 'off',
      'perfectionist/sort-imports': 'off',
      'perfectionist/sort-named-imports': 'off',
    },
  },
);
