import type { Config } from "@jest/types";
const config: Config.InitialOptions = {
  preset: "ts-jest",
  testEnvironment: "node",
  testTimeout: 30000,
  globals: {
    CREDITCOIN_API_URL: "ws://127.0.0.1:9944",
    CREDITCOIN_METRICS_BASE: "http://127.0.0.1:9615",
  },
  globalSetup: "./src/globalSetup.ts",
};

export default config;
