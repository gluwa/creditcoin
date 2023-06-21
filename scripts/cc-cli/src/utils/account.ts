import { mnemonicValidate } from "@polkadot/util-crypto";
import { ApiPromise, Keyring } from "creditcoin-js";
import prompts from "prompts";

export function initKeyringPair(seed: string) {
  const keyring = new Keyring({ type: "sr25519" });
  const pair = keyring.addFromUri(`${seed}`);
  return pair;
}

export async function getStashSeedFromEnvOrPrompt() {
  return await getSeedFromEnvOrPrompt(
    process.env.CC_STASH_SEED,
    "Specify a seed phrase for the Stash account"
  );
}
export async function getControllerSeedFromEnvOrPrompt() {
  return await getSeedFromEnvOrPrompt(
    process.env.CC_CONTROLLER_SEED,
    "Specify a seed phrase for the Controller account"
  );
}

export async function getSeedFromEnvOrPrompt(envVar?: string | undefined, promptStr?: string | null) {
  if (envVar && mnemonicValidate(envVar)) {
    return envVar;
  }
  let seedPromptResult = await prompts([
    {
      type: "invisible",
      name: "seed",
      message: promptStr ? promptStr : "Enter seed phrase",
      validate: seed => mnemonicValidate(seed)
    }
  ]);
  return seedPromptResult.seed;
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
