import { mnemonicValidate } from "@polkadot/util-crypto";
import { Keyring, KeyringPair } from "creditcoin-js";
import prompts from "prompts";
import { getErrorMessage } from "./error";
import { OptionValues } from "commander";

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

export async function initStashKeyring(
  options: OptionValues,
): Promise<KeyringPair> {
  try {
    return await initKeyringFromEnvOrPrompt(
      "CC_STASH_SECRET",
      "stash",
      options,
    );
  } catch (e) {
    console.error(getErrorMessage(e));
    process.exit(1);
  }
}

export async function initControllerKeyring(
  options: OptionValues,
): Promise<KeyringPair> {
  try {
    return await initKeyringFromEnvOrPrompt(
      "CC_CONTROLLER_SECRET",
      "controller",
      options,
    );
  } catch (e) {
    console.error(getErrorMessage(e));
    process.exit(1);
  }
}

export async function initCallerKeyring(
  options: OptionValues,
): Promise<KeyringPair> {
  try {
    return await initKeyringFromEnvOrPrompt("CC_SECRET", "caller", options);
  } catch (e) {
    console.error(getErrorMessage(e));
    process.exit(1);
  }
}

export async function initKeyringFromEnvOrPrompt(
  envVar: string,
  accountRole: string,
  options: OptionValues,
): Promise<KeyringPair> {
  const interactive = options.input;
  const ecdsa = options.ecdsa;
  const inputName = ecdsa ? "private key" : "seed phrase";
  const validateInput = ecdsa ? validateECDSAKey : mnemonicValidate;
  const generateKeyring = ecdsa ? initECDSAKeyringPairFromPK : initKeyringPair;

  if (!interactive && !process.env[envVar]) {
    throw new Error(
      `Error: Must specify a ${inputName} for the ${accountRole} account in the environment variable ${envVar} or use an interactive shell.`,
    );
  }

  if (typeof process.env[envVar] === "string") {
    if (validateInput(process.env[envVar]!)) {
      return generateKeyring(process.env[envVar]!);
    } else {
      throw new Error(
        `Error: Seed phrase provided in environment variable ${envVar} is invalid.`,
      );
    }
  } else if (interactive) {
    const promptResult = await prompts([
      {
        type: "password",
        name: "seed",
        message: `Specify a ${inputName} for the ${accountRole} account`,
        validate: (input) => validateInput(input),
      },
    ]);
    // If SIGTERM is issued while prompting, it will log a bogus address anyways and exit without error.
    // To avoid this, we check if prompt was successful, before returning.
    if (promptResult.seed) {
      return generateKeyring(promptResult.seed);
    }
  }
  throw new Error(`Error: Could not retrieve ${inputName}`);
}

function validateECDSAKey(pk: string): boolean {
  const keyring = initECDSAKeyringPairFromPK(pk);
  const msg = "";
  const sig = keyring.sign(msg);
  return keyring.verify(msg, sig, keyring.publicKey);
}
