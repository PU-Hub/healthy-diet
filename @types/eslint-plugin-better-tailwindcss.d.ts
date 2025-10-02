declare module 'eslint-plugin-better-tailwindcss' {
  import { ESLint, Linter } from 'eslint';

  const plugin: ESLint.Plugin & {
    configs: {
      'recommended-warn': Linter.Config;
    };
  };

  export default plugin;
}
