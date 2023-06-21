import { ApiPromise } from "creditcoin-js";
import { initKeyringPair } from "./account";
import { signSendAndWatch } from "./tx";

export interface StakingPalletValidatorPrefs {
  // The validator's commission.
  commission: number;
  // Whether or not the validator is accepting more nominations.
  blocked: boolean;
}

export async function validate(
  seed: string,
  prefs: StakingPalletValidatorPrefs,
  api: ApiPromise
) {
  const stash = initKeyringPair(seed);

  console.log("Creating validate transaction with params:");

  const preferences: StakingPalletValidatorPrefs = prefs
    ? prefs
    : { commission: 0, blocked: false };

  console.log(`Comission: ${preferences.commission}`);
  console.log(`Blocked for new nominators: ${preferences.blocked.toString()}`);

  const validateTx = api.tx.staking.validate(preferences);

  const result = await signSendAndWatch(validateTx, api, stash);

  return result;
}

export async function chill(seed: string, api: ApiPromise) {
  const account = initKeyringPair(seed);

  const chillTx = api.tx.staking.chill();

  const result = await signSendAndWatch(chillTx, api, account);

  return result;
}
