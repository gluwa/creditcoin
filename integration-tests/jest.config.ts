import type { Config } from '@jest/types';
const { pathsToModuleNameMapper } = require('ts-jest');
const { compilerOptions } = require('./tsconfig.json');

const config: Config.InitialOptions = {
    preset: 'ts-jest',
    testEnvironment: 'node',
    testTimeout: 30000,
    globalSetup: './src/globalSetup.ts',
    moduleNameMapper: pathsToModuleNameMapper(compilerOptions.paths),
    modulePaths: ['<rootDir>'],
};

export default config;
