declare module 'eslint-plugin-better-tailwindcss' {
  import { ESLint, Linter } from 'eslint';

  const plugin: {
    configs: {
      'recommended-warn': Linter.Config;
    };
  } & ESLint.Plugin;

  export default plugin;
}
