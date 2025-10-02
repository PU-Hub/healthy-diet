import { FlatCompat } from '@eslint/eslintrc';
import { defineConfig } from 'eslint/config';
import { dirname } from 'path';
import { fileURLToPath } from 'url';

import javascript from '@eslint/js';
import perfectionist from 'eslint-plugin-perfectionist';
import prettyImport from '@kamiya4047/eslint-plugin-pretty-import';
import stylistic from '@stylistic/eslint-plugin';
import typescript from 'typescript-eslint';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const compat = new FlatCompat({
  baseDirectory: __dirname,
});

export default defineConfig(
  ...compat.extends('next/core-web-vitals', 'next/typescript'),
  {
    ignores: [
      'node_modules/**',
      '.next/**',
      'out/**',
      'build/**',
      'next-env.d.ts',
    ],
  },
  {
    languageOptions: {
      parser: typescript.parser,
      parserOptions: {
        projectService: true,
        tsconfigRootDir: __dirname,
      },
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
    name: 'disables',
    rules: {
      'perfectionist/sort-imports': 'off',
      'perfectionist/sort-named-imports': 'off',
    },
  },
);
