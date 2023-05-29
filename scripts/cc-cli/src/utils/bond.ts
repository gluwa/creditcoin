import { ApiPromise } from "creditcoin-js";
import { initKeyringPair } from "./account";

export async function bond(
  stashSeed: string,
  controllerAddress: string,
  amount: number,
  rewardDestination: "Staked" | "Stash" | "Controller",
  api: ApiPromise
) {
  if (amount < 1) {
    throw new Error("Amount to bond must be at least 1");
  }
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

export function parseRewardDestination(
  rewardDestinationRaw: string
): "Staked" | "Stash" | "Controller" {
  // Capitalize first letter and lowercase the rest
  const rewardDestination =
    rewardDestinationRaw.charAt(0).toUpperCase() +
    rewardDestinationRaw.slice(1).toLowerCase();

  if (rewardDestination != "Staked" || "Stash" || "Controller") {
    throw new Error(
      "Invalid reward destination, must be one of 'Staked', 'Stash', or 'Controller'"
    );
  } else {
    return rewardDestination;
  }
}
