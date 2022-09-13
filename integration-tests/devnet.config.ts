import type { Config } from "@jest/types";
const config: Config.InitialOptions = {
  preset: "ts-jest",
  testEnvironment: "node",
  testTimeout: 240000,
  globalSetup: "./src/devnetSetup.ts",
};

export default config;
