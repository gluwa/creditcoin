import { Keyring } from "creditcoin-js";
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
