import { mnemonicValidate } from "@polkadot/util-crypto";
import { Keyring } from "creditcoin-js";
import prompts from "prompts";

export function initKeyringPair(seed: string) {
  const keyring = new Keyring({ type: "sr25519" });
  const pair = keyring.addFromUri(`${seed}`);
  return pair;
}
export function initECDSAKeyringPairFromPK(pk: string) {
  const keyring = new Keyring({ type: "ecdsa" });
  const pair = keyring.addFromUri(`${pk}`);
  return pair;
}

export async function getStashSeedFromEnvOrPrompt(interactive: boolean) {
  if (!interactive && !process.env.CC_STASH_SEED) {
    console.error(
      "Error: Must specify a seed phrase for the Stash account in the environment variable CC_STASH_SEED or use an interactive shell."
    );
    process.exit(1);
  }
  return await getSeedFromEnvOrPrompt(
    process.env.CC_STASH_SEED,
    "Specify a seed phrase for the Stash account"
  );
}
export async function getControllerSeedFromEnvOrPrompt(interactive: boolean) {
  if (!interactive && !process.env.CC_CONTROLLER_SEED) {
    console.error(
      "Error: Must specify a seed phrase for the Controller account in the environment variable CC_CONTROLLER_SEED or use an interactive shell."
    );
    process.exit(1);
  }
  return await getSeedFromEnvOrPrompt(
    process.env.CC_CONTROLLER_SEED,
    "Specify a seed phrase for the Controller account"
  );
}
export async function getCallerSeedFromEnvOrPrompt(interactive: boolean) {
  if (!interactive && !process.env.CC_SEED) {
    console.error(
      "Error: Must specify a seed phrase for the Caller account in the environment variable CC_SEED or use an interactive shell."
    );
    process.exit(1);
  }
  return await getSeedFromEnvOrPrompt(
    process.env.CC_SEED,
    "Specify caller's seed phrase"
  );
}

async function getSeedFromEnvOrPrompt(
  envVar?: string | undefined,
  promptStr?: string | null
) {
  if (envVar) {
    if (mnemonicValidate(envVar)) {
      return envVar;
    } else {
      console.log(
        "Error: Seed phrase provided in environment variable is invalid."
      );
      process.exit(1);
    }
  }
  const seedPromptResult = await prompts([
    {
      type: "password",
      name: "seed",
      message: promptStr ? promptStr : "Enter seed phrase",
      validate: (seed) => mnemonicValidate(seed),
    },
  ]);

  // If SIGTERM is issued while prompting, it will log a bogus address anyways and exit without error.
  // To avoid this, we check if prompt was successful, before returning.
  if (seedPromptResult.seed) {
    return seedPromptResult.seed;
  }
  console.log("Error: Could not retrieve seed phrase.");
  process.exit(1);
}
