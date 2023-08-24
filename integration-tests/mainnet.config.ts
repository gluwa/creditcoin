import type { Config } from "@jest/types";
const config: Config.InitialOptions = {
  preset: "ts-jest",
  testEnvironment: "node",
  testTimeout: 240000,
  globalSetup: "./src/mainnetSetup.ts",
};

export default config;
