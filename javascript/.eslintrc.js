// Copyright 2017-2022 @polkadot/api authors & contributors
// SPDX-License-Identifier: Apache-2.0

const base = require('@polkadot/dev/config/eslint.cjs');

module.exports = {
  ...base,
  ignorePatterns: [
    ...base.ignorePatterns,
    '**/src/**/*.d.ts',
    '**/integration-tests/**/*.d.ts',
  ],
  parserOptions: {
    ...base.parserOptions,
    project: [
      './tsconfig.eslint.json'
    ]
  },
  rules: {
    ...base.rules,
    // add override for any (a metric ton of them, initial conversion)
    '@typescript-eslint/no-explicit-any': 'off',
    // this seems very broken atm, false positives
    '@typescript-eslint/unbound-method': 'off',
    'header/header': [2, 'line', [
      { pattern: ' Copyright 20(22|23|24)(-2022)? Gluwa, Inc.' },
      ' SPDX-License-Identifier: The Unlicense'
    ], 1],
  }
};
