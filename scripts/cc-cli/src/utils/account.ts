import { mnemonicValidate } from "@polkadot/util-crypto";
import { Keyring } from "creditcoin-js";
import prompts from "prompts";
import { getErrorMessage } from "./error";

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
  try {
    return await getSeedFromEnvOrPrompt("CC_STASH_SEED", "stash", interactive);
  } catch (e) {
    console.error(getErrorMessage(e));
    process.exit(1);
  }
}
export async function getControllerSeedFromEnvOrPrompt(interactive: boolean) {
  try {
    return await getSeedFromEnvOrPrompt(
      "CC_CONTROLLER_SEED",
      "controller",
      interactive,
    );
  } catch (e) {
    console.error(getErrorMessage(e));
    process.exit(1);
  }
}
export async function getCallerSeedFromEnvOrPrompt(interactive: boolean) {
  try {
    return await getSeedFromEnvOrPrompt("CC_SEED", "caller", interactive);
  } catch (e) {
    console.error(getErrorMessage(e));
    process.exit(1);
  }
}

async function getSeedFromEnvOrPrompt(
  envVar = "CC_SEED",
  accountRole = "caller",
  interactive = true,
) {
  if (!interactive && !process.env[envVar]) {
    throw new Error(
      `Error: Must specify a seed phrase for the ${accountRole} account in the environment variable ${envVar} or use an interactive shell.`,
    );
  }

  if (typeof process.env[envVar] === "string") {
    const seedFromEnv = process.env[envVar];
    if (mnemonicValidate(seedFromEnv!)) {
      return seedFromEnv;
    } else {
      throw new Error(
        `Error: Seed phrase provided in environment variable ${envVar} is invalid.`,
      );
    }
  } else if (interactive) {
    const seedPromptResult = await prompts([
      {
        type: "password",
        name: "seed",
        message: `Specify a seed phrase for the ${accountRole} account`,
        validate: (seed) => mnemonicValidate(seed),
      },
    ]);
    // If SIGTERM is issued while prompting, it will log a bogus address anyways and exit without error.
    // To avoid this, we check if prompt was successful, before returning.
    if (seedPromptResult.seed) {
      return seedPromptResult.seed;
    }
  }
  throw new Error("Error: Could not retrieve seed phrase.");
}
