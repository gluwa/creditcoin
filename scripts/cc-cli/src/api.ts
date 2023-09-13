import { creditcoinApi } from "creditcoin-js";
import { cryptoWaitReady } from "@polkadot/util-crypto";

// Create new API instance
export async function newApi(url = "ws://localhost:9944") {
  const ccApi = await creditcoinApi(url.trim(), true);
  await cryptoWaitReady();
  return ccApi;
}
