import { ApiPromise, BN, KeyringPair } from "creditcoin-js";
import { MICROUNITS_PER_CTC } from "./balance";
import { requireEnoughFundsToSend, signSendAndWatch } from "./tx";

export async function bond(
  stashKeyring: KeyringPair,
  controllerAddress: string,
  amount: BN,
  rewardDestination: "Staked" | "Stash" | "Controller",
  api: ApiPromise,
  extra = false
) {
  if (amount.lt(new BN(1).mul(MICROUNITS_PER_CTC))) {
    throw new Error("Amount to bond must be at least 1");
  }

  const amountInMicroUnits = amount;

  let bondTx;

  if (extra) {
    bondTx = api.tx.staking.bondExtra(amountInMicroUnits.toString());
  } else {
    bondTx = api.tx.staking.bond(
      controllerAddress,
      amountInMicroUnits.toString(),
      rewardDestination
    );
  }

  await requireEnoughFundsToSend(bondTx, stashKeyring.address, api, amount);

  const result = await signSendAndWatch(bondTx, api, stashKeyring);

  return result;
}

export function parseRewardDestination(
  rewardDestinationRaw: string
): "Staked" | "Stash" | "Controller" {
  // Capitalize first letter and lowercase the rest
  const rewardDestination =
    rewardDestinationRaw.charAt(0).toUpperCase() +
    rewardDestinationRaw.slice(1).toLowerCase();

  if (
    rewardDestination !== "Staked" &&
    rewardDestination !== "Stash" &&
    rewardDestination !== "Controller"
  ) {
    throw new Error(
      "Invalid reward destination, must be one of 'Staked', 'Stash', or 'Controller'"
    );
  } else {
    return rewardDestination;
  }
}

export function checkRewardDestination(
  rewardDestinationRaw: string
): "Staked" | "Stash" | "Controller" {
  // Capitalize first letter and lowercase the rest
  const rewardDestination =
    rewardDestinationRaw.charAt(0).toUpperCase() +
    rewardDestinationRaw.slice(1).toLowerCase();

  if (
    rewardDestination !== "Staked" &&
    rewardDestination !== "Stash" &&
    rewardDestination !== "Controller"
  ) {
    throw new Error(
      "Invalid reward destination, must be one of 'Staked', 'Stash', or 'Controller'"
    );
  } else {
    return rewardDestination;
  }
}
