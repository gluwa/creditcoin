import { ApiPromise } from "creditcoin-js";
import { initKeyringPair } from "./account";

export async function bond(
    stashSeed: string,
    controllerAddress: string,
    amount: number,
    rewardDestination: "Staked" | "Stash" | "Controller",
    api: ApiPromise) {

        const bondTx = api.tx.staking.bond(
            controllerAddress,
            `${amount}000000000000000000`, // Add 18 zeros to convert to micro units TODO: Improve amount handling
            rewardDestination);

        const stashKeyring = initKeyringPair(stashSeed);

        const hash = await bondTx.signAndSend(stashKeyring);

        return hash.toHex();
}

export function parseRewardDestination(rewardDestination: string) {
    if (rewardDestination === 'staked') {
        return 'Staked';
    } else if (rewardDestination === 'stash') {
        return 'Stash';
    } else if (rewardDestination === 'controller') {
        return 'Controller';
    } else {
        console.log("Invalid reward destination, must be one of 'staked', 'stash', or 'controller'");
        process.exit(1);
    }
}
