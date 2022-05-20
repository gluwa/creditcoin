import type { Config } from "@jest/types";
const config: Config.InitialOptions = {
  preset: "ts-jest",
  testEnvironment: "node",
  testTimeout: 30000,
  globalSetup: "./src/globalSetup.ts",
};

export default config;
