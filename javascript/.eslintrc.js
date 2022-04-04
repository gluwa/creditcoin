// Copyright 2017-2022 @polkadot/api authors & contributors
// SPDX-License-Identifier: Apache-2.0

module.exports = {
  extends: ['eslint:recommended', 'plugin:@typescript-eslint/eslint-plugin/recommended'],
  ignorePatterns: [
    '**/src/**/*.d.ts',
    '**/integration-tests/**/*.d.ts',
    '.eslintrc.js',
    'babel.config.cjs',
    'jest.config.cjs',
  ],
  plugins: [
    '@typescript-eslint/eslint-plugin',
    'header'
  ],
  parser: '@typescript-eslint/parser',
  parserOptions: {
    project: [
      './tsconfig.eslint.json'
    ],
    sourceType: 'module',
    extraFileExtensions: ['.cjs']
  },
  rules: {
    // add override for any (a metric ton of them, initial conversion)
    '@typescript-eslint/no-explicit-any': 'off',
    // this seems very broken atm, false positives
    '@typescript-eslint/unbound-method': 'off',

    // creditcoin disables
    'no-async-promise-executor': 'off',
    '@typescript-eslint/no-unsafe-assignment': 'off',
    '@typescript-eslint/no-unsafe-call': 'off',
    '@typescript-eslint/no-unsafe-member-access': 'off',
    '@typescript-eslint/no-unsafe-return': 'off',

    'header/header': [2, 'line', [{
        pattern: ' Copyright 20(22|23|24)(-2022)? Gluwa, Inc.'
      },
      ' SPDX-License-Identifier: The Unlicense'
    ], 1],
  }
};
