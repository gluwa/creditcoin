import { ApiPromise } from "creditcoin-js";
import { initKeyringPair } from "./account";

export async function bond(
  stashSeed: string,
  controllerAddress: string,
  amount: number,
  rewardDestination: "Staked" | "Stash" | "Controller",
  api: ApiPromise
) {
  const amountInMicroUnits = BigInt(amount) * BigInt(1000000000000000000); // Multiply by to convert to micro units

  const bondTx = api.tx.staking.bond(
    controllerAddress,
    amountInMicroUnits.toString(),
    rewardDestination
  );

  const stashKeyring = initKeyringPair(stashSeed);

  const hash = await bondTx.signAndSend(stashKeyring);

  return hash.toHex();
}

export function parseRewardDestination(rewardDestination: string) {
  if (rewardDestination === "staked") {
    return "Staked";
  } else if (rewardDestination === "stash") {
    return "Stash";
  } else if (rewardDestination === "controller") {
    return "Controller";
  } else {
    console.log(
      "Invalid reward destination, must be one of 'staked', 'stash', or 'controller'"
    );
    process.exit(1);
  }
}
