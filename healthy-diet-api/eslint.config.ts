import { defineConfig } from 'eslint/config';

import javascript from '@eslint/js';
import prettyImport from '@kamiya4047/eslint-plugin-pretty-import';
import stylistic from '@stylistic/eslint-plugin';
import typescript from 'typescript-eslint';

export default defineConfig(
  {
    name: 'files',
    files: ['**/*.ts'],
  },
  {
    name: 'parser',
    languageOptions: {
      parser: typescript.parser,
      parserOptions: {
        projectService: true,
        tsconfigRootDir: import.meta.dirname,
      },
    },
  },
  javascript.configs.recommended,
  typescript.configs.recommendedTypeChecked,
  typescript.configs.stylisticTypeChecked,
  stylistic.configs.customize({
    arrowParens: true,
    semi: true,
  }),
  prettyImport.configs.warn,
);
