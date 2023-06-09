import { ApiPromise } from "creditcoin-js";
import { initKeyringPair } from "./account";

export async function bondExtra(
  stashSeed: string,
  amount: number,
  api: ApiPromise
) {
  if (amount < 1) {
    throw new Error("Amount to bond must be at least 1");
  }
  const amountInMicroUnits = BigInt(amount) * BigInt(1000000000000000000); // Multiply by to convert to micro units

  const bondTx = api.tx.staking.bondExtra(amountInMicroUnits.toString());

  const stashKeyring = initKeyringPair(stashSeed);

  const hash = await bondTx.signAndSend(stashKeyring);

  return hash.toHex();
}
