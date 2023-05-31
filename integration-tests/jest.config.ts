import type { Config } from "@jest/types";
const config: Config.InitialOptions = {
  preset: "ts-jest",
  testEnvironment: "node",
  testTimeout: 45000,
  globalSetup: "./src/globalSetup.ts",
};

export default config;
