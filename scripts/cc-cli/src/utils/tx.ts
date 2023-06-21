import { ISubmittableResult } from "@polkadot/types/types";
import { ApiPromise } from "creditcoin-js";

export function handleTxStatusAndExit(
  result: ISubmittableResult,
  api: ApiPromise
): void {
  const { status, dispatchError } = result;

  if (status.isFinalized) {
    console.log(
      `Transaction succeeded and included at blockHash ${status.asFinalized.toString()}`
    );
    process.exit(0);
  }

  if (dispatchError) {
    if (dispatchError.isModule) {
      // for module errors, the section is indexed, lookup
      const decoded = api.registry.findMetaError(dispatchError.asModule);
      const { docs, name, section } = decoded;

      const error = `${section}.${name}: ${docs.join(" ")}`;

      console.log(`Transaction failed with error: "${error}"`);
      process.exit(1);
    } else {
      // Other, CannotLookup, BadOrigin, no extra info
      console.log(
        `Transaction failed with error: "${dispatchError.toString()}"`
      );
      process.exit(1);
    }
  }
}
