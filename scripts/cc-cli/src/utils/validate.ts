import { ApiPromise } from "creditcoin-js";
import { initKeyringPair } from "./account";

export interface StakingPalletValidatorPrefs {
    // The validator's commission.
    commission: number,
    // Whether or not the validator is accepting more nominations.
    blocked: boolean,
}

export async function validate(seed: string, prefs: StakingPalletValidatorPrefs, api: ApiPromise) {
    const stash = initKeyringPair(seed);

    console.log("Creating validate transaction with params:");

    const preferences: StakingPalletValidatorPrefs = prefs ? prefs : { commission: 0, blocked: false };

    console.log("Comission: " + preferences.commission);
    console.log("Blocked for new nominators: " + preferences.blocked);

    const validate = api.tx.staking.validate(preferences);

    const hash = await validate.signAndSend(stash);

    console.log("Validate transaction sent with hash:", hash.toHex());
    return hash;
}
