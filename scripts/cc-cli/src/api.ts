import { ApiPromise, WsProvider } from "creditcoin-js";
import { cryptoWaitReady } from "@polkadot/util-crypto";

// Create new API instance
export async function newApi(url: string = "ws://localhost:9944") {
  const api = await ApiPromise.create({
    provider: new WsProvider(url),
  });
  await cryptoWaitReady();
  return api;
}
