import { ApiPromise, Keyring } from "creditcoin-js";
import { OptionValues } from "commander";
import { readFileSync } from "fs";

export function initKeyringPair(seed: string) {
  const keyring = new Keyring({ type: "sr25519" });
  const pair = keyring.addFromUri(`${seed}`);
  return pair;
}

export function getSeedFromOptions(options: OptionValues) {
  if (options.seed) {
    return options.seed;
  } else if (options.file) {
    return readFileSync(options.file).toString();
  } else {
    throw new Error("Must specify either mnemonic phrase or file as an option");
  }
}

export function checkAddress(address: string, api: ApiPromise) {
  if (!address) {
    console.log("Must specify address to get balance of");
    process.exit(1);
  } else {
    checkIfAddressIsValid(address, api);
  }
}

function checkIfAddressIsValid(address: string, api: ApiPromise) {
  try {
    api.createType("Address", address);
  } catch (e) {
    console.log("Invalid controller address");
    process.exit(1);
  }
}
